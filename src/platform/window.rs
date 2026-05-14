use super::{InputEvent, InputState, MousePos};
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, Event, MouseScrollDelta, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone)]
pub struct WindowEvents {
    pub close_requested: bool,
    pub resized: Option<(u32, u32)>,
    pub redraw_requested: bool,
    pub interactive: bool,
    pub input: InputState,
    pub input_events: Vec<InputEvent>,
}

impl WindowEvents {
    pub fn new() -> Self {
        Self {
            close_requested: false,
            resized: None,
            redraw_requested: false,
            interactive: false,
            input: InputState::new(),
            input_events: Vec::new(),
        }
    }
}

impl Default for WindowEvents {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppWindow {
    event_loop: EventLoop<()>,
    window: Window,
}

impl AppWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let event_loop = EventLoop::new()
            .map_err(|e| format!("Failed to create event loop: {}", e))?;

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
            .build(&event_loop)
            .map_err(|e| format!("Failed to create window: {}", e))?;

        Ok(Self { event_loop, window })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn run<F>(self, mut on_frame: F) -> Result<(), String>
    where
        F: FnMut(&Window, WindowEvents) + 'static,
    {
        let window = self.window;
        let mut pending_resize: Option<(u32, u32)> = None;
        let mut input = InputState::new();
        let mut pending_input = Vec::new();
        let mut interactive_until: Option<Instant> = None;
        const INTERACTIVE_WINDOW: Duration = Duration::from_millis(200);
        self.event_loop
            .run(move |event, elwt| {
                let now = Instant::now();
                if interactive_until.is_some_and(|t| now < t) {
                    // Poll gives absolute lowest latency during resize.
                    // CPU usage is high only while dragging; we sleep after.
                    elwt.set_control_flow(ControlFlow::Poll);
                } else {
                    interactive_until = None;
                    elwt.set_control_flow(ControlFlow::Wait);
                }

                match event {
                    Event::NewEvents(StartCause::Init) => {
                        window.request_redraw();
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            pending_resize = Some((size.width, size.height));
                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let new_size = window.inner_size();
                            pending_resize = Some((new_size.width, new_size.height));
                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            if let PhysicalKey::Code(code) = event.physical_key {
                                let pressed = event.state == ElementState::Pressed;
                                input.set_key(code, pressed);
                                if pressed {
                                    pending_input.push(InputEvent::KeyDown(code));
                                } else {
                                    pending_input.push(InputEvent::KeyUp(code));
                                }
                            }

                            if event.state == ElementState::Pressed {
                                if let Some(text) = event.text {
                                    if !text.is_empty() {
                                        pending_input.push(InputEvent::Text(text.to_string()));
                                    }
                                }
                            }

                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::ModifiersChanged(modifiers) => {
                            input.modifiers = modifiers.state();
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let pos = MousePos::new(position.x, position.y);
                            input.mouse_pos = Some(pos);
                            pending_input.push(InputEvent::MouseMove(pos));
                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let pos = input.mouse_pos.unwrap_or(MousePos::new(0.0, 0.0));
                            let pressed = state == ElementState::Pressed;
                            input.set_mouse_button(button, pressed);
                            if pressed {
                                pending_input.push(InputEvent::MouseDown(button, pos));
                            } else {
                                pending_input.push(InputEvent::MouseUp(button, pos));
                            }
                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let (x, y) = match delta {
                                MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                            };
                            pending_input.push(InputEvent::MouseWheel(x, y));
                            interactive_until = Some(now + INTERACTIVE_WINDOW);
                            window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                            let events = WindowEvents {
                                close_requested: false,
                                resized: pending_resize.take(),
                                redraw_requested: true,
                                interactive: interactive_until.is_some(),
                                input: input.clone(),
                                input_events: std::mem::take(&mut pending_input),
                            };
                            on_frame(&window, events);
                        }
                        _ => {}
                    },
                    Event::AboutToWait => {
                        // If a resize just happened, render IMMEDIATELY in this cycle
                        // instead of waiting for RedrawRequested. This eliminates one
                        // full event-loop iteration, matching Alacritty's approach.
                        if let Some(size) = pending_resize.take() {
                            let events = WindowEvents {
                                close_requested: false,
                                resized: Some(size),
                                redraw_requested: false,
                                interactive: interactive_until.is_some(),
                                input: input.clone(),
                                input_events: std::mem::take(&mut pending_input),
                            };
                            on_frame(&window, events);
                        } else if interactive_until.is_some() {
                            window.request_redraw();
                        }
                    }
                    _ => {}
                }
            })
            .map_err(|e| format!("Event loop error: {}", e))
    }
}
