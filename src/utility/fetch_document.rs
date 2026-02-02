use crate::{
    db::models::Document,
    utility::{self, common::Env},
};
use anyhow::{Result, anyhow};
use url::Url;

/// Fetches the content at a URL and extracts a [`Document`].
pub async fn fetch_document(env: &mut Env, url: &Url) -> Result<Document> {
    if let Some(valid_arxiv_url) = utility::arxiv::ValidArxivUrl::parse(url)? {
        let arxiv = valid_arxiv_url.fetch().await?;
        Ok(arxiv.summary)
    } else if let Some(valid_github_repo_url) = utility::github::ValidGithubRepoUrl::parse(url)? {
        let github_repo = valid_github_repo_url.fetch(&env.octocrab).await?;
        let readme = github_repo
            .readme
            .ok_or_else(|| anyhow!("No README found for this repository"))?;
        Ok(readme)
    } else if let Some(valid_x_url) = utility::x::ValidXUrl::parse(url)? {
        let x_post = valid_x_url.fetch().await?;
        let article = env.readability.parse(&x_post.html)?;
        Ok(article.text_content)
    } else {
        // fetch content at URL
        let response = env.client.get(url.as_str()).send().await?;
        let headers = response.headers();

        let content_type = match headers.get("content-type") {
            None => {
                return Result::Err(anyhow!(
                    "Failed to get the content type, since the response does not have a header for content-type: {response:?}"
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
            _ => Err(anyhow!("Unrecognized content type: {content_type}")),
        }
    }
}
