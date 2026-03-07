pub mod events;
pub mod dispatcher;
pub mod a11y;
pub mod app;
pub mod context;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

pub use self::a11y::{SemanticRole, AccessibilityNode};
pub use self::app::{App, AppMetadata};
pub use self::context::{PlatformCore, SharedPlatformCore, request_redraw, register_redraw_proxy};

#[derive(Debug)]
pub enum PlatformEvent {
    RequestRedraw,
}
