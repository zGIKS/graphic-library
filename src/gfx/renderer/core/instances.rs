use super::super::RectInstance;
use super::Renderer;

impl Renderer {
    pub(super) fn ensure_instance_capacity(&mut self, required_bytes: u64) {
        if required_bytes <= self.instance_capacity {
            return;
        }

        let new_capacity = required_bytes.next_power_of_two().max(16 * 1024 * 1024);
        self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer (resized)"),
            size: new_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.instance_capacity = new_capacity;
    }

    pub(super) fn fill_instances(&mut self, rects: &[crate::ui::Rect]) -> u64 {
        self.instances.clear();
        self.instances.reserve(rects.len());
        for r in rects {
            self.instances.push(RectInstance {
                pos: [r.x, r.y],
                size: [r.width, r.height],
                color: r.color,
            });
        }

        let required_bytes = std::mem::size_of_val(self.instances.as_slice()) as u64;
        self.ensure_instance_capacity(required_bytes);
        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(self.instances.as_slice()),
        );
        required_bytes
    }
}

