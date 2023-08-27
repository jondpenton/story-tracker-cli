use chrono::{DateTime, Utc};
use pivotal_tracker_derive::BrandedInt;
use serde::{Deserialize, Serialize};

use crate::{project::ProjectID, story_counts::StoryCounts};

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#label_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
	/// This field is read only.
	pub created_at: DateTime<Utc>,

	/// This field is read only.
	pub kind: String,

	/// This field is read only.
	pub id: LabelID,
	pub name: String,

	/// This field is read only.
	pub project_id: ProjectID,

	/// Summary of numbers of stories and points contained.
	///
	/// This field is read only.
	#[serde(rename = "counts")]
	pub story_counts: Option<StoryCounts>,

	/// This field is read only.
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct LabelID(pub u64);
