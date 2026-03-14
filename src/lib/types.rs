use serde::{Deserialize, Serialize};

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

// pub async fn archive(url: String, username: String, password: String) -> Result<(), Errors> {
//     let post_id = Regex::new(r"profile/([a-zA-Z0-9._-]+)/post/([A-Za-z0-9._:~-]+)")
//         .map_err(|error| Errors::RegexError(error))?;
//     let captures = &post_id.captures(&url);
//     let post_info_pieces = match captures {
//         Some(captures) => captures,
//         None => exit(69)
//     };
//     let mut headers = HeaderMap::new();
//         headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
//         headers.insert("Content-Type", HeaderValue::from_static("application/json"));
//     let client = ClientBuilder::new()
//         .redirect(Policy::limited(100))
//         .default_headers(headers)
//         .build()
//         .map_err(|error| Errors::ReqwestSendError(error))?;
//     let url = format!("https://web.archive.org/save/{}", &url);

//     // Here we will login to Bluesky, get a JWT token, then get the post
//     let auth_request = client
//         .post("https://bsky.social/xrpc/com.atproto.server.createSession")
//         .body(json!({
//             "identifier": username,
//             "password": password
//         })
//         .to_string())
//         .send()
//         .await
//         .map_err(|error| Errors::ReqwestSendError(error))?
//         .bytes()
//         .await
//         .map_err(|error| Errors::ReqwestBytesError(error))?
//         .to_vec();
//     let creds = serde_json::from_slice::<BskyCreds>(&auth_request).map_err(|error| Errors::DeserializeError(error))?;
//     let request = client
//         .get(format!("https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle={}", &post_info_pieces[1]))
//         .send()
//         .await
//         .map_err(|error| Errors::ReqwestSendError(error))?
//         .bytes()
//         .await
//         .map_err(|error| Errors::ReqwestBytesError(error))?
//         .to_vec();
//     let did = serde_json::from_slice::<Did>(&request)
//         .map_err(|error| Errors::DeserializeError(error))?;
//     let post_bytes = client
//         .get(format!("https://bsky.social/xrpc/app.bsky.feed.getPostThread?uri=at://{}/app.bsky.feed.post/{}", did.did, &post_info_pieces[2]))
//         .bearer_auth(creds.access_jwt)
//         .send()
//         .await
//         .map_err(|error| Errors::ReqwestSendError(error))?
//         .bytes()
//         .await
//         .map_err(|error| Errors::ReqwestBytesError(error))?
//         .to_vec();
//     let post_data = serde_json::from_slice::<ThreadData>(&post_bytes)
//         .map_err(|error| Errors::DeserializeError(error))?;
//     if let Some(post) = post_data
//         .thread
//         .post {
//         if let Some(record) = post
//             .record {
//             // Write the post content to a file to preserve its contents locally
//             let _ = fs::create_dir(format!("./posts/{}", &post_info_pieces[2]));
//             let mut file = File::create(&format!("./posts/{}/raw.json", &post_info_pieces[2]))
//                 .map_err(|error| Errors::FileCreateError(error))?;
//             let _ = file.write(&post_bytes);
//             println!("Raw post data archived...Saving associated media...");
//             if let Some(media) = record
//                 .embed {
//                 for image in media.images {
//                     let image = match &image.image {
//                         Some(image) => image,
//                         None => break,  
//                     };
//                     let referer = match &image.referer {
//                         Some(referer) => referer,
//                         None => break,
//                     let post_id = match Regex::new(r"profile/([a-zA-Z0-9._-]+)/post/([A-Za-z0-9._:~-]+)")
//                         .map_err(|error| Errors::RegexError(error)) {
//                             Ok(post_id) => post_id,
//                             Err(error) => {
//                                 panic!("Unable to extract post ID from URL provided => {error}");
//                             }
//                         };
//                     let captures = &post_id.captures(&url);
//                     let post_info_pieces = match captures {
//                         Some(captures) => captures,
//                         None => exit(69)
// };
//                     let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
//                     let blob = client
//                         .get(&url)
//                     let mut headers = HeaderMap::new();
//                         headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
//                         headers.insert("Content-Type", HeaderValue::from_static("application/json"));
//                     let client = match ClientBuilder::new()
//                         .redirect(Policy::limited(100))
//                         .default_headers(headers)
//                         .build() {
//                             Ok(client) => client,
//                             Err(error) =>
//                                 panic!("Unable to build client => {error}")
//                         };
//                     let url = format!("https://web.archive.org/save/{}", &url);

