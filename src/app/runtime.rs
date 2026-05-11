use crate::gfx::Renderer;
use crate::platform::AppWindow;
use crate::ui::Scene;

pub struct App {
    renderer: Renderer,
    window: AppWindow,
    vertices: Vec<crate::gfx::Vertex>,
}

impl App {
    pub fn new(window: AppWindow, renderer: Renderer) -> Self {
        Self {
            renderer,
            window,
            vertices: Vec::new(),
        }
    }

    pub fn run_loop<F>(self, mut update_fn: F) -> !
    where
        F: FnMut(&mut Renderer, &mut Scene) + 'static,
    {
        let (w, h) = self.window.inner_size();

        let mut renderer = self.renderer;
        let mut vertices = self.vertices;
        let window = self.window;
        let mut scene = Scene::new(w as f32, h as f32);

        window.run(move |_window, events| {
            if let Some((rw, rh)) = events.resized {
                scene.update_size(rw as f32, rh as f32);
                renderer.resize(rw, rh);
            }

            scene.begin_frame();
            update_fn(&mut renderer, &mut scene);

            vertices.clear();
            for rect in scene.rects() {
                vertices.extend_from_slice(&rect.to_vertices(scene.width, scene.height));
            }

            renderer.render_vertices(&vertices);
        })
    }
}
