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

    pub(super) fn upload_rects(&mut self, rects: &[crate::ui::Rect]) -> (u64, u32) {
        if rects.is_empty() {
            return (0, 0);
        }

        let vw = self.config.width as f32;
        let vh = self.config.height as f32;

        // Fast-path: if everything fits, upload without allocation.
        let required_bytes = std::mem::size_of_val(rects) as u64;
        self.ensure_instance_capacity(required_bytes);

        // If no rects are culled, zero-copy upload the whole slice.
        let all_visible = rects.iter().all(|r| {
            r.x < vw && r.y < vh && r.x + r.width > 0.0 && r.y + r.height > 0.0
        });

        if all_visible {
            self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(rects),
            );
            return (required_bytes, rects.len() as u32);
        }

        // Slow-path: filter out invisible rects into a temporary stack buffer.
        // 4096 rects * 32 bytes = 128 KiB, well within typical stack limits.
        const STACK_LIMIT: usize = 4096;
        let visible_count = rects.iter().filter(|r| {
            r.x < vw && r.y < vh && r.x + r.width > 0.0 && r.y + r.height > 0.0
        }).count();

        let bytes = (visible_count * std::mem::size_of::<crate::ui::Rect>()) as u64;
        self.ensure_instance_capacity(bytes);

        if visible_count <= STACK_LIMIT {
            let mut tmp: [crate::ui::Rect; STACK_LIMIT] = unsafe { std::mem::zeroed() };
            let mut i = 0;
            for r in rects {
                if r.x < vw && r.y < vh && r.x + r.width > 0.0 && r.y + r.height > 0.0 {
                    tmp[i] = *r;
                    i += 1;
                }
            }
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&tmp[..i]));
        } else {
            let mut tmp = Vec::with_capacity(visible_count);
            for r in rects {
                if r.x < vw && r.y < vh && r.x + r.width > 0.0 && r.y + r.height > 0.0 {
                    tmp.push(*r);
                }
            }
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&tmp));
        }

        (bytes, visible_count as u32)
    }
}
