use rendui::{InputEvent, Rect, Text};

fn main() {
    let app = rendui::run("Rendui Input", 800, 600).expect("Failed to init");
    let mut typed = String::new();
    let mut last_event = String::from("Interactua con mouse o teclado");

    app.run_loop_events(move |_renderer, scene, events| {
        for event in &events.input_events {
            match event {
                InputEvent::Text(text) => typed.push_str(text),
                InputEvent::KeyDown(key) => last_event = format!("KeyDown: {key:?}"),
                InputEvent::KeyUp(key) => last_event = format!("KeyUp: {key:?}"),
                InputEvent::MouseMove(pos) => {
                    last_event = format!("MouseMove: {:.0}, {:.0}", pos.x, pos.y)
                }
                InputEvent::MouseDown(button, pos) => {
                    last_event = format!("MouseDown {button:?}: {:.0}, {:.0}", pos.x, pos.y)
                }
                InputEvent::MouseUp(button, pos) => {
                    last_event = format!("MouseUp {button:?}: {:.0}, {:.0}", pos.x, pos.y)
                }
                InputEvent::MouseWheel(x, y) => {
                    last_event = format!("MouseWheel: {x:.1}, {y:.1}")
                }
            }
        }

        scene.add_rect(Rect::new(32.0, 32.0, 560.0, 210.0, [0.08, 0.10, 0.14, 1.0]));
        scene.add_text(Text::new(
            56.0,
            56.0,
            500.0,
            36.0,
            22.0,
            "Input events",
            [245, 247, 250, 255],
        ));
        scene.add_text(Text::new(
            56.0,
            104.0,
            500.0,
            32.0,
            18.0,
            last_event.clone(),
            [180, 190, 210, 255],
        ));
        scene.add_text(Text::new(
            56.0,
            148.0,
            500.0,
            64.0,
            18.0,
            format!("Texto: {typed}"),
            [140, 220, 170, 255],
        ));
    })
    .expect("Failed to run loop");
}
