use chrono::{DateTime, Utc};
use pivotal_tracker_derive::BrandedInt;
use serde::{Deserialize, Serialize};

use crate::project::ProjectID;

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#membership_summary_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipSummary {
  /// Whether or not the project is one of the member's favorites.
  #[serde(rename = "favorite")]
  pub is_favorite: bool,

  /// This field is read only.
  pub kind: String,

  /// This field is read only.
  pub id: MembershipSummaryID,

  /// The last (approximate) time at which the authenticated user accessed
  /// the project.
  pub last_viewed_at: DateTime<Utc>,

  /// The color of the project on the member's views.
  pub project_color: String,
  pub project_id: ProjectID,
  pub project_name: String,

  /// The relationship between the authenticated user making the request and
  /// the project.
  pub role: MembershipSummaryRole,
}

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct MembershipSummaryID(pub u64);

/// The relationship between the authenticated user making the request and
/// the project.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MembershipSummaryRole {
  Inactive,
  Member,
  Owner,
  Viewer,
}
