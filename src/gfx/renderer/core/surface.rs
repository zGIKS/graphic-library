use super::super::pipeline;
use super::Renderer;
use wgpu::Color;

impl Renderer {
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
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
        match self.surface.get_current_texture() {
            Ok(frame) => Some(frame),
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                if self.config.width > 0 && self.config.height > 0 {
                    self.surface.configure(&self.device, &self.config);
                }
                None
            }
            Err(wgpu::SurfaceError::Timeout) => None,
            Err(wgpu::SurfaceError::OutOfMemory) => {
                panic!("wgpu surface out of memory");
            }
        }
    }
}

