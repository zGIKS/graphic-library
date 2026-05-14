use super::Renderer;

impl Renderer {
    pub(super) fn ensure_instance_capacity(&mut self, required_bytes: u64) {
        if required_bytes <= self.instance_capacity {
            return;
        }

        let new_capacity = required_bytes.next_power_of_two().max(256 * 1024);
        self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer (resized)"),
            size: new_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.instance_capacity = new_capacity;
    }

    pub(super) fn upload_rects(&mut self, rects: &[crate::ui::Rect]) -> (u64, u32) {
        if rects.is_empty() {
            return (0, 0);
        }

        let vw = self.config.width as f32;
        let vh = self.config.height as f32;

        // Single pass: cull + pack into the persistent instance scratch buffer.
        self.instances.clear();
        self.instances.reserve(rects.len());
        for r in rects {
            if r.x < vw && r.y < vh && r.x + r.width > 0.0 && r.y + r.height > 0.0 {
                self.instances.push(super::super::RectInstance {
                    pos: [r.x, r.y],
                    size: [r.width, r.height],
                    color: r.color,
                });
            }
        }

        let bytes = std::mem::size_of_val(self.instances.as_slice()) as u64;
        self.ensure_instance_capacity(bytes);
        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(self.instances.as_slice()),
        );

        (bytes, self.instances.len() as u32)
    }
}
