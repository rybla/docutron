use anyhow::Result;
use diesel::SqliteConnection;

const HACKERNEWS_RSS_FEED_URL: &'static str = "https://hnrss.org/best";

lazy_static::lazy_static! {
    static ref KEYWORDS: Vec<String> = [
        "programming languages",
        "type theory",
        "type system",
        "haskell",
        "AI",
        "developer tools",
        "video game development",
        "functional programming",
        "dev tools",
        "rust",
        "purescript",
        "compilers",
        "developer experience",
        "category theory",
        "liquid haskell",
        "monad",
        "metaprogramming",
        "mac mini",
        "logic programming",
        "effect systems for purely functional programming langauges",
        "typescript",
        "ocaml",
        "rust",
        "purescript",
        "compiler",
        "mcp",
        "prediction market",
        "homotopy"
        ].into_iter().map(|s| s.to_owned()).collect();
    static ref TOPICS: Vec<String> = [
        "haskell",
        "functional",
        "google",
        "software",
        "korea",
        "japan",
        "singapore",
        "palantir",
        "math",
        "meta",
        "gwern",
        "type",
        "lang",
        "syntax",
        "semantics",
        "github"
        ].into_iter().map(|s| s.to_owned()).collect();
}

/// Fetches the top stories from Hacker News, via the "best" RSS feed: https://hnrss.org/best. Then inserts those urls into the `documents` table in the database via [`crate::db::queries::add_document`]. Indicates that the source is "hackernews/best" using [`crate::db::models::NewDocumentBuilder::source`]
///
/// If an insertion fails, a warning is logged and the process continues.
pub async fn fetch_hackernews(conn: &mut SqliteConnection) -> Result<()> {
    log::trace!("fetch_hackernews");

    let content = reqwest::get(HACKERNEWS_RSS_FEED_URL).await?.bytes().await?;
    let channel = rss::Channel::read_from(std::io::Cursor::new(content))?;

    for item in channel.items {
        if let Some(url) = item.link {
            if let Ok(Some(_)) = crate::db::queries::get_document_by_url(conn, &url) {
                continue;
            }

            let new_doc = crate::db::models::NewDocumentBuilder::new(url.clone())
                .source("hackernews/best")
                .build();

            if let Err(e) = crate::db::queries::add_document(conn, new_doc) {
                log::warn!("Failed to insert hackernews story '{url}': {e}");
            }
        }
    }

    Ok(())
}
