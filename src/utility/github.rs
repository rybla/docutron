use anyhow::{Result, anyhow};
use octocrab::Octocrab;
use url::Url;

// ----------------------------------------------------------------------------
// ValidGithubRepoUrl
// ----------------------------------------------------------------------------

pub struct ValidGithubRepoUrl {
    pub url: Url,
    pub owner: String,
    pub name: String,
    pub readme_url: String,
}

impl ValidGithubRepoUrl {
    pub fn parse(url: &Url) -> Result<Option<Self>> {
        let mut segments = url
            .path_segments()
            .ok_or_else(|| anyhow!("Invalid URL: cannot get path segments"))?;
        let owner = segments
            .next()
            .ok_or_else(|| anyhow!("Invalid URL: missing owner"))?;
        let repo_name = segments
            .next()
            .ok_or_else(|| anyhow!("Invalid URL: missing repo name"))?;
        Ok(Some(Self {
            url: url.clone(),
            owner: owner.to_string(),
            name: repo_name.to_string(),
            readme_url: format!(
                "https://raw.githubusercontent.com/{owner}/{repo_name}/main/README.md"
            ),
        }))
    }

    pub async fn fetch(&self, octocrab: &Octocrab) -> Result<RepoInfo> {
        fetch_repo_info(octocrab, &self.url).await
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct RepoInfo {
    pub owner: String,
    pub name: String,
    pub readme: Option<String>,
}

pub async fn fetch_repo_info(octocrab: &Octocrab, url: &Url) -> Result<RepoInfo> {
    let mut segments = url
        .path_segments()
        .ok_or_else(|| anyhow!("Invalid URL: cannot get path segments"))?;
    let owner = segments
        .next()
        .ok_or_else(|| anyhow!("Invalid URL: missing owner"))?;
    let repo_name = segments
        .next()
        .ok_or_else(|| anyhow!("Invalid URL: missing repo name"))?;
    let readme = octocrab.repos(owner, repo_name).get_readme().send().await?;

    Ok(RepoInfo {
        owner: owner.to_owned(),
        name: repo_name.to_owned(),
        readme: readme.decoded_content(),
    })
}
