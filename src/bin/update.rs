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

    let mut conn = docutron::db::establish_connection();

    // fetches
    fetch_bookmarks(&mut conn).await?;
    fetch_hackernews(&mut conn).await?;
    fetch_saveds(&mut conn).await?;

    // updates
    update_all(&mut conn).await?;

    Ok(())
}
