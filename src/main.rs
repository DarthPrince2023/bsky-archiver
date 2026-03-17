pub mod lib;

use eframe::NativeOptions;
use egui::{Context, IconData, Vec2, ViewportBuilder};

// Re exports
pub use lib::post_information::PostInformation;

use crate::lib::errors::Errors;

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


use std::collections::HashMap;

fn get_user_score(users: &HashMap<&str, i32>, name: &str) -> Option<i32> {
    users.get(name).copied()
}

fn compute_bonus(score: i32) -> Option<i32> {
    if score > 50 {
        Some(score + 10)
    } else {
        None
    }
}

fn main() {
    let mut users = HashMap::new();
    users.insert("alice", 90);
    users.insert("bob", 40);

    let username = "bob";

    // Looks harmless
    let score = get_user_score(&users, username).unwrap();

    // Hidden failure path
    let bonus = compute_bonus(score).unwrap();

    println!("User bonus: {bonus}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[should_panic]
    fn unwrap_panics_when_bonus_is_none() {
        let mut users = HashMap::new();
        users.insert("bob", 40);

        let score = get_user_score(&users, "bob").unwrap();
        let _bonus = compute_bonus(score).unwrap(); // should panic
    }

    #[test]
    fn unwrap_does_not_panic_when_bonus_exists() {
        let mut users = HashMap::new();
        users.insert("alice", 90);

        let score = get_user_score(&users, "alice").unwrap();
        let bonus = compute_bonus(score).unwrap();

        assert_eq!(bonus, 100);
    }

    #[test]
    #[should_panic]
    fn unwrap_panics_when_user_does_not_exist() {
        let users = HashMap::new();

        // get_user_score returns None
        let _score = get_user_score(&users, "ghost").unwrap();
    }
}
