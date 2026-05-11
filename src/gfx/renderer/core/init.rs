use super::super::{pipeline, Vertex};
use super::Renderer;
use wgpu::util::DeviceExt;
use wgpu::{Color, SurfaceConfiguration};
use winit::window::Window;

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, String> {
        let backends = wgpu::Backends::all();
        let instance = wgpu::Instance::new(backends);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .enumerate_adapters(backends)
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

        // Prefer low-latency present modes when available; fall back to FIFO for compatibility.
        let formats = surface.get_supported_formats(&adapter);
        let alpha_modes = surface.get_supported_alpha_modes(&adapter);

        let format = formats
            .first()
            .copied()
            .unwrap_or(wgpu::TextureFormat::Bgra8Unorm);

        let present_modes = surface.get_supported_present_modes(&adapter);
        let present_mode = if present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox // lowest latency, ideal for resizing
        } else if present_modes.contains(&wgpu::PresentMode::Immediate) {
            wgpu::PresentMode::Immediate
        } else {
            wgpu::PresentMode::Fifo
        };
        let alpha_mode = alpha_modes
            .first()
            .copied()
            .unwrap_or(wgpu::CompositeAlphaMode::Auto);

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode,
        };
        surface.configure(&device, &config);

        let globals_layout = pipeline::create_bind_group_layout(&device);
        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("globals buffer"),
            size: std::mem::size_of::<pipeline::Globals>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let globals_bg = pipeline::create_bind_group(&device, &globals_layout, &globals_buffer);

        let pipeline = pipeline::create_pipeline(&device, format, &globals_layout);

        let quad_vertices: [Vertex; 4] = [
            Vertex::new(0.0, 0.0),
            Vertex::new(1.0, 0.0),
            Vertex::new(0.0, 1.0),
            Vertex::new(1.0, 1.0),
        ];
        let quad_indices: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("quad vertex buffer"),
            contents: bytemuck::cast_slice(&quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let quad_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("quad index buffer"),
            contents: bytemuck::cast_slice(&quad_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let initial_instance_capacity: u64 = 16 * 1024 * 1024; // 16MB
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer"),
            size: initial_instance_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut this = Self {
            device,
            queue,
            surface,
            config,
            surface_needs_configure: false,
            pipeline,
            globals_bg,
            globals_buffer,
            clear_color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            quad_vertex_buffer,
            quad_index_buffer,
            quad_index_count: quad_indices.len() as u32,
            instance_buffer,
            instance_capacity: initial_instance_capacity,
            instances: Vec::with_capacity(4096),
        };

        this.update_globals();
        Ok(this)
    }
}
