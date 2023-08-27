use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{person::PersonID, project::ProjectID, story::StoryID};

// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#story_transition_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct StoryTransition {
	/// This field is read only.
	#[serde(rename = "performed_by_id")]
	pub actor_person_id: PersonID,

	/// This field is read only.
	pub kind: String,

	/// This field is read only.
	pub occurred_at: DateTime<Utc>,

	/// This field is read only.
	pub project_id: ProjectID,

	/// The activity version of the story transition.
	///
	/// This field is read only.
	pub project_version: u64,

	/// This field is read only.
	pub state: StoryTransitionState,

	/// This field is read only.
	pub story_id: StoryID,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StoryTransitionState {
	Accepted,
	Delivered,
	Finished,
	Planned,
	Rejected,
	Started,
	Unscheduled,
	Unstarted,
}
