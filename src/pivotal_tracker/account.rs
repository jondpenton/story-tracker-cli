use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
  kind: String,
  id: AccountID,
  name: String,
  plan: AccountPlan,
  status: AccountStatus,
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
