use anyhow::Result;
use url::Url;

// ----------------------------------------------------------------------------
// ValidXUrl
// ----------------------------------------------------------------------------

pub struct ValidXUrl {
    pub url: Url,
}

impl ValidXUrl {
    pub fn parse(url: &Url) -> Result<Option<Self>> {
        if url.domain() == Some("x.com") || url.domain() == Some("twitter.com") {
            Ok(Some(Self { url: url.clone() }))
        } else {
            Ok(None)
        }
    }

    pub async fn fetch(&self) -> Result<Post> {
        fetch_post(&self.url).await
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Post {
    pub url: String,
    pub author_name: String,
    pub author_url: String,
    pub html: String,
}

pub async fn fetch_post(post_url: &Url) -> Result<Post> {
    let post_url = urlencoding::encode(post_url.as_str());
    let result = reqwest::get(format!("https://publish.twitter.com/oembed?url={post_url}")).await?;
    let post = result.json::<Post>().await?;
    Ok(post)
}
