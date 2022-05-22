use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let res = client
    .get("https://www.pivotaltracker.com/services/v5/me")
    .header("X-TrackerToken", &env::var("PIVOTAL_TRACKER_API_KEY")?)
    .send()
    .await?;
  let body = res.json::<Me>().await?;

  println!("{:?}", body);

  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Me {
  email: String,
  name: String,
}
