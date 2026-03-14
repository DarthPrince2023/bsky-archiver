use tokio::{fs::File, io::AsyncWriteExt};

use crate::lib::{errors::Errors, extension::reqwest_client::ClientBuilder};

pub mod reqwest_client;

pub async fn save_media(
    media_type: Option<&str>,
    did: &str,
    cid: &str,
    client: &mut ClientBuilder,
    post_id: &str
) -> Result<(), Errors> {
    let media_type = media_type.unwrap_or("png");

    // Get the response headers for the redirect location to get the blob data
    let reqwest_response = client
        .set_url(format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={did}&cid={cid}"))
        .send_request()
        .await?;
    let mut file = File::create(format!("./posts/{post_id}/{cid}.{media_type}")).await?;

    file.write_all(&reqwest_response).await?;

    Ok(())
}
