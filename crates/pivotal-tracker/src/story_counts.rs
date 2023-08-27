use serde::{Deserialize, Serialize};

use crate::counts_by_story_state::CountsByStoryState;

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#story_counts_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct StoryCounts {
	/// This field is read only.
	pub kind: String,

	/// Total number of stories in each story state.
	#[serde(rename = "number_of_stories_by_state")]
	pub story_counts_by_state: CountsByStoryState,

	/// Total point estimates for each story state.
	#[serde(rename = "sum_of_story_estimates_by_state")]
	pub story_estimate_total_by_state: CountsByStoryState,

	/// How many stories in each story state have no points.
	#[serde(rename = "number_of_zero_point_stories_by_state")]
	pub zero_point_story_counts_by_state: CountsByStoryState,
}
