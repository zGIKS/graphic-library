fn main() {
    let app = rendui::run("Rendui", 800, 600).expect("Failed to init");

    app.run_loop(|_renderer, _scene| {}).expect("Failed to run loop");
}
