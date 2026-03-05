use std::sync::Arc;

/// Standard events supported by Rupaui components.
pub enum Event {
    Click,
    Input(String),
    Focus,
    Blur,
    MouseEnter,
    MouseLeave,
    KeyDown(String),
}

/// A container for component event listeners.
/// Uses Arc to allow safe sharing across threads/wasm tasks.
#[derive(Clone, Default)]
pub struct EventListeners {
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_input: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl std::fmt::Debug for EventListeners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListeners").finish()
    }
}

impl PartialEq for EventListeners {
    fn eq(&self, _other: &Self) -> bool { true } // Listeners are functional, comparison is skipped
}
