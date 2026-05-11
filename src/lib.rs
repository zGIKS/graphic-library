pub mod platform;
pub mod window;
pub mod renderer;
pub mod scene;
pub mod input;

pub use platform::Platform;
pub use window::AppWindow;
pub use renderer::{Renderer, Vertex};
pub use scene::{Scene, Rect};
pub use input::{InputState, MousePos};

use std::thread;
use std::time::Duration;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

pub fn run(title: &str, width: u32, height: u32) -> Result<(AppWindow, Renderer), String> {
    let window = AppWindow::new(title, width, height).map_err(|e| e.to_string())?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;

    let renderer = rt.block_on(async {
        Renderer::new(window.window())
            .await
            .map_err(|e| e.to_string())
    })?;

    Ok((window, renderer))
}

pub struct App {
    pub renderer: Renderer,
    pub window: AppWindow,
}

impl App {
    pub fn run_loop<F>(self, mut update_fn: F)
    where
        F: FnMut(&Renderer, &mut Scene) + Send + 'static,
    {
        let mut scene = Scene::new(800.0, 600.0);

        loop {
            if self.window.close_requested() {
                break;
            }

            let (w, h) = self.window.inner_size();
            scene.update_size(w as f32, h as f32);

            update_fn(&self.renderer, &mut scene);

            let device = self.renderer.device();
            
            let mut all_vertices: Vec<Vertex> = Vec::new();
            for rect in &scene.rects {
                let verts = rect.to_vertices(scene.width, scene.height);
                all_vertices.extend_from_slice(&verts);
            }
            
            if !all_vertices.is_empty() {
                let vertex_buffer = device.create_buffer_init(
                    &BufferInitDescriptor {
                        label: Some("vertex buffer"),
                        contents: bytemuck::cast_slice(&all_vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }
                );

                let vertex_count = all_vertices.len() as u32;
                
                self.renderer.render_scene(vertex_buffer, vertex_count);
            } else {
                self.renderer.render_scene_placeholder();
            }

            thread::sleep(Duration::from_millis(16));
        }
    }
}