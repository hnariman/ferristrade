use news::{Feeds, FeedsError};

#[tokio::main]
async fn main() -> Result<(), FeedsError> {
    println!("news basic example");
    let feed = Feeds::default();
    feed.update_and_show().await;
    let _ = tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    Ok(())
}
