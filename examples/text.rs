fn main() {
    let app = rendui::run("Rendui Text", 800, 600).expect("Failed to init");

    app.run_loop(|_renderer, scene| {
        scene.add_rect(rendui::Rect::new(
            32.0,
            32.0,
            420.0,
            132.0,
            [0.08, 0.10, 0.14, 1.0],
        ));

        scene.add_text(rendui::Text::new(
            56.0,
            56.0,
            360.0,
            88.0,
            28.0,
            "Texto renderizado con GPU",
            [245, 247, 250, 255],
        ));
    })
    .expect("Failed to run loop");
}
