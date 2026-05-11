mod pipeline;
mod vertex;
pub mod core;

pub use core::Renderer;
pub use vertex::Vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct RectInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

unsafe impl bytemuck::Pod for RectInstance {}
unsafe impl bytemuck::Zeroable for RectInstance {}
