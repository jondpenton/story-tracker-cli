use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ProjectID;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
  /// Creation time. This field is read only.
  pub created_at: Option<DateTime<Utc>>,

  /// The number of days remaining in the account's Free Trial period, or 0 if
  /// it has expired. This field is read only.
  pub days_left: Option<u8>,

  /// The type of this object: account. This field is read only.
  pub kind: String,

  /// Database id of the account. This field is read only.
  pub id: AccountID,

  /// The name of the account.
  pub name: String,

  /// True if the account is currently over its subscription plan limits. This
  /// field is read only.
  pub over_the_limit: Option<bool>,

  /// The name of the account's current subscription plan. This field is
  /// read only.
  pub plan: AccountPlan,

  /// IDs of the project(s) that are associated with the account. This field
  /// is read only.
  pub project_ids: Option<Vec<ProjectID>>,

  /// This string gives the subscription status of the account that contains
  /// the project. In particular, conditions that can cause the project to be
  /// read-only will be included here. This field is read only.
  pub status: AccountStatus,

  /// Time of last update. This field is read only.
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountID(u64);

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountPlan {
  Free,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
  Active,
  Deleted,
  Delinquent,
  Limited,
  Suspended,
}
