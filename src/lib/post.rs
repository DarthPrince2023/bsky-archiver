use crate::PostInformation;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Post {
    pub info: PostInformation,
    pub posts_dir_exists: bool,
    pub post_id_regex: Regex,
}
