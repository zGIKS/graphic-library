use super::{pipeline, Vertex};
use wgpu::{Color, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::window::Window;

pub struct Renderer {
    device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    clear_color: Color,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity_bytes: u64,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|a| a.is_surface_supported(&surface))
            .ok_or("No suitable adapter found")?;

        println!("Using adapter: {:?}", adapter.get_info().name);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to request device: {}", e))?;

        let format = wgpu::TextureFormat::Bgra8Unorm;
        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let pipeline = pipeline::create_pipeline(&device, format);

        let initial_capacity_bytes = 1024 * 1024;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex buffer"),
            size: initial_capacity_bytes,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            device,
            queue,
            surface,
            config,
            pipeline,
            clear_color: Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
            vertex_buffer,
            vertex_capacity_bytes: initial_capacity_bytes,
        })
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    fn acquire_frame(&mut self) -> Option<wgpu::SurfaceTexture> {
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

    fn ensure_vertex_capacity(&mut self, required_bytes: u64) {
        if required_bytes <= self.vertex_capacity_bytes {
            return;
        }

        let new_capacity = required_bytes.next_power_of_two().max(1024 * 1024);
        self.vertex_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex buffer (resized)"),
            size: new_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.vertex_capacity_bytes = new_capacity;
    }

    pub fn render_vertices(&mut self, vertices: &[Vertex]) {
        if vertices.is_empty() {
            self.render_placeholder();
            return;
        }

        let required_bytes = std::mem::size_of_val(vertices) as u64;
        self.ensure_vertex_capacity(required_bytes);
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));

        let Some(target) = self.acquire_frame() else {
            return;
        };
        let view = target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("command encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..required_bytes));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        target.present();
    }

    pub fn render_placeholder(&mut self) {
        let Some(target) = self.acquire_frame() else {
            return;
        };
        let view = target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("command encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        target.present();
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
