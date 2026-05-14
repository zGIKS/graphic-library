use super::super::{pipeline, Vertex};
use super::Renderer;
use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer};
use wgpu::util::DeviceExt;
use wgpu::{Color, SurfaceConfiguration};
use winit::window::Window;

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .map_err(|e| format!("Failed to create surface: {:?}", e))?;

        // Safety: window outlives Renderer because Renderer is destroyed before
        // the window (App stores renderer before window, drop order is inverse).
        let surface: wgpu::Surface<'static> = unsafe { std::mem::transmute(surface) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("No suitable adapter found")?;

        println!("Using adapter: {:?}", adapter.get_info().name);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to request device: {}", e))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .first()
            .copied()
            .unwrap_or(wgpu::TextureFormat::Bgra8Unorm);

        // Prefer low-latency modes for responsive resize. Mailbox replaces the
        // previous frame immediately (no stretched-buffer artifacts). Immediate
        // renders as fast as possible. Fifo is last because it queues frames
        // and causes visible lag / bouncing during resize.
        let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox
        } else if caps.present_modes.contains(&wgpu::PresentMode::Immediate) {
            wgpu::PresentMode::Immediate
        } else if caps.present_modes.contains(&wgpu::PresentMode::FifoRelaxed) {
            wgpu::PresentMode::FifoRelaxed
        } else {
            wgpu::PresentMode::Fifo
        };
        let alpha_mode = caps
            .alpha_modes
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
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
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
        let font_system = FontSystem::new();
        let text_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&device, &queue, format);
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

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

        let initial_instance_capacity: u64 = 256 * 1024;
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
            font_system,
            text_cache,
            text_atlas,
            text_renderer,
            text_buffers: Vec::with_capacity(64),
            cached_texts: Vec::with_capacity(64),
        };

        this.update_globals();
        Ok(this)
    }
}
