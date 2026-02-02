use anyhow::{Result, anyhow};

use crate::utility::{self, common::Env};

/// Fetches the content at a URL and extracts the text content.
pub async fn fetch_text_content(env: &mut Env, url: &str) -> Result<String> {
    if let Some(arxiv_id) = utility::arxiv::get_id_from_url(url) {
        unimplemented!("extract abstract")
    } else if let Ok(github_repo) = utility::github::fetch_repo_info(&env.octocrab, url).await {
        unimplemented!("extract README")
    } else if let Ok(x_post) = utility::x::fetch_post(url).await {
        unimplemented!("extract content")
    } else {
        // fetch content at URL
        let response = env.client.get(url).send().await?;
        let headers = response.headers();

        let content_type = match headers.get("content-type") {
            None => {
                return Result::Err(anyhow!(
                    "I failed to get the content type, since the response does not have a header for content-type: {response:?}"
                ));
            }
            Some(content_type) => {
                let bytes = content_type.as_bytes();
                let str = String::from_utf8_lossy(bytes);
                str.to_string()
            }
        };

        // extract content
        #[allow(clippy::single_match)]
        match content_type.as_str() {
            "text/pdf" => {
                unimplemented!("extract text content from PDF")
            }
            content_type if content_type.starts_with("text/html") => {
                unimplemented!("use Readability to extract text content from HTML")
            }
            // TODO: handle other types of content
            _ => {
                return Err(anyhow!("unrecognized content type: {content_type}"));
            }
        }
    }
}
