use winit::{
    event::{ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone, Copy)]
pub struct WindowEvents {
    pub close_requested: bool,
    pub resized: Option<(u32, u32)>,
    pub redraw_requested: bool,
}

impl WindowEvents {
    pub fn new() -> Self {
        Self {
            close_requested: false,
            resized: None,
            redraw_requested: false,
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
        self.event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::Wait;
            let mut events = WindowEvents::new();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        events.close_requested = true;
                    }
                    WindowEvent::Resized(size) => {
                        events.resized = Some((size.width, size.height));
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        events.resized = Some((new_inner_size.width, new_inner_size.height));
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
                Event::RedrawRequested(_) => {
                    events.redraw_requested = true;
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }

            if events.close_requested {
                *control_flow = ControlFlow::Exit;
            }

            if events.redraw_requested || events.resized.is_some() {
                on_frame(&window, events);
            }
        });
    }
}