//                     // Here we will login to Bluesky, get a JWT token, then get the post
//                     let auth_response = match client
//                         .post("https://bsky.social/xrpc/com.atproto.server.createSession")
//                         .body(json!({
//                             "identifier": username,
//                             "password": password
//                         })
//                         .to_string())
// .send()
// .await
//                         .map_err(|error| Errors::ReqwestSendError(error))?
//                         .map_err(|error| Errors::ReqwestSendError(error)) {
//                             Ok(response) => response,
//                             Err(error) =>
//                                 panic!("Failed to send request to service => {error}"),
//                         };
//                     let response_bytes = match auth_response
// .bytes()
//                         .await
//                         .map_err(|error| Errors::ReqwestBytesError(error))?
//                         .await {
//                             Ok(response_bytes) => response_bytes,
//                             Err(error) =>
//                                 panic!("Failed to get bytes from response => {error}"),
//                         }
// .to_vec();
//                     let mut image_file = File::create(format!("./posts/{}/{}.png", &post_info_pieces[2], &referer.resource_cid))
//                         .map_err(|error| Errors::FileCreateError(error))?;
//                     let _ = image_file.write(&blob);
//                     println!("Saved {}", &referer.resource_cid)
//                 }

//                 if let Some(video) = media.video {
//                     println!("Saving video from post");
//                     let referer = match video.referer {
//                         Some(referer) => referer,
//                         None => exit(71)
//                     let creds = match serde_json::from_slice::<BskyCreds>(&response_bytes) {
//                         Ok(creds) => creds,
//                         Err(error) =>
//                             panic!("Failed to deserialize received bytes => {error}")
// };
//                     let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
//                     let blob = client
//                         .get(&url)
//                     let response = match client
//                         .get(format!("https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle={}", &post_info_pieces[1]))
// .send()
//                         .await
//                         .map_err(|error| Errors::ReqwestSendError(error))?;
//                     let blob = match blob
//                         .await {
//                             Ok(response) => response,
//                             Err(error) =>
//                                 panic!("Failed to send request to host => {error}")
//                         };
//                     let bytes = match response
// .bytes()
// .await {
//                         Ok(bytes) => bytes,
//                         Err(_) => panic!("Unable to get response bytes")
//                     }.to_vec();
//                     let mut image_file = File::create(format!("./posts/{}/{}.mp4", &post_info_pieces[2], &referer.resource_cid))
//                         .map_err(|error| Errors::FileCreateError(error))?;
//                     let _ = image_file.write(&blob);
//                 }
//             }
//         }
//     }
//     println!("Archiving externally...");
              
//     let _ = client
//         .get(url)
//         .send()
//         .await;              
//                             Ok(bytes) => bytes,
//                             Err(error) =>
//                                 panic!("Failed to get bytes from response => {error}")
//                         }
//                         .to_vec();
//                     let did = match serde_json::from_slice::<Did>(&bytes) {
//                         Ok(did) => did,
//                         Err(error) =>
//                             panic!("Failed to deserialize received bytes => {error}")
//                     };
//                     let response = match client
//                         .get(format!("https://bsky.social/xrpc/app.bsky.feed.getPostThread?uri=at://{}/app.bsky.feed.post/{}", did.did, &post_info_pieces[2]))
//                         .bearer_auth(creds.access_jwt)
//                         .send()
//                         .await {
//                             Ok(response) => response,
//                             Err(error) =>
//                                 panic!("Failed to send request to host => {error}")
//                         };
//                     let response_bytes = match response
//                         .bytes() 
//                         .await {
//                             Ok(bytes) => bytes,
//                             Err(error) =>
//                                 panic!("Could not get bytes from response => {error}")
//                         }
//                         .to_vec();
//                     let post_data = match serde_json::from_slice::<ThreadData>(&response_bytes) {
//                         Ok(post_data) => post_data,
//                         Err(error) =>
//                             panic!("Failed to deserialize received bytes => {error}")
//                     };
//                     if let Some(post) = post_data
//                         .thread
//                         .post {
//                         if let Some(record) = post
//                             .record {
//                             println!("Saving post locally...");

