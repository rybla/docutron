use anyhow::Result;

/// Fetch the bookmark urls via [`crate::utility::url_list::read_bookmarks_url_list`], then insert those urls via [`crate::db::models::NewDocument`] into the `documents` table in the database. Only initialize the `url` and `bookmark_count` fields.
///
/// If an insertion fails, a warning is logged and the process continues.
pub async fn fetch_bookmarks() -> Result<()> {
    log::trace!("fetch_bookmarks");
    let bookmarks = crate::utility::url_list::read_bookmarks_url_list()?;
    let mut conn = crate::db::establish_connection();

    for url in bookmarks {
        let new_doc = crate::db::models::NewDocumentBuilder::new(url.clone())
            .bookmarked(true)
            .build();

        if let Err(e) = crate::db::queries::add_document(&mut conn, new_doc) {
            log::warn!("Failed to insert bookmark '{url}': {e}");
        }
    }

    Ok(())
}
