use std::{env, error};

use pivotal_tracker::client::{Client, ClientNewOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let client = Client::new(ClientNewOptions {
    api_key: env::var("PIVOTAL_TRACKER_API_KEY")?,
    api_version: 5,
  });
  let me = client.get_me().await?;

  println!("{:?}", me.person);

  Ok(())
}
