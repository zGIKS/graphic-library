pub mod app;
pub mod gfx;
pub mod platform;
pub mod ui;

pub use app::App;
pub use gfx::{Renderer, Vertex};
pub use platform::{AdapterInfo, AppWindow, InputEvent, InputState, MousePos, Platform, WindowEvents};
pub use ui::{Rect, Scene};

pub fn run(title: &str, width: u32, height: u32) -> Result<App, String> {
    let window = AppWindow::new(title, width, height)?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;

    let renderer = rt.block_on(async { Renderer::new(window.window()).await })?;
    Ok(App::new(window, renderer))
}

