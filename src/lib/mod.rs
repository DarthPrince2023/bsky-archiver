pub mod post;
pub mod post_information;

use bsky_parser::{BskyCreds, Did, ThreadData};
use dotenvy::Error as DotEnvError;
use image::EncodableLayout;
use regex::Error as RegexError;
use reqwest::{
    header::{HeaderMap, HeaderValue, ToStrError}, redirect::Policy, ClientBuilder, Error as ReqwestError
};
use serde_json::{Error as SerdeError, json};
use std::{env::VarError, fmt::Display, fs::{self, OpenOptions}, io::{Error as IoError, Read, Write}, net::TcpStream, num::ParseIntError, os::windows::fs::FileExt, process::exit};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufWriter}};
use native_tls::{Error as NativeTlsError, HandshakeError};

use crate::lib::post::Post;

#[derive(Debug)]
pub enum MediaType {
    Mp4,
    Mov,
    Webm,
    Mpeg,
    Invalid,
}

impl<'a> From<&'a str> for MediaType {
    fn from(value: &'a str) -> Self {
        match value {
            "video/mp4" => Self::Mp4,
            "video/mov" => Self::Mov,
            "video/webm" => Self::Webm,
            "video/mpeg" => Self::Mpeg,
            _ => Self::Invalid
        }
    }
}

impl<'a> Into<&'a str> for MediaType {
    fn into(self) -> &'a str {
        match self {
            Self::Mp4 => "mp4",
            Self::Mov => "mov",
            Self::Webm => "webm",
            Self::Mpeg => "mpeg",
            Self::Invalid => "Invalid media type"
        }
    }
}

#[derive(Debug)]
pub enum Errors {
    Reqwest(ReqwestError),
    DotEnv(DotEnvError),
    Deserialize(SerdeError),
    Regex(RegexError),
    EnvVar(VarError),
    Io(IoError),
    NativeTls(NativeTlsError),
    Handshake(HandshakeError<TcpStream>),
    ToStr(ToStrError),
    ParseInt(ParseIntError),
}

impl From<ReqwestError> for Errors {
    fn from(error: ReqwestError) -> Self {
        Self::Reqwest(error)
    }
}

impl From<DotEnvError> for Errors {
    fn from(error: DotEnvError) -> Self {
        Self::DotEnv(error)
    }
}

impl From<VarError> for Errors {
    fn from(error: VarError) -> Self {
        Self::EnvVar(error)
    }
}

impl From<RegexError> for Errors {
    fn from(error: RegexError) -> Self {
        Self::Regex(error)
    }
}

impl From<SerdeError> for Errors {
    fn from(error: SerdeError) -> Self {
        Self::Deserialize(error)
    }
}

impl From<IoError> for Errors {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<NativeTlsError> for Errors {
    fn from(value: NativeTlsError) -> Self {
        Self::NativeTls(value)
    }
}

impl From<HandshakeError<TcpStream>> for Errors {
    fn from(value: HandshakeError<TcpStream>) -> Self {
        Self::Handshake(value)
    }
}

impl From<ToStrError> for Errors {
    fn from(value: ToStrError) -> Self {
        Self::ToStr(value)
    }
}

impl From<ParseIntError> for Errors {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(error) => write!(f, "Unable to send request => {error}"),
            Self::DotEnv(error) => write!(f, "Unable to load environment file => {error}"),
            Self::Deserialize(error) => write!(f, "Could not deserialize bytes => {error}"),
            Self::Regex(error) => write!(f, "Unable to build regular expression => {error}"),
            Self::EnvVar(error) => write!(f, "Could not load environment variable => {error}"),
            Self::Io(error) => write!(f, "Unable to create file due to error => {error}"),
            Self::NativeTls(error) => write!(f, "TLS error => {error}"),
            Self::Handshake(error) => write!(f, "Unable to successfully complete TCP handshake => {error}"),
            Self::ToStr(error) => write!(f, "Unable to convert to str => {error}"),
            Self::ParseInt(error) => write!(f, "Could not parse integer => {error}"),
        }
    }
}

