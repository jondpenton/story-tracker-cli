use serde::{Deserialize, Serialize};

use super::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
  email: String,
  name: String,
}

impl Client {
  pub async fn get_me(self) -> reqwest::Result<Me> {
    let client = reqwest::Client::new();
    let res = client
      .get("https://www.pivotaltracker.com/services/v5/me")
      .header("X-TrackerToken", &self.api_key)
      .send()
      .await?;

    res.json::<Me>().await
  }
}
