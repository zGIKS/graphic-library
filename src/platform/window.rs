use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, Event, KeyboardInput, MouseScrollDelta, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone, Copy)]
pub struct WindowEvents {
    pub close_requested: bool,
    pub resized: Option<(u32, u32)>,
    pub redraw_requested: bool,
    pub interactive: bool,
}

impl WindowEvents {
    pub fn new() -> Self {
        Self {
            close_requested: false,
            resized: None,
            redraw_requested: false,
            interactive: false,
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
        let event_loop = EventLoop::new();

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

    pub fn run<F>(self, mut on_frame: F) -> !
    where
        F: FnMut(&Window, WindowEvents) + 'static,
    {
        let window = self.window;
        let mut pending_resize: Option<(u32, u32)> = None;
        let mut interactive_until: Option<Instant> = None;
        self.event_loop.run(move |event, _target, control_flow| {
            // Default: sleep when idle. During interactive resize, poll/redraw for smoothness.
            let now = Instant::now();
            if interactive_until.is_some_and(|t| now < t) {
                *control_flow = ControlFlow::Poll;
            } else {
                interactive_until = None;
                *control_flow = ControlFlow::Wait;
            }

            match event {
                Event::NewEvents(StartCause::Init) => {
                    window.request_redraw();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        pending_resize = Some((size.width, size.height));
                        interactive_until = Some(now + Duration::from_millis(250));
                        window.request_redraw();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        pending_resize = Some((new_inner_size.width, new_inner_size.height));
                        interactive_until = Some(now + Duration::from_millis(250));
                        window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { input: KeyboardInput { state, .. }, .. } => {
                        if state == ElementState::Pressed {
                            window.request_redraw();
                        }
                    }
                    WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, _), .. } => {}
                    WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(_), .. } => {}
                    _ => {}
                },
                Event::MainEventsCleared => {
                    if interactive_until.is_some() {
                        window.request_redraw();
                    }
                }
                Event::RedrawRequested(_) => {
                    let events = WindowEvents {
                        close_requested: false,
                        resized: pending_resize.take(),
                        redraw_requested: true,
                        interactive: interactive_until.is_some(),
                    };
                    on_frame(&window, events);
                }
                _ => {}
            }
        });
    }
}
