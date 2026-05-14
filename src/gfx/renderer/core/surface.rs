use super::super::pipeline;
use super::Renderer;
use wgpu::Color;

impl Renderer {
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let w = width.max(1);
        let h = height.max(1);
        if w == self.config.width && h == self.config.height {
            return;
        }

        self.config.width = w;
        self.config.height = h;
        // Configure immediately so the surface is ready before the next render
        // call. This avoids the compositor seeing an outdated buffer size.
        self.surface.configure(&self.device, &self.config);
        self.update_globals();
    }

    pub(super) fn update_globals(&mut self) {
        let globals = pipeline::Globals {
            viewport_size: [self.config.width as f32, self.config.height as f32],
            _pad: [0.0, 0.0],
        };
        self.queue
            .write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&globals));
    }

    pub(super) fn acquire_frame(&mut self) -> Option<wgpu::SurfaceTexture> {
        if self.surface_needs_configure {
            self.surface.configure(&self.device, &self.config);
            self.surface_needs_configure = false;
        }

        loop {
            match self.surface.get_current_texture() {
                Ok(frame) => return Some(frame),
                Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                    // Surface changed under us (resize, moved to another monitor, etc).
                    // Reconfigure and retry immediately; never drop a resize frame.
                    self.surface.configure(&self.device, &self.config);
                    continue;
                }
                Err(wgpu::SurfaceError::Timeout) => return None,
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    panic!("wgpu surface out of memory");
                }
            }
        }
    }
}
