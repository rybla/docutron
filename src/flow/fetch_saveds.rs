use anyhow::Result;

/// Fetch the saveds urls via [`crate::utility::url_list::read_saveds_url_list`], then insert those urls via [`crate::db::models::NewDocument`] into the `documents` table in the database. Only initialize the `url` field.
///
/// If an insertion fails, a warning is logged and the process continues.
pub async fn fetch_saveds() -> Result<()> {
    log::trace!("fetch_saveds");
    unimplemented!()
}
