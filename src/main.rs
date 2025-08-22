use std::{
    fmt::Display, fs::{self, File}, io::{Write, Error as IoError}, process::exit
};
use eframe::{App, NativeOptions};
use egui::{
    CentralPanel, Color32, Context, IconData, Label,
    Margin, TextEdit, Ui, Vec2, ViewportBuilder
};
use regex::{Regex, Error as RegexError};
use reqwest::{
    header::{HeaderMap, HeaderValue}, redirect::Policy,
    ClientBuilder, Error as ReqwestError
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error as SerdeError};
use std::env::VarError;
use dotenvy::Error as DotEnvError;

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let _ctx = Context::default();
    let _ = dotenvy::dotenv()
        .map_err(|error| Errors::DotEnvError(error))?;
    let username = std::env::var("BSKYUSERNAME")
        .map_err(|error| Errors::EnvVarError(error))?;
    let password = std::env::var("BSKYPASSWORD")
        .map_err(|error| Errors::EnvVarError(error))?;
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
        height
    };
    let frame = ViewportBuilder::default()
        .with_icon(icon)
        .with_inner_size(window_size);
    let options = NativeOptions { viewport: frame,..Default::default() };
    let _ = eframe::run_native("Archiver", options, Box::new(|_| {
        Ok(Box::new(post))
    }));
    Ok(())
}

#[derive(Debug)]
pub struct PostInformation {
    pub username: String,
    pub password: String,
    pub url: String
}

impl PostInformation {
    fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            url: "".to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Did {
    pub did: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BskyCreds {
    pub did: String,
    pub handle: String,
    pub email: String,
    pub email_confirmed: bool,
    pub email_auth_factor: bool,
    pub access_jwt: String,
    pub refresh_jwt: String,
    pub active: bool
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
        ui.horizontal(|ui| {
            ui.add(
          Label::new("Bluesky Archive Tool")
            )
        });
        ui.horizontal(|ui| {
            let label = ui.label("Post URL");
            ui.centered_and_justified(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.url)
                        .text_color(Color32::from_rgb(0, 200, 0))
                        .desired_width(250.0)
                        .margin(Margin::same(4))
                        .hint_text("Enter URL of post")
                        .background_color(Color32::from_gray(10))
                )
                .labelled_by(label.id)
            })
        });
        ui.add_space(10.0);
        if ui.button("Archive").clicked() {
                let url = self.url.clone();
                let username = self.username.clone();
                let password = self.password.clone();

                tokio::spawn(async move {
                    let _ = archive(url, username, password).await;
                });
            }
            Ok(())
    }
}

