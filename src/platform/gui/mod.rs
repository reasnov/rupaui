pub mod window;

use std::sync::OnceLock;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent, KeyEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy, EventLoop};
use winit::window::WindowId;
use winit::keyboard::{PhysicalKey, KeyCode as WinitKeyCode};

use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::layout::engine::LayoutEngine;
use crate::utils::vector::Vec2;
use crate::platform::gui::window::Window;
use crate::platform::dispatcher::EventDispatcher;
use crate::platform::{RupauiEvent, events::*};

static EVENT_PROXY: OnceLock<EventLoopProxy<RupauiEvent>> = OnceLock::new();

pub fn get_event_proxy() -> Option<&'static EventLoopProxy<RupauiEvent>> {
    EVENT_PROXY.get()
}

pub fn set_event_proxy(proxy: EventLoopProxy<RupauiEvent>) {
    let _ = EVENT_PROXY.set(proxy);
}

pub struct RupauiRunner {
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,
    pub layout_engine: LayoutEngine,

    pub app_name: String,
    pub root: Option<Box<dyn Component>>,

    pub cursor_pos: Vec2,
    pub root_node: Option<taffy::prelude::NodeId>,
    pub scale_factor: f64,
}

impl RupauiRunner {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            window: None,
            renderer: None,
            layout_engine: LayoutEngine::new(),
            app_name,
            root,
            cursor_pos: Vec2::zero(),
            root_node: None,
            scale_factor: 1.0,
        }
    }

    pub fn run_app(mut self) {
        let event_loop = EventLoop::<RupauiEvent>::with_user_event().build().unwrap();
        set_event_proxy(event_loop.create_proxy());
        event_loop.run_app(&mut self).unwrap();
    }

    fn handle_redraw(&mut self) {
        let root = match &self.root {
            Some(r) => r,
            None => return,
        };

        let (win_width, win_height) = self.window.as_ref().unwrap().size();
        let root_node = self.layout_engine.compute(root.as_ref(), win_width as f32, win_height as f32);
        self.root_node = Some(root_node);

        let renderer = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        if let Ok((output, view, mut encoder)) = renderer.begin_frame() {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Rupaui UI Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None, depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None, multiview_mask: None,
                });

                root.paint(
                    renderer,
                    &self.layout_engine.taffy,
                    root_node,
                    false, 
                    &mut pass,
                    Vec2::zero(),
                );

                renderer.flush_batches(&mut pass);
            }
            renderer.end_frame(output, encoder);
        }
    }

    fn dispatch_event(&mut self, event: RawInputEvent) {
        if let (Some(root), Some(node)) = (&self.root, self.root_node) {
            EventDispatcher::dispatch(
                event,
                root.as_ref(),
                &self.layout_engine.taffy,
                node,
                &mut self.cursor_pos,
            );
        }
    }
}

impl ApplicationHandler<RupauiEvent> for RupauiRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Window::new(event_loop, &self.app_name, 1024, 768).unwrap();
            let renderer = pollster::block_on(Renderer::new(window.raw()));
            self.scale_factor = window.raw().scale_factor();
            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, event: RupauiEvent) {
        let RupauiEvent::RequestRedraw = event;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.dispatch_event(RawInputEvent::Quit);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                self.dispatch_event(RawInputEvent::Resize { 
                    size: Vec2::new(size.width as f32, size.height as f32),
                    scale_factor: self.scale_factor 
                });
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let logical_pos = Vec2::new(position.x as f32, position.y as f32);
                self.dispatch_event(RawInputEvent::PointerMove { position: logical_pos });
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Auxiliary,
                    MouseButton::Other(id) => PointerButton::Extra(id),
                };
                let st = match state {
                    ElementState::Pressed => ButtonState::Pressed,
                    ElementState::Released => ButtonState::Released,
                };
                self.dispatch_event(RawInputEvent::PointerButton { button: btn, state: st });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vec2::new(x, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => Vec2::new(pos.x as f32, pos.y as f32),
                };
                self.dispatch_event(RawInputEvent::PointerScroll { delta: scroll });
            }
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                if let PhysicalKey::Code(WinitKeyCode::KeyQ) = physical_key {
                    event_loop.exit();
                }
                // Mapping other keys to standardized KeyCode...
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
