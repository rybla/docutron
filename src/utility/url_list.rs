use crate::config;
use anyhow::{Context, Result};
use std::fs;

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
