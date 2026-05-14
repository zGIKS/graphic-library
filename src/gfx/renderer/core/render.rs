use super::Renderer;
use glyphon::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, TextArea, TextBounds};

impl Renderer {
    pub fn render_rects(&mut self, rects: &[crate::ui::Rect]) {
        self.render_frame(rects, &[]);
    }

    pub fn render_scene(&mut self, scene: &crate::ui::Scene) {
        self.render_frame(scene.rects(), scene.texts());
    }

    fn render_frame(&mut self, rects: &[crate::ui::Rect], texts: &[crate::ui::Text]) {
        let (required_bytes, visible_count) = self.upload_rects(rects);

        let has_text = !texts.is_empty();

        if has_text {
            self.prepare_text_buffers(texts);

            let areas = self
                .text_buffers
                .iter()
                .zip(texts)
                .map(|(buffer, text)| TextArea {
                    buffer,
                    left: text.x,
                    top: text.y,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: text.x as i32,
                        top: text.y as i32,
                        right: (text.x + text.width) as i32,
                        bottom: (text.y + text.height) as i32,
                    },
                    default_color: Color::rgba(
                        text.color[0],
                        text.color[1],
                        text.color[2],
                        text.color[3],
                    ),
                });

            self.text_renderer
                .prepare(
                    &self.device,
                    &self.queue,
                    &mut self.font_system,
                    &mut self.text_atlas,
                    Resolution {
                        width: self.config.width,
                        height: self.config.height,
                    },
                    areas,
                    &mut self.text_cache,
                )
                .expect("failed to prepare text");
        }

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

            if has_text {
                self.text_renderer
                    .render(&self.text_atlas, &mut render_pass)
                    .expect("failed to render text");
            }
        }

        self.queue.submit(Some(encoder.finish()));
        target.present();
        self.text_atlas.trim();
    }

    fn prepare_text_buffers(&mut self, texts: &[crate::ui::Text]) {
        self.text_buffers.truncate(texts.len());
        self.cached_texts.truncate(texts.len());

        for (index, text) in texts.iter().enumerate() {
            if self.cached_texts.get(index) == Some(text) {
                continue;
            }

            let buffer = build_text_buffer(&mut self.font_system, text);
            if index < self.text_buffers.len() {
                self.text_buffers[index] = buffer;
                self.cached_texts[index] = text.clone();
            } else {
                self.text_buffers.push(buffer);
                self.cached_texts.push(text.clone());
            }
        }
    }
}

fn build_text_buffer(font_system: &mut FontSystem, text: &crate::ui::Text) -> Buffer {
    let mut buffer = Buffer::new(font_system, Metrics::new(text.size, text.line_height));
    buffer.set_size(font_system, text.width, text.height);
    buffer.set_text(
        font_system,
        &text.content,
        Attrs::new().family(Family::SansSerif),
        Shaping::Advanced,
    );
    buffer.shape_until_scroll(font_system);
    buffer
}
