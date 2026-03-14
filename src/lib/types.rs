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
