use crate::plugin::{Plugin, PluginRegistry};
use crate::utils::Theme;
use crate::window::Window;
use crate::Component;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::WindowId;

/// Internal state manager for the winit event loop.
struct RupauiRunner {
    pub window: Option<Window>,
    pub app_name: String,
    pub root: Option<Box<dyn Component>>,
}

impl ApplicationHandler for RupauiRunner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            log::info!("Resumed: Creating window for {}", self.app_name);
            let window = Window::new(event_loop, &self.app_name, 1024, 768)
                .expect("Failed to create Rupaui window");
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close requested. Exiting application.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(ref root) = self.root {
                    root.render();
                }
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

/// The primary entry point for a Rupaui application.
/// Manages the application lifecycle and plugin registry.
pub struct App {
    pub name: String,
    pub registry: PluginRegistry,
    pub root: Option<Box<dyn Component>>,
}

impl App {
    /// Creates a new application instance.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            registry: PluginRegistry::new(),
            root: None,
        }
    }

    /// Sets the root component of the application's UI tree.
    pub fn root(mut self, component: impl Component + 'static) -> Self {
        self.root = Some(Box::new(component));
        self
    }

    /// Adds a plugin to the application.
    pub fn add_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.registry.add(Box::new(plugin));
        self
    }

    /// Starts the application, building all plugins and initializing the event loop.
    pub fn run(mut self) {
        log::info!("Starting Rupaui Application: {}", self.name);
        
        // 1. Initialize DNA Visual from Default Artisan Theme
        let _ = Theme::current();

        // 2. Build all registered plugins safely
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(&mut self);
        self.registry = registry;

        // 3. Initialize the OS Event Loop
        let event_loop = EventLoop::new().expect("Failed to initialize Winit EventLoop");
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut runner = RupauiRunner {
            window: None,
            app_name: self.name.clone(),
            root: self.root,
        };

        log::debug!("Handing over to OS Application Loop...");
        let _ = event_loop.run_app(&mut runner);
    }
}
