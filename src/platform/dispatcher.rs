use crate::core::component::Component;
use crate::utils::vector::Vec2;
use crate::platform::events::{InputEvent, PointerButton, ButtonState};
use crate::scene::{SceneCore, HitDiscovery};

/// The UIEvent passed to components during dispatch.
pub struct UIEvent {
    pub consumed: bool,
    pub local_pos: Vec2,
}

impl UIEvent {
    pub fn new(local_pos: Vec2) -> Self {
        Self { consumed: false, local_pos }
    }
    pub fn consume(&mut self) { self.consumed = true; }
}

pub struct InputDispatcher;

impl InputDispatcher {
    /// The central entry point for dispatching standardized input events into the component tree.
    pub fn dispatch(
        event: InputEvent,
        root: &dyn Component,
        scene: &SceneCore,
        cursor_pos: &mut Vec2,
    ) {
        match event {
            InputEvent::PointerMove { position } => {
                *cursor_pos = position;
                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path {
                        comp.on_drag(&mut ui_ev, Vec2::zero()); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::PointerButton { button, state } => {
                if button == PointerButton::Primary && state == ButtonState::Pressed {
                    if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                        let mut ui_ev = UIEvent::new(hit.local_pos);
                        for comp in hit.path {
                            comp.on_click(&mut ui_ev);
                            if ui_ev.consumed { break; }
                        }
                    }
                }
            }
            InputEvent::PointerScroll { delta } => {
                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }
            _ => {}
        }
    }
}
