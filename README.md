# picdl-rs
A simple Rust library to download media from various services, such as Gelbooru.

## Examples
### Fetching one post by tags
```rust
#[tokio::main]
async fn main() {
    let client = picdl_rs::gelbooru::Gelbooru::<picdl_rs::http::reqwest::ReqwestClient>::new();
    let post = client.fetch("omori rating:general", 1, 1, 0).await.unwrap().post[0].clone();
    println!("Tags: {}", post.tags);
    println!("File URL: {}", post.file_url);
}
```
