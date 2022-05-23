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
  pub person: Person,

  pub accounts: Vec<Account>,
  pub api_token: String,
  pub created_at: String,
  pub has_google_identity: bool,
  pub projects: Vec<MembershipSummary>,
  pub receives_in_app_notifications: bool,
  pub time_zone: TimeZone,
  pub updated_at: String,
}
