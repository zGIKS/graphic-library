use rendui::Rect;

fn main() {
    let (window, renderer) = rendui::run("Rendui MVP", 800, 600).expect("Failed to init");

    let app = rendui::App { renderer, window };

    app.run_loop(|_renderer, scene| {
        scene.add_rect(Rect::new(100.0, 100.0, 200.0, 150.0, [1.0, 0.3, 0.3, 1.0]));
        scene.add_rect(Rect::new(400.0, 200.0, 150.0, 100.0, [0.3, 1.0, 0.3, 1.0]));
        scene.add_rect(Rect::new(200.0, 400.0, 300.0, 80.0, [0.3, 0.3, 1.0, 1.0]));
    });
}
