use crate::core::component::Component;
use crate::support::{vector::Vec2, state::Signal};
use crate::platform::events::{InputEvent, PointerButton, ButtonState, Modifiers, CursorIcon};
use crate::scene::{SceneCore, HitDiscovery};
use std::sync::Arc;

/// The UIEvent passed to components during dispatch.
/// Contains rich context about the user interaction.
pub struct UIEvent {
    pub consumed: bool,
    pub local_pos: Vec2,
    pub modifiers: Modifiers,
    pub button: Option<PointerButton>,
    pub button_state: Option<ButtonState>,
    pub capture_request: Option<bool>,
    pub focus_request: Option<bool>, // Some(true) to request focus, Some(false) to blur
}

impl UIEvent {
    pub fn new(local_pos: Vec2) -> Self {
        Self { 
            consumed: false, 
            local_pos,
            modifiers: Modifiers::default(),
            button: None,
            button_state: None,
            capture_request: None,
            focus_request: None,
        }
    }

    pub fn with_context(mut self, modifiers: Modifiers, button: Option<PointerButton>, state: Option<ButtonState>) -> Self {
        self.modifiers = modifiers;
        self.button = button;
        self.button_state = state;
        self
    }

    pub fn consume(&mut self) { self.consumed = true; }
    pub fn capture_pointer(&mut self) { self.capture_request = Some(true); }
    pub fn release_pointer(&mut self) { self.capture_request = Some(false); }
    pub fn request_focus(&mut self) { self.focus_request = Some(true); }
    pub fn blur(&mut self) { self.focus_request = Some(false); }
}

pub struct InputDispatcher;

impl InputDispatcher {
    /// The central entry point for dispatching standardized input events into the component tree.
    pub fn dispatch(
        event: InputEvent,
        root: &dyn Component,
        scene: &SceneCore,
        viewport: &Signal<Vec2>,
        cursor_pos: &mut Vec2,
        requested_cursor: &mut CursorIcon,
        pointer_capture: &mut Option<String>,
        focused_id: &mut Option<String>,
        event_listeners: &[Arc<dyn Fn(&InputEvent) + Send + Sync>],
        debug: bool,
    ) {
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }
        
        // Trigger all registered plugin hooks first
        for listener in event_listeners {
            listener(&event);
        }

        // --- Focus Trap Detection ---
        let mut target_root = root;
        if let Some(body) = root.as_any().downcast_ref::<crate::core::body::Body>() {
            if let Ok(stack) = body.logic.overlays.read() {
                // Find the topmost modal overlay
                if let Some(modal) = stack.iter().rev().find(|o| o.is_modal()) {
                    target_root = modal.as_ref();
                }
            }
        }

        match event {
            InputEvent::PointerMove { position } => {
                let delta = position - *cursor_pos;
                *cursor_pos = position;

                // Handle Pointer Capture (Capture overrides Focus Trap)
                if let Some(capture_id) = pointer_capture.as_ref() {
                    if let Some(hit) = scene.find_by_id(root, capture_id) {
                        let local_pos = position - hit.local_pos;
                        let mut ui_ev = UIEvent::new(local_pos);
                        hit.component.on_drag(&mut ui_ev, delta);
                        if let Some(false) = ui_ev.capture_request {
                             *pointer_capture = None;
                        }
                        return;
                    }
                }

                // Normal hit-testing
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    // Resolve Cursor from topmost hovered component
                    if let Some(target) = hit.path.last() {
                        if let Some(body) = target.as_any().downcast_ref::<crate::core::body::Body>() {
                             *requested_cursor = body.view.core.style.read().unwrap().interactivity.cursor;
                        } else if let Some(div) = target.as_any().downcast_ref::<crate::primitives::div::Div>() {
                             *requested_cursor = div.view.core.style.read().unwrap().interactivity.cursor;
                        }
                    }

                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_drag(&mut ui_ev, delta); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::PointerButton { button, state } => {
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(Modifiers::default(), Some(button), Some(state));
                    
                    for comp in hit.path.iter().rev() {
                        match state {
                            ButtonState::Pressed => comp.on_click(&mut ui_ev),
                            ButtonState::Released => comp.on_release(&mut ui_ev),
                        }
                        
                        // Handle Capture Requests
                        if let Some(true) = ui_ev.capture_request {
                            *pointer_capture = Some(comp.id().to_string());
                        } else if let Some(false) = ui_ev.capture_request {
                            *pointer_capture = None;
                        }

                        // Handle Focus Requests
                        if let Some(true) = ui_ev.focus_request {
                            *focused_id = Some(comp.id().to_string());
                        } else if let Some(false) = ui_ev.focus_request {
                            *focused_id = None;
                        }

                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::PointerScroll { delta } => {
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Key { key, state, modifiers } => {
                // Priority 1: Component with Focus (must be within target_root if trap is active)
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero())
                            .with_context(modifiers, None, Some(state));
                        
                        for comp in hit.path.iter().rev() {
                            comp.on_key(&mut ui_ev, key);
                            if ui_ev.consumed { break; }
                        }
                        return;
                    }
                }

                // Priority 2: Hovered Component within target_root
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(modifiers, None, Some(state));
                    
                    for comp in hit.path.iter().rev() {
                        comp.on_key(&mut ui_ev, key); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Ime(text) => {
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero());
                        for comp in hit.path.iter().rev() {
                            comp.on_text(&mut ui_ev, &text);
                            if ui_ev.consumed { break; }
                        }
                    }
                }
            }
            InputEvent::Resize { size, .. } => {
                viewport.set(size);
                fn broadcast_resize(comp: &dyn Component, size: Vec2) {
                    let mut ui_ev = UIEvent::new(Vec2::zero());
                    comp.on_resize(&mut ui_ev, size);
                    for child in comp.children() {
                        broadcast_resize(child, size);
                    }
                }
                broadcast_resize(root, size);
            }
            InputEvent::SafeArea { top, right, bottom, left } => {
                fn broadcast_safe_area(comp: &dyn Component, t: f32, r: f32, b: f32, l: f32) {
                    let mut ui_ev = UIEvent::new(Vec2::zero());
                    comp.on_safe_area(&mut ui_ev, t, r, b, l);
                    for child in comp.children() {
                        broadcast_safe_area(child, t, r, b, l);
                    }
                }
                broadcast_safe_area(root, top, right, bottom, left);
            }
            InputEvent::Quit => {}
            _ => {}
        }
    }
}
