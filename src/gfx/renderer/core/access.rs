use super::Renderer;
use wgpu::{Device, Queue};

impl Renderer {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

