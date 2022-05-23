use serde::{Deserialize, Serialize};

use super::ProjectID;

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipSummary {
  pub favorite: bool,
  pub kind: String,
  pub id: MembershipSummaryID,
  pub last_viewed_at: String,
  pub project_color: String,
  pub project_id: ProjectID,
  pub project_name: String,
  pub role: MembershipSummaryRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipSummaryID(u64);

#[derive(Debug, Serialize, Deserialize)]
pub enum MembershipSummaryRole {
  #[serde(rename = "owner")]
  Owner,
}