pub async fn archive(url: String, username: String, password: String) -> Result<(), Errors> {
    let post_id = Regex::new(r"profile/([a-zA-Z0-9._-]+)/post/([A-Za-z0-9._:~-]+)")
        .map_err(|error| Errors::RegexError(error))?;
    let captures = &post_id.captures(&url);
    let post_info_pieces = match captures {
        Some(captures) => captures,
        None => exit(69)
    };
    let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .redirect(Policy::limited(100))
        .default_headers(headers)
        .build()
        .map_err(|error| Errors::ReqwestSendError(error))?;
    let url = format!("https://web.archive.org/save/{}", &url);

    // Here we will login to Bluesky, get a JWT token, then get the post
    let auth_request = client
        .post("https://bsky.social/xrpc/com.atproto.server.createSession")
        .body(json!({
            "identifier": username,
            "password": password
        })
        .to_string())
        .send()
        .await
        .map_err(|error| Errors::ReqwestSendError(error))?
        .bytes()
        .await
        .map_err(|error| Errors::ReqwestBytesError(error))?
        .to_vec();
    let creds = serde_json::from_slice::<BskyCreds>(&auth_request).map_err(|error| Errors::DeserializeError(error))?;
    let request = client
        .get(format!("https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle={}", &post_info_pieces[1]))
        .send()
        .await
        .map_err(|error| Errors::ReqwestSendError(error))?
        .bytes()
        .await
        .map_err(|error| Errors::ReqwestBytesError(error))?
        .to_vec();
    let did = serde_json::from_slice::<Did>(&request)
        .map_err(|error| Errors::DeserializeError(error))?;
    let post_bytes = client
        .get(format!("https://bsky.social/xrpc/app.bsky.feed.getPostThread?uri=at://{}/app.bsky.feed.post/{}", did.did, &post_info_pieces[2]))
        .bearer_auth(creds.access_jwt)
        .send()
        .await
        .map_err(|error| Errors::ReqwestSendError(error))?
        .bytes()
        .await
        .map_err(|error| Errors::ReqwestBytesError(error))?
        .to_vec();
    let post_data = serde_json::from_slice::<ThreadData>(&post_bytes)
        .map_err(|error| Errors::DeserializeError(error))?;
    if let Some(post) = post_data
        .thread
        .post {
        if let Some(record) = post
            .record {
            // Write the post content to a file to preserve its contents locally
            let _ = fs::create_dir(format!("./posts/{}", &post_info_pieces[2]));
            let mut file = File::create(&format!("./posts/{}/raw.json", &post_info_pieces[2]))
                .map_err(|error| Errors::FileCreateError(error))?;
            let _ = file.write(&post_bytes);
            println!("Raw post data archived...Saving associated media...");
            if let Some(media) = record
                .embed {
                for image in media.images {
                    let image = match &image.image {
                        Some(image) => image,
                        None => break,  
                    };
                    let referer = match &image.referer {
                        Some(referer) => referer,
                        None => break,
                    };
                    let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
                    let blob = client
                        .get(&url)
                        .send()
                        .await
                        .map_err(|error| Errors::ReqwestSendError(error))?
                        .bytes()
                        .await
                        .map_err(|error| Errors::ReqwestBytesError(error))?
                        .to_vec();
                    let mut image_file = File::create(format!("./posts/{}/{}.png", &post_info_pieces[2], &referer.resource_cid))
                        .map_err(|error| Errors::FileCreateError(error))?;
                    let _ = image_file.write(&blob);
                    println!("Saved {}", &referer.resource_cid)
                }

                if let Some(video) = media.video {
                    println!("Saving video from post");
                    let referer = match video.referer {
                        Some(referer) => referer,
                        None => exit(71)
                    };
                    let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
                    let blob = client
                        .get(&url)
                        .send()
                        .await
                        .map_err(|error| Errors::ReqwestSendError(error))?;
                    let blob = match blob
                        .bytes()
                        .await {
                        Ok(bytes) => bytes,
                        Err(_) => panic!("Unable to get response bytes")
                    }.to_vec();
                    let mut image_file = File::create(format!("./posts/{}/{}.mp4", &post_info_pieces[2], &referer.resource_cid))
                        .map_err(|error| Errors::FileCreateError(error))?;
                    let _ = image_file.write(&blob);
                }
            }
        }
    }
    println!("Archiving externally...");
              
    let _ = client
        .get(url)
        .send()
        .await;              
              
    println!("Post archived successfully.");
    
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllowIncoming {
    All,
    Following,
    None
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub allow_incoming: AllowIncoming
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllowSubscriptions {
    All,
    Followers,
    None
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySubscription {
    pub allow_subscriptions: AllowSubscriptions
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Associated {
    pub chat: Chat,
    pub activity_subscription: ActivitySubscription
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub muted: bool,
    pub blocked_by: bool
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    pub uri: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub byte_end: u16,
    pub byte_start: u16
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    #[serde(default)]
    pub features: Vec<Feature>,
    pub index: Index,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAuthor {
    pub did: String,
    pub handle: String,
    pub display_name: String,
    pub avatar: String,
    pub associated: Associated,
    pub viewer: Viewer,
    #[serde(default)]
    pub labels: Vec<String>,
    pub created_at: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRecord {
    pub uri: Option<String>,
    pub cid: String,
    pub author: PostAuthor,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostViewer {
    pub thread_muted: bool,
    pub embedding_disabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AspectRatio {
    pub height: u16,
    pub width: u16
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedMediaData {
    pub cid: Option<String>,
    pub playlist: Option<String>,
    pub thumbnail: Option<String>,
    pub thumb: Option<String>,
    pub fullsize: Option<String>,
    pub alt: Option<String>,
    pub aspect_ratio: Option<AspectRatio>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    #[serde(default)]
    pub images: Vec<EmbedMediaData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageRef {
    #[serde(rename = "$link")]
    pub resource_cid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaBlobData {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    #[serde(rename = "ref")]
    pub referer: Option<ImageRef>,
    pub mime_type: String,
    pub size: u64
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordEmbedMedia {
    pub alt: Option<String>,
    pub aspect_ratio: Option<AspectRatio>,
    pub image: Option<MediaBlobData>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordEmbed {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    #[serde(default)]
    pub images: Vec<RecordEmbedMedia>,
    pub alt: Option<String>,
    pub aspect_ratio: Option<AspectRatio>,
    pub video: Option<MediaBlobData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    pub created_at: String,
    pub embed: Option<RecordEmbed>,
    #[serde(default)]
    pub facets: Vec<Facet>,
    #[serde(default)]
    pub langs: Vec<String>,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub record: Option<Record>,
    pub embed: Option<Embed>,
    pub reply_count: u64,
    pub repost_count: u64,
    pub like_count: u64,
    pub quote_count: u64,
    pub indexed_at: String,
    pub viewer: PostViewer,
    #[serde(default)]
    pub labels: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyData {
    reply_count: Option<u16>,
    repost_count: u16,
    like_count: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    #[serde(rename = "$type")]
    pub type_of: Option<String>,
    pub post: Option<PostData>,
    // TODO:
    // pub replies: Vec<ReplyData>,
    pub thread_context: Option<ThreadContext>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadContext {
    #[serde(rename = "rootAuthorLike")]
    pub root_author_like: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadData {
    pub thread: Thread
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub did: String,
}

#[derive(Debug)]
pub enum Errors {
    ReqwestSendError(ReqwestError),
    HttpClientBuildError(ReqwestError),
    ReqwestBytesError(ReqwestError),
    DotEnvError(DotEnvError),
    DeserializeError(SerdeError),
    RegexError(RegexError),
    EnvVarError(VarError),
    FileCreateError(IoError)
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqwestSendError(error) => write!(f, "Unable to send request => {error}"),
            Self::DotEnvError(error) => write!(f, "Unable to load environment file => {error}"),
            Self::DeserializeError(error) => write!(f, "Could not deserialize bytes => {error}"),
            Self::RegexError(error) => write!(f, "Unable to build regular expression => {error}"),
            Self::EnvVarError(error) => write!(f, "Could not load environment variable => {error}"),
            Self::HttpClientBuildError(error) => write!(f, "Could not build HTTP client => {error}"),
            Self::ReqwestBytesError(error) => write!(f, "Failed to get response bytes => {error}"),
            Self::FileCreateError(error) => write!(f, "Unable to create file due to error => {error}")
        }
    }
}