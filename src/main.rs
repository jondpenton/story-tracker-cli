mod pivotal_tracker;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = pivotal_tracker::Client::new(pivotal_tracker::ClientNewOptions {
    api_key: env::var("PIVOTAL_TRACKER_API_KEY")?,
    api_version: 5,
  });
  let me = client.get_me().await?;

  println!("{:?}", me);

  Ok(())
}
