use crate::lib::{Errors, archive, post::Post};
use eframe::App;
use egui::{CentralPanel, Color32, Label, Margin, TextEdit, Ui};
use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
pub struct PostInformation {
    pub username: String,
    pub password: String,
    pub url: String,
}

impl PostInformation {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            url: "".to_string(),
        }
    }
}

impl Default for PostInformation {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            url: String::new(),
        }
    }
}

impl App for PostInformation {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |interface| {
            let _ = self.build_gui(interface);
        });
    }
}

impl PostInformation {
    pub fn build_gui(&mut self, ui: &mut Ui) -> Result<(), Errors> {
        ui.horizontal(|ui| ui.add(Label::new("Bluesky Archive Tool")));
        ui.horizontal(|ui| {
            let label = ui.label("Post URL");
            ui.centered_and_justified(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.url)
                        .text_color(Color32::from_rgb(0, 200, 0))
                        .desired_width(250.0)
                        .margin(Margin::same(4))
                        .hint_text("Enter URL of post")
                        .background_color(Color32::from_gray(10)),
                )
                .labelled_by(label.id)
            })
        });
        ui.add_space(10.0);
        if ui.button("Archive").clicked() {
            let info = self.clone();
            let posts_dir_exists = fs::exists("./posts")?;
            let post_id_regex = Regex::new(r"profile/([a-zA-Z0-9._-]+)/post/([A-Za-z0-9._:~-]+)")?;
            let post = Post {
                info,
                posts_dir_exists,
                post_id_regex,
            };

            tokio::spawn(archive(post));
        }

        Ok(())
    }
}
