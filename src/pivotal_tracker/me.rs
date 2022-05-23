use super::{Account, Client, Person, RequestError, TimeZone};
use serde::{Deserialize, Serialize};

impl Client {
  pub async fn get_me(&self) -> Result<Me, RequestError> {
    self
      .request::<Me, _>(|client, base_url| {
        client.get(format!("{}/me", base_url))
      })
      .await
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
  #[serde(flatten)]
  person: Person,

  accounts: Vec<Account>,
  api_token: String,
  created_at: String,
  has_google_identity: bool,
  // personal_settings: PersonalSettings,
  // project_ids: Vec<u64>,
  receives_in_app_notifications: bool,
  time_zone: TimeZone,
  updated_at: String,
  // workspace_ids: Vec<u64>,
}
