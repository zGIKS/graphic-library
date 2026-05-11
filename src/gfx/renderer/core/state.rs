use super::super::RectInstance;
use wgpu::{Color, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};

pub struct Renderer {
    pub(super) device: Device,
    pub(super) queue: Queue,
    pub(super) surface: Surface,
    pub(super) config: SurfaceConfiguration,
    pub(super) pipeline: RenderPipeline,
    pub(super) globals_bg: wgpu::BindGroup,
    pub(super) globals_buffer: wgpu::Buffer,
    pub(super) clear_color: Color,
    pub(super) quad_vertex_buffer: wgpu::Buffer,
    pub(super) quad_index_buffer: wgpu::Buffer,
    pub(super) quad_index_count: u32,
    pub(super) instance_buffer: wgpu::Buffer,
    pub(super) instance_capacity: u64,
    pub(super) instances: Vec<RectInstance>,
}

