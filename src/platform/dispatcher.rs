use taffy::prelude::*;
use crate::core::component::Component;
use crate::utils::vector::Vec2;
use crate::platform::events::{RawInputEvent, PointerButton, ButtonState};

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

pub struct HitTestResult<'a> {
    pub path: Vec<&'a dyn Component>,
    pub local_pos: Vec2,
}

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn hit_test<'a>(
        taffy: &TaffyTree<()>,
        root: &'a dyn Component,
        root_node: NodeId,
        cursor_pos: Vec2,
        parent_global_pos: Vec2,
    ) -> Option<HitTestResult<'a>> {
        let layout = taffy.layout(root_node).ok()?;
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        let is_inside = cursor_pos.x >= global_pos.x 
            && cursor_pos.x <= global_pos.x + layout.size.width
            && cursor_pos.y >= global_pos.y 
            && cursor_pos.y <= global_pos.y + layout.size.height;

        if !is_inside { return None; }

        let children = root.children();
        let taffy_children = taffy.children(root_node).ok()?;

        for (i, child) in children.iter().enumerate().rev() {
            if let Some(child_node) = taffy_children.get(i) {
                if let Some(mut result) = Self::hit_test(taffy, *child, *child_node, cursor_pos, global_pos) {
                    result.path.push(root);
                    return Some(result);
                }
            }
        }

        Some(HitTestResult {
            path: vec![root],
            local_pos: cursor_pos - global_pos,
        })
    }

    /// The central entry point for dispatching raw events into the component tree.
    pub fn dispatch(
        event: RawInputEvent,
        root: &dyn Component,
        taffy: &TaffyTree<()>,
        root_node: NodeId,
        cursor_pos: &mut Vec2,
    ) {
        match event {
            RawInputEvent::PointerMove { position } => {
                *cursor_pos = position;
                if let Some(hit) = Self::hit_test(taffy, root, root_node, *cursor_pos, Vec2::zero()) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    // Currently drag is simplified to move
                    for comp in hit.path {
                        comp.on_drag(&mut ui_ev, Vec2::zero()); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            RawInputEvent::PointerButton { button, state } => {
                if button == PointerButton::Primary && state == ButtonState::Pressed {
                    if let Some(hit) = Self::hit_test(taffy, root, root_node, *cursor_pos, Vec2::zero()) {
                        let mut ui_ev = UIEvent::new(hit.local_pos);
                        for comp in hit.path {
                            comp.on_click(&mut ui_ev);
                            if ui_ev.consumed { break; }
                        }
                    }
                }
            }
            RawInputEvent::PointerScroll { delta } => {
                if let Some(hit) = Self::hit_test(taffy, root, root_node, *cursor_pos, Vec2::zero()) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }
            _ => {
                // Keyboard and other events would go to Focus Manager (Layer 5)
            }
        }
    }
}
