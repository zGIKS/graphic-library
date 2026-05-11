use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use std::sync::Arc;
use parking_lot::Mutex;

pub struct AppWindow {
    window: Window,
    event_loop: EventLoop<()>,
    close_requested: Arc<Mutex<bool>>,
}

impl AppWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                width as f64,
                height as f64,
            ))
            .build(&event_loop)
            .map_err(|e| format!("Failed to create window: {}", e))?;

        let close_requested = Arc::new(Mutex::new(false));

        Ok(Self {
            window,
            event_loop,
            close_requested,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn close_requested(&self) -> bool {
        *self.close_requested.lock()
    }

    pub fn run<F>(self, mut update_fn: F)
    where
        F: FnMut(&Window) + 'static,
    {
        let close_requested = self.close_requested.clone();

        self.event_loop.run(
            move |event, _target, _control_flow| {
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        *close_requested.lock() = true;
                    }
                    Event::MainEventsCleared => {
                        update_fn(&self.window);
                    }
                    _ => {}
                }
            },
        );
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}