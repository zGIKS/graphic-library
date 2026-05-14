use crate::gfx::Renderer;
use crate::platform::AppWindow;
use crate::ui::Scene;
use std::time::{Duration, Instant};

pub struct App {
    renderer: Renderer,
    window: AppWindow,
}

impl App {
    pub fn new(window: AppWindow, renderer: Renderer) -> Self {
        Self { renderer, window }
    }

    pub fn run_loop<F>(self, mut update_fn: F) -> Result<(), String>
    where
        F: FnMut(&mut Renderer, &mut Scene) + 'static,
    {
        let (w, h) = self.window.inner_size();

        let mut renderer = self.renderer;
        let window = self.window;
        let mut scene = Scene::new(w as f32, h as f32);
        let mut last_rendered_version: u64 = u64::MAX; // Force first frame
        let mut last_frame_time = Instant::now();
        const MIN_FRAME_TIME: Duration = Duration::from_nanos(16_666_667); // ~60 FPS

        window.run(move |_window, events| {
            let mut force_render = false;

            if let Some((rw, rh)) = events.resized {
                scene.update_size(rw as f32, rh as f32);
                renderer.resize(rw, rh);
                force_render = true; // Viewport changed, must re-render
            }

            if events.interactive {
                force_render = true;
            }

            // Frame pacing: skip non-forced frames that arrive too fast,
            // but always allow the very first frame so the window shows up.
            let now = Instant::now();
            let is_first_frame = last_rendered_version == u64::MAX;
            if !force_render && !is_first_frame && now.duration_since(last_frame_time) < MIN_FRAME_TIME {
                return;
            }

            scene.clear();
            update_fn(&mut renderer, &mut scene);

            if force_render || scene.version() != last_rendered_version {
                renderer.render_rects(scene.rects());
                last_rendered_version = scene.version();
                last_frame_time = now;
            }
        })
    }
}
