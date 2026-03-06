use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode as CrossKeyCode, MouseEventKind, MouseButton as CrossMouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size},
};
use crate::core::component::Component;
use crate::layout::engine::LayoutEngine;
use crate::utils::vector::Vec2;
use crate::platform::dispatcher::EventDispatcher;
use crate::platform::events::*;

pub struct TuiRunner {
    pub app_name: String,
    pub root: Option<Box<dyn Component>>,
    pub layout_engine: LayoutEngine,
    pub should_quit: bool,
    pub cursor_pos: Vec2,
}

impl TuiRunner {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            app_name,
            root,
            layout_engine: LayoutEngine::new(),
            should_quit: false,
            cursor_pos: Vec2::zero(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;

        let (cols, rows) = size()?;
        let mut current_size = Vec2::new(cols as f32, rows as f32);

        while !self.should_quit {
            self.handle_redraw(current_size);

            if event::poll(Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(key) => {
                        let code = match key.code {
                            CrossKeyCode::Char(c) => KeyCode::Char(c),
                            CrossKeyCode::Enter => KeyCode::Enter,
                            CrossKeyCode::Esc => KeyCode::Escape,
                            CrossKeyCode::Tab => KeyCode::Tab,
                            CrossKeyCode::Backspace => KeyCode::Backspace,
                            CrossKeyCode::Up => KeyCode::ArrowUp,
                            CrossKeyCode::Down => KeyCode::ArrowDown,
                            CrossKeyCode::Left => KeyCode::ArrowLeft,
                            CrossKeyCode::Right => KeyCode::ArrowRight,
                            _ => KeyCode::Unknown,
                        };
                        
                        if let KeyCode::Char('q') = code { self.should_quit = true; }
                        
                        self.dispatch_event(RawInputEvent::Key { 
                            key: code, 
                            state: ButtonState::Pressed, 
                            modifiers: Modifiers::default() 
                        });
                    }
                    Event::Mouse(mouse) => {
                        let pos = Vec2::new(mouse.column as f32, mouse.row as f32);
                        self.dispatch_event(RawInputEvent::PointerMove { position: pos });

                        let button = match mouse.button {
                            CrossMouseButton::Left => Some(PointerButton::Primary),
                            CrossMouseButton::Right => Some(PointerButton::Secondary),
                            CrossMouseButton::Middle => Some(PointerButton::Auxiliary),
                            _ => None,
                        };

                        if let Some(btn) = button {
                            let state = match mouse.kind {
                                MouseEventKind::Down(_) => Some(ButtonState::Pressed),
                                MouseEventKind::Up(_) => Some(ButtonState::Released),
                                _ => None,
                            };
                            if let Some(st) = state {
                                self.dispatch_event(RawInputEvent::PointerButton { button: btn, state: st });
                            }
                        }
                    }
                    Event::Resize(w, h) => {
                        current_size = Vec2::new(w as f32, h as f32);
                        self.dispatch_event(RawInputEvent::Resize { size: current_size, scale_factor: 1.0 });
                    }
                    _ => {}
                }
            }
        }

        execute!(stdout, event::DisableMouseCapture, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn handle_redraw(&mut self, size: Vec2) {
        if let Some(root) = &self.root {
            // TUI redraw logic (L2) will be called here
            let _root_node = self.layout_engine.compute(root.as_ref(), size.x, size.y);
            // In the future, Layer 2 will draw characters to the terminal buffer here.
        }
    }

    fn dispatch_event(&mut self, event: RawInputEvent) {
        if let Some(root) = &self.root {
            // Since TUI doesn't store NodeId in L1 yet, we use the engine to find the root node
            // Note: In a real implementation, we'd cache the NodeId from handle_redraw.
            let root_node = self.layout_engine.compute(root.as_ref(), 80.0, 24.0); 
            EventDispatcher::dispatch(
                event,
                root.as_ref(),
                &self.layout_engine.taffy,
                root_node,
                &mut self.cursor_pos,
            );
        }
    }
}
