use anyhow::Result;
use docutron::flow::{
    fetch_bookmarks::fetch_bookmarks, fetch_hackernews::fetch_hackernews,
    fetch_saveds::fetch_saveds, update_all::update_all,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv()?;

    // fetches
    fetch_bookmarks().await?;
    fetch_hackernews().await?;
    fetch_saveds().await?;

    // updates
    update_all().await?;

    Ok(())
}