//                             // Write the post content to a file to preserve its contents locally
//                             if !posts_dir_exists {
//                                 let _ = fs::create_dir("./posts");
//                             }
//                             let _ = fs::create_dir(format!("./posts/{}", &post_info_pieces[2]));
//                             let filename = &format!("./posts/{}/raw.json", &post_info_pieces[2]);
//                             let mut file = match File::create_new(filename) {
//                                 Ok(file) => file,
//                                 Err(error) =>
//                                     panic!("Could not create archive file => {error}")
//                             };
//                             let _ = file.write(&response_bytes);
//                             println!("Raw post data archived...Saving associated media...");
//                             if let Some(media) = record
//                                 .embed {
//                                 for image in media.images {
//                                     let image = match &image.image {
//                                         Some(image) => image,
//                                         None => break,  
//                                     };
//                                     let referer = match &image.referer {
//                                         Some(referer) => referer,
//                                         None => break,
//                                     };
//                                     let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
//                                     let response = match client
//                                         .get(&url)
//                                         .send()
//                                         .await {
//                                             Ok(response) => response,
//                                             Err(error) =>
//                                                 panic!("Failed to send request to host => {error}")
//                                         };
//                                     let blob = match response
//                                         .bytes()
//                                         .await {
//                                             Ok(bytes) => bytes,
//                                             Err(error) =>
//                                                 panic!("Failed to get bytes from response => {error}")
//                                         }
//                                         .to_vec();
//                                     let mut image_file = match File::create(format!("./posts/{}/{}.png", &post_info_pieces[2], &referer.resource_cid)) {
//                                         Ok(file) => file,
//                                         Err(error) =>
//                                             panic!("Unable to create image file => {error}")
//                                     };
//                                     let _ = image_file.write(&blob);
//                                     println!("Saved {}", &referer.resource_cid)
//                                 }

//                                 if let Some(video) = media.video {
//                                     println!("Saving video from post");
//                                     let referer = match video.referer {
//                                         Some(referer) => referer,
//                                         None => exit(71)
//                                     };
//                                     let url = format!("https://bsky.social/xrpc/com.atproto.sync.getBlob?did={}&cid={}", &did.did, &referer.resource_cid);
//                                     let response = match client
//                                         .get(&url)
//                                         .send()
//                                         .await {
//                                             Ok(response) => response,
//                                             Err(error) =>
//                                                 panic!("Failed to send request to host => {error}")
//                                         };
//                                     let blob = match response
//                                         .bytes()
//                                         .await {
//                                         Ok(bytes) => bytes,
//                                         Err(error) => panic!("Unable to get response bytes => {error}")
//                                     }.to_vec();
//                                     let mut video_file = match File::create(format!("./posts/{}/{}.mp4", &post_info_pieces[2], &referer.resource_cid)) {
//                                         Ok(file) => file,
//                                         Err(error) =>
//                                             panic!("Failed to create video file => {error}")
//                                     };
//                                     let _ = video_file.write(&blob);
//                                 }
//                             }
//                         }
//                     }
//                     println!("Archiving externally...");
                
//                     let _ = client
//                         .get(url)
//                         .send()
//                         .await;              

//     println!("Post archived successfully.");
//                     println!("Post archived successfully.");

//     Ok(())
//                 });
//             }
//             Ok(())
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Did {
    pub did: String
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
    pub type_of: String,
    #[serde(rename = "ref")]
    pub referer: ImageRef,
    pub mime_type: String,
    pub size: u64
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordEmbedMedia {
    pub alt: Option<String>,
    pub aspect_ratio: AspectRatio,
    pub image: MediaBlobData
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
