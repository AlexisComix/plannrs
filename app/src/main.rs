use lib;
use anyhow;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // I could maybe use async to make a spinny for loading
    println!("Finding Database...");
    let db = lib::db::create_or_get_handle().await?;
    Ok(())
}
