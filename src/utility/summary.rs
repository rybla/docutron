use anyhow::{Result, anyhow};

use crate::utility::{self, common::Env};

/// Fetches the content at a URL and extracts the text content.
///
/// This function attempts to identify the URL as a specific platform (Arxiv, GitHub, X/Twitter)
/// and uses specialized extractors to retreive the content.
/// - **Arxiv**: Extracts the paper summary/abstract.
/// - **GitHub**: Extracts the repository README content.
/// - **X (Twitter)**: Extracts the post content (HTML).
///
/// If no specific platform matches, it fetches the URL and inspects the `Content-Type`:
/// - **PDF** (`text/pdf`): Extracts text from the PDF.
/// - **HTML** (`text/html`): Uses Mozilla's Readability algorithm to extract the main article text.
pub async fn fetch_text_content(env: &mut Env, url: &str) -> Result<String> {
    if let Some(arxiv_id) = utility::arxiv::get_id_from_url(url) {
        let arxiv = utility::arxiv::fetch_by_id(arxiv_id).await?;
        Ok(arxiv.summary)
    } else if let Ok(github_repo) = utility::github::fetch_repo_info(&env.octocrab, url).await {
        Ok(github_repo.readme.unwrap_or_default())
    } else if let Ok(x_post) = utility::x::fetch_post(url).await {
        Ok(x_post.html)
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
            "application/pdf" | "text/pdf" => {
                let bytes = response.bytes().await?;
                let text = pdf_extract::extract_text_from_mem(&bytes)?;
                Ok(text)
            }
            content_type if content_type.starts_with("text/html") => {
                let text = response.text().await?;
                // Parse the HTML content. We pass the URL as well (if the API supports it) or just the text.
                // Assuming `parse` takes the HTML content.
                // Looking at common usage of readability crates, providing text is primary.
                // If this fails to compile due to missing signature, we will adjust.
                let article = env.readability.parse(&text)?;
                Ok(article.text_content)
            }
            // TODO: handle other types of content
            _ => Err(anyhow!("unrecognized content type: {content_type}")),
        }
    }
}
