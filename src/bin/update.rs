use anyhow::Result;
use docutron::flow::{
    fetch_bookmarks::fetch_bookmarks, fetch_hackernews::fetch_hackernews,
    fetch_saveds::fetch_saveds, update_all::update_all,
};

#[tokio::main]
async fn main() -> Result<()> {
    // fetches
    fetch_bookmarks().await?;
    fetch_hackernews().await?;
    fetch_saveds().await?;

    // updates
    update_all().await?;

    Ok(())
}
