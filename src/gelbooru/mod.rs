extern crate pretty_env_logger;
pub mod error;

use error::GelbooruError;
use serde::{Deserialize, Serialize};
use super::http::{client::{Headers, HttpClient, Query}, reqwest::ReqwestClient};


/// Gelbooru client struct
///
/// # Example
///
/// Using an async http client (reqwest)
/// ```no_test
/// let client = Gelbooru {
///     http: picdl_rs::http::reqwest::ReqwestClient {}
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Gelbooru<Http: HttpClient> {
    http: Http,
}

/// Post rating, like `Explicit` is totally NSFW,
/// `Sensitive` is near NSFW and `General` is SFW
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    General,
    Sensitive,
    Explicit,
    Questionable
}

/// Gelbooru post struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    /// Post ID
    id: i64,
    /// Post creation date, for example "Sat Dec 14 04:00:44 -0600 2024"
    created_at: String,
    /// Post score
    score: i64,
    /// Width of image in `file_url`
    width: i64,
    /// Height of image in `file_url`
    height: i64,
    /// MD5 hash of post
    md5: String,
    /// Gelbooru directory where image is stored
    directory: String,
    /// Image filename 
    image: String,
    /// Image rating
    rating: Rating,
    /// Image source (link to original post, author)
    source: String,
    /// Post edit UNIX date
    change: Option<i64>,
    /// Post owner on Gelbooru
    owner: String,
    /// Post creator ID on Gelbooru
    creator_id: i64,
    /// Post parent ID, 0 if no parent post
    parent_id: i64,
    /// Does this post have sample image
    sample: i64,
    /// Preview image height
    preview_height: i64,
    /// Preview image width
    preview_width: i64,
    /// Post tags
    tags: String,
    /// Post title
    title: String,
    /// Does the post has notes (true/false but in String)
    has_notes: String,
    /// Does the post has comments (true/false but in String)
    has_comments: String,
    /// Original, full-size image url
    file_url: String,
    /// Preview url
    preview_url: String,
    /// Sample url
    sample_url: String,
    /// Sample image height
    sample_height: i64,
    /// Sample image width
    sample_width: i64,
    /// Post status
    status: String,
    /// Is the post locked
    post_locked: i64,
    /// Does the post has children
    has_children: String
}

/// Gelbooru post attributes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attributes {
    /// Count of posts on this page (in this response)
    limit: i64,
    /// Post offset
    offset: i64,
    /// Count of posts to return
    count: i64
}

/// Gelbooru response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// Response attributes, like `limit`, `offset` and `count`
    #[serde(rename(deserialize = "@attributes"))]
    attributes: Attributes,
    /// Vector of `Post`
    post: Vec<Post>,
}

/// Gelbooru API client
impl<Http: HttpClient> Gelbooru<Http>
    where GelbooruError: From<<Http as HttpClient>::Error> {
    /// Returns a Gelbooru client with ReqwestClient as HTTP client
    pub fn new() -> Gelbooru<ReqwestClient> {
        Gelbooru { http: <ReqwestClient as Default>::default() }
    }
    
    /// Fetches post(s) from Gelbooru
    ///
    /// # Arguments
    /// `tags` - tags to find post(s) by, meta tags such as `rating` are allowed 
    /// `limit` - how much posts to return, from 1 to 100
    /// `pid` - page number
    /// `cid` - change ID of the post
    /// If not sure, set `limit` to 100 and `cid` to 0
    pub async fn fetch(&self, tags: &str, limit: i8, pid: i64, cid: i64) -> Result<Response, GelbooruError> {
        let headers = Headers::new();
        let payload = Query::new();
        let response = self.http.get(
            format!("https://gelbooru.com/index.php?page=dapi&s=post&q=index&tags={tags}&limit={limit}&pid={pid}&cid={cid}&json=1"),
            &headers,
            &payload).await?;

        let json: Response = serde_json::from_str(&response)?;
        Ok(json)
    }

    /// Fetches post(s) from Gelbooru just by tags and page
    ///
    /// # Arguments
    /// `tags` - tags to find post(s) by, meta tags such as `rating` are allowed 
    /// `pid` - page number
    pub async fn fetch_all_posts(&self, tags: &str, pid: i64) -> Result<Response, GelbooruError> {
        self.fetch(tags, 100, pid, 0).await
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::gelbooru::Gelbooru;
    use crate::http::reqwest::ReqwestClient;

    #[tokio::test]
    async fn test_gelbooru_fetch() {
        pretty_env_logger::init();
        let client = Gelbooru::<ReqwestClient>::new();
        let result = client.fetch("omori", 1, 1, 0);
        debug!("{:?}", result.await);
    }
}
