# picdl-rs
A simple Rust library to download media from various services, such as Gelbooru.

## Installation
```sh
cargo add picdl_rs
```

## Examples
### Gelbooru
Fetch one post by tags
```rust
#[tokio::main]
async fn main() {
    let client = picdl_rs::gelbooru::Gelbooru::<picdl_rs::http::reqwest::ReqwestClient>::new();
    let post = client.fetch("omori rating:general", 1, 1, 0).await.unwrap().post[0].clone(); // "omori rating:general" is the tags to get posts by. Meta-tags are supported. Second argument is the count of posts to get (limit), third is page number and the last is change id, you can keep it 0. 
    println!("Tags: {}", post.tags);
    println!("File URL: {}", post.file_url);
}
```

### Boosty
Fetch one post from blog
```rust
#[tokio::main]
async fn main() {
    let client = picdl_rs::boosty::Boosty::<picdl_rs::http::reqwest::ReqwestClient>::new();
    let post = client.fetch("boosty", 1).await.unwrap(); // Here, "boosty" is the name of a blog to get posts from, and 1 is count of posts to get (limit)
    println!("Post title: {}", post.data[0].title);
}
```
