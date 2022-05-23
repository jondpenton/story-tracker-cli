use super::{
  Account, Client, MembershipSummary, Person, RequestError, TimeZone,
};
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
  pub projects: Vec<MembershipSummary>,
  receives_in_app_notifications: bool,
  time_zone: TimeZone,
  updated_at: String,
}
