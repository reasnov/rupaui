pub mod events;
pub mod dispatcher;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

use crate::core::component::Component;
use crate::core::plugin::PluginRegistry;
use crate::utils::Theme;
use std::error::Error;

/// The standard interface for any platform backend (GUI, TUI, Headless).
pub trait PlatformRunner {
    /// Initialize the backend (Open window, raw mode, etc).
    fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Enter the main execution loop.
    fn run(self) -> Result<(), Box<dyn Error>>;
    
    /// Request a UI redraw from the platform.
    fn request_redraw(&self);
}

#[derive(Debug)]
pub enum RupauiEvent {
    RequestRedraw,
}

/// Global hook to request a redraw. Implementation is platform-dependent.
pub fn request_redraw() {
    #[cfg(feature = "gui")]
    if let Some(proxy) = gui::get_event_proxy() {
        let _ = proxy.send_event(RupauiEvent::RequestRedraw);
    }
    
    #[cfg(feature = "tui")]
    {
        // TUI redraw logic handled via signal or polling
    }
}

pub struct App {
    pub name: String,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            registry: PluginRegistry::new(),
        }
    }

    pub fn root(mut self, component: impl Component + 'static) -> Self {
        self.root = Some(Box::new(component));
        self
    }

    fn bootstrap(&mut self) {
        let _ = Theme::current();
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(self);
    }

    #[cfg(feature = "gui")]
    pub fn run(mut self) {
        self.bootstrap();
        let runner = gui::RupauiRunner::new(self.name.clone(), self.root);
        // gui runner uses winit's specific run_app which consumes the event loop
        runner.run_app();
    }

    #[cfg(feature = "tui")]
    pub fn run_tui(mut self) {
        self.bootstrap();
        let mut runner = tui::TuiRunner::new(self.name.clone(), self.root);
        if let Err(e) = runner.run() {
            eprintln!("TUI Error: {}", e);
        }
    }
}
