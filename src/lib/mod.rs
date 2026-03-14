pub mod errors;
pub mod post;
pub mod post_information;
pub mod types;
pub mod extension;

use reqwest::{redirect::Policy, ClientBuilder as ReqwestClientBuilder};
use serde_json::json;
use std::fs::create_dir;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::lib::{errors::Errors, extension::{reqwest_client::{ClientBuilder, RequestType}, save_media}, post::Post, types::{BskyCreds, Did, MediaType, ThreadData}};

pub async fn archive(post_info: Post) -> Result<(), Errors> {
    let captures = &post_info.post_id_regex.captures(&post_info.info.url);
    let post_info_pieces = captures.as_ref().expect("Invalid post URL");
    let handle = &post_info_pieces[1];
    let post_id = &post_info_pieces[2];
    let client = ReqwestClientBuilder::new()
        .redirect(Policy::limited(100))
        .build()?;
    let auth_payload = json!({
        "identifier": &post_info.info.username,
        "password": &post_info.info.password
    }).to_string();
    let mut client = ClientBuilder::new(Some(client), RequestType::Post, "https://bsky.social/xrpc/com.atproto.server.createSession".into(), None, Some(auth_payload.as_bytes().into()), None, None);
    let url = format!("https://web.archive.org/save/{}", &post_info.info.url);

    // Here we will login to Bluesky, get a JWT token, then get the post
    let auth_response = client.send_request().await?;
    let creds = serde_json::from_slice::<BskyCreds>(&auth_response)?;
    let response = &client
        .set_request_type(RequestType::Get)
        .set_url(format!("https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle={handle}"))
        .send_request()
        .await?;
    let did = serde_json::from_slice::<Did>(&response)?.did;
    let response = &client
        .set_url(format!("https://bsky.social/xrpc/app.bsky.feed.getPostThread?uri=at://{did}/app.bsky.feed.post/{post_id}"))
        .set_bearer_auth(Some(creds.access_jwt))
        .send_request()
        .await?;
    let post_data = serde_json::from_slice::<ThreadData>(&response)?;

    if let Some(post) = post_data.thread.post {
        if let Some(record) = post.record {
            println!("Saving post locally...");

            // Write the post content to a file to preserve its contents locally
            if !&post_info.posts_dir_exists {
                create_dir("./posts")?;
            }
            create_dir(format!("./posts/{post_id}"))?;

            let filename = &format!("./posts/{post_id}/raw.json");
            let mut file = File::create_new(filename).await?;

            file.write_all(&response).await?;
            println!("Raw post data archived...Saving associated media...");

            if let Some(media) = record.embed {
                for image in media.images {
                    let cid = image.image.referer.resource_cid;
                    let _ = save_media(None, &did, &cid, &mut client, post_id.into()).await?;

                    println!("Saved {cid}");
                }
                if let Some(video) = media.video {
                    let media_type = video.mime_type.as_str();
                    let media_type: MediaType = MediaType::from(media_type);
                    let media_type: &'static str = media_type.into();
                    let cid = video.referer.resource_cid;
                    let _ = save_media(Some(media_type), &did, &cid, &mut client, post_id.into()).await?;

                    println!("Saved video")
                }
            }
        }
    }

    let _ = client
        .set_url(url)
        .send_request()
        .await?;

    Ok(())
}
