use anyhow::Result;
use diesel::SqliteConnection;

/// Fetch the saved urls, then insert those urls into the `documents` table in the database.
///
/// If an insertion fails, a warning is logged and the process continues.
pub async fn fetch_saveds(conn: &mut SqliteConnection) -> Result<()> {
    log::trace!("fetch_saveds");
    let saveds = crate::utility::url_list::read_saveds_url_list()?;

    for url in saveds {
        let new_doc = crate::db::models::NewDocumentBuilder::new(url.clone()).build();

        if let Err(e) = crate::db::queries::add_document(conn, new_doc) {
            log::warn!("Failed to insert saved '{url}': {e}");
        }
    }

    Ok(())
}
