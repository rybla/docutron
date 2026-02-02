use anyhow::Result;

/// Fetch the bookmark urls via [`crate::utility::url_list::read_bookmarks_url_list`], then insert those urls via [`crate::db::models::NewDocument`] into the `documents` table in the database. Only initialize these fields:
/// - `url`
/// - `added_date`
pub async fn fetch_bookmarks() -> Result<()> {
    unimplemented!()
}
