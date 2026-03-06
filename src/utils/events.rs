use std::sync::Arc;
use crate::utils::vector::Vec2;

pub type ClickCallback = Arc<dyn Fn() + Send + Sync>;
pub type ChangeCallback<T> = Arc<dyn Fn(T) + Send + Sync>;
pub type HoverCallback = Arc<dyn Fn(bool) + Send + Sync>;
pub type ScrollCallback = Arc<dyn Fn(f32) + Send + Sync>; // Delta Y
pub type DragCallback = Arc<dyn Fn(Vec2) + Send + Sync>; // Delta position

#[derive(Clone, Default)]
pub struct EventListeners {
    pub on_click: Option<ClickCallback>,
    pub on_change: Option<ChangeCallback<String>>,
    pub on_hover: Option<HoverCallback>,
    pub on_scroll: Option<ScrollCallback>,
    pub on_drag: Option<DragCallback>,
}

impl std::fmt::Debug for EventListeners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListeners").finish()
    }
}

impl PartialEq for EventListeners {
    fn eq(&self, _other: &Self) -> bool { true }
}
