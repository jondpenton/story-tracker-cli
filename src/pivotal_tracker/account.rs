use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
  pub kind: String,
  pub id: AccountID,
  pub name: String,
  pub plan: AccountPlan,
  pub status: AccountStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountID(u64);

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountPlan {
  Free,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountStatus {
  #[serde(rename = "active")]
  Active,
}
