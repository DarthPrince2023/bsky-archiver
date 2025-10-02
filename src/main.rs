pub mod lib;

use eframe::NativeOptions;
use egui::{Context, IconData, Vec2, ViewportBuilder};

// Re exports
pub use lib::post_information::PostInformation;

use crate::lib::Errors;

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let _ctx = Context::default();
    let _ = dotenvy::dotenv()?;
    let username = std::env::var("BSKYUSERNAME")?;
    let password = std::env::var("BSKYPASSWORD")?;
    let post = PostInformation::new(username, password);
    let window_size = Vec2::new(450.0, 150.0);
    let icon_bytes = include_bytes!("../res/favicon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();
    let icon = IconData {
        rgba: image.into_raw(),
        width,
        height,
    };
    let frame = ViewportBuilder::default()
        .with_icon(icon)
        .with_inner_size(window_size);
    let options = NativeOptions {
        viewport: frame,
        ..Default::default()
    };
    let _ = eframe::run_native("Archiver", options, Box::new(|_| Ok(Box::new(post))));
    
    Ok(())
}
