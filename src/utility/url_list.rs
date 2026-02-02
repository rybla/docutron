use crate::config;
use anyhow::{Context, Result};
use std::fs;

use url::Url;

/// Clean up a URL by removing query parameters, fragments, etc.
pub fn clean_url(url: &str) -> String {
    log::trace!("clean_url: {}", url);
    match Url::parse(url) {
        Ok(mut parsed_url) => {
            parsed_url.set_query(None);
            parsed_url.set_fragment(None);
            parsed_url.to_string()
        }
        Err(e) => {
            log::warn!("Failed to parse URL '{}': {}", url, e);
            url.to_string()
        }
    }
}

/// Reads the file at `config::BOOKMARKS_URL_LIST_FILEPATH`, which is a newline-separated list of URLs.
/// Returns a vector of strings, where each string is a URL.
pub fn read_bookmarks_url_list() -> Result<Vec<String>> {
    log::trace!("read_bookmarks_url_list");
    let content = fs::read_to_string(&*config::BOOKMARKS_URL_LIST_FILEPATH).with_context(|| {
        format!(
            "Failed to read bookmarks file at: {}",
            *config::BOOKMARKS_URL_LIST_FILEPATH
        )
    })?;
    Ok(content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

/// Reads the file at `config::SAVEDS_URL_LIST_FILEPATH`, which is a newline-separated list of URLs.
/// Returns a vector of strings, where each string is a URL.
pub fn read_saved_url_list() -> Result<Vec<String>> {
    log::trace!("read_saved_url_list");
    let content = fs::read_to_string(&*config::SAVEDS_URL_LIST_FILEPATH).with_context(|| {
        format!(
            "Failed to read saved urls file at: {}",
            *config::SAVEDS_URL_LIST_FILEPATH
        )
    })?;
    Ok(content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}
