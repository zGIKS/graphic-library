use super::Renderer;

impl Renderer {
    pub fn render_rects(&mut self, rects: &[crate::ui::Rect]) {
        let (required_bytes, visible_count) = self.upload_rects(rects);

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
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.globals_bg, &[]);
            render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            if visible_count > 0 {
                render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..required_bytes));
                render_pass.draw_indexed(0..self.quad_index_count, 0, 0..visible_count);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        target.present();
    }
}
