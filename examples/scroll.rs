use rendui::{ClipRect, InputEvent, Rect, Text};

const LINE_COUNT: usize = 100;
const LINE_HEIGHT: f32 = 42.0;

fn main() {
    let app = rendui::run("Rendui Scroll", 800, 600).expect("Failed to init");
    let mut scroll_y = 0.0_f32;
    let labels = (0..LINE_COUNT)
        .map(|index| format!("Linea {} - contenido dentro del viewport", index + 1))
        .collect::<Vec<_>>();

    app.run_loop_events(move |_renderer, scene, events| {
        for event in &events.input_events {
            if let InputEvent::MouseWheel(_, y) = event {
                let max_scroll = (LINE_COUNT as f32 * LINE_HEIGHT - 380.0).max(0.0);
                scroll_y = (scroll_y - (*y as f32 * 28.0)).clamp(0.0, max_scroll);
            }
        }

        scene.add_rect(Rect::new(0.0, 0.0, scene.width, scene.height, [0.03, 0.04, 0.06, 1.0]));
        scene.add_text(Text::new(
            40.0,
            28.0,
            600.0,
            32.0,
            24.0,
            "ScrollArea basico",
            [245, 247, 250, 255],
        ));

        let viewport = ClipRect::new(40.0, 80.0, 420.0, 380.0);
        scene.add_rect(Rect::new(
            viewport.x,
            viewport.y,
            viewport.width,
            viewport.height,
            [0.08, 0.10, 0.14, 1.0],
        ));

        scene.push_clip(viewport);
        scene.push_offset(0.0, -scroll_y);

        let first_visible = (scroll_y / LINE_HEIGHT).floor().max(0.0) as usize;
        let visible_count = (viewport.height / LINE_HEIGHT).ceil() as usize + 2;
        let last_visible = (first_visible + visible_count).min(LINE_COUNT);

        for index in first_visible..last_visible {
            let y = 96.0 + index as f32 * LINE_HEIGHT;
            let shade = if index % 2 == 0 { 0.12 } else { 0.10 };
            scene.add_rect(Rect::new(56.0, y, 388.0, 32.0, [shade, shade + 0.02, 0.18, 1.0]));
            scene.add_text(Text::new(
                68.0,
                y + 5.0,
                340.0,
                24.0,
                17.0,
                labels[index].clone(),
                [220, 226, 240, 255],
            ));
        }

        scene.pop_offset();
        scene.pop_clip();

        let content_h = LINE_COUNT as f32 * LINE_HEIGHT;
        let max_scroll = (content_h - viewport.height).max(0.0);
        let thumb_h = (viewport.height * viewport.height / content_h).max(42.0);
        let thumb_y = if max_scroll > 0.0 {
            viewport.y + (viewport.height - thumb_h) * (scroll_y / max_scroll)
        } else {
            viewport.y
        };
        scene.add_rect(Rect::new(472.0, viewport.y, 8.0, viewport.height, [0.12, 0.14, 0.18, 1.0]));
        scene.add_rect(Rect::new(472.0, thumb_y, 8.0, thumb_h, [0.45, 0.55, 0.75, 1.0]));
    })
    .expect("Failed to run loop");
}