pub async fn archive(post_info: Post) -> Result<(), Errors> {
    let captures = &post_info.post_id_regex.captures(&post_info.info.url);

    // Exit code 100 means no post data could be extracted.
    let post_info_pieces = match captures {
        Some(captures) => captures,
        None => exit(100),
    };
    let mut headers = HeaderMap::new();

    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = ClientBuilder::new()
        .redirect(Policy::limited(100))
        .default_headers(headers)
        .build()?;
    let url = format!("https://web.archive.org/save/{}", &post_info.info.url);

    // Here we will login to Bluesky, get a JWT token, then get the post
    let auth_response = client
        .post("https://bsky.social/xrpc/com.atproto.server.createSession")
        .body(
            json!({
                "identifier": &post_info.info.username,
                "password": &post_info.info.password
            })
            .to_string(),
        )
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();
    let creds = serde_json::from_slice::<BskyCreds>(&auth_response)?;
    let response = &client
        .get(format!(
            "https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle={}",
            &post_info_pieces[1],
        ))
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();
    let did = serde_json::from_slice::<Did>(&response)?;
    let response = client
        .get(format!(
            "https://bsky.social/xrpc/app.bsky.feed.getPostThread?uri=at://{}/app.bsky.feed.post/{}",
            did.did, &post_info_pieces[2]
        ))
        .bearer_auth(&creds.access_jwt)
        .send()
        .await?
        .bytes()
        .await?.to_vec();
    let post_data = serde_json::from_slice::<ThreadData>(&response)?;

    if let Some(post) = post_data.thread.post {
        if let Some(record) = post.record {
            println!("Saving post locally...");

            // Write the post content to a file to preserve its contents locally
            if !&post_info.posts_dir_exists {
                fs::create_dir("./posts")?;
            }
            fs::create_dir(format!("./posts/{}", &post_info_pieces[2]))?;

            let filename = &format!("./posts/{}/raw.json", &post_info_pieces[2]);
            let mut file = File::create_new(filename).await?;

            file.write_all(&response).await?;
            println!("Raw post data archived...Saving associated media...");
            let mut line_counter = 0;

            if let Some(media) = record.embed {
                for image in media.images {
                    let referer = &image.image.referer;
                    let url = format!(
                        "https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                        &did.did, &referer.cid
                    );
                    let response = client.get(&url).send().await?.bytes().await?.to_vec();
                    let mut image_file = File::create(format!(
                        "./posts/{}/{}.png",
                        &post_info_pieces[2], &referer.cid
                    ))
                    .await?;
                    image_file.write(&response).await?;
                    println!("Saved {}", &referer.cid)
                }
                if let Some(video) = media.video {
                    println!("Saving video from post");

                    // Exit code 101 is for no media type being provided in the response
                    let media_type = video.mime_type.as_str();
                    println!("MEDIA TYPE => {media_type}");
                    let media_type: MediaType = MediaType::from(media_type);
                    let media_type: &'static str = media_type.into();
                    let referer = video.referer;
                    let url_path = format!(
                        "/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
                        &did.did, &referer.cid
                    );

                    // Get the response headers for the redirect location to get the blob data
                    let reqwest_response = client
                        .get(format!("https://bsky.social{url_path}"))
                        .bearer_auth(&creds.access_jwt)
                        .send()
                        .await?;
                    let reqwest_response = reqwest_response
                        .bytes()
                        .await?;
                    let reqwest_response = reqwest_response
                        .as_bytes();
                    let mut video_file = File::create(format!(
                            "./posts/{}/{}.{}",
                        &post_info_pieces[2], &referer.cid, media_type
	                )).await?;

                    video_file.write_all(reqwest_response).await?;
                }
            }
        }
    }
    println!("Archiving externally...");
    // client.get(url).send().await?;
    println!("Post archived successfully.");

    Ok(())
}
