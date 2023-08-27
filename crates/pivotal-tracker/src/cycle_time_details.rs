use serde::{Deserialize, Serialize};

use crate::story::StoryID;

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#cycle_time_details_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct CycleTimeDetails {
	/// The total amount of time, in milliseconds, between when the story was
	/// first started to when it was last accepted, excluding weekends. In the
	/// case where a story has not been accepted, it is the time between when
	/// the story was first started to the current time. If the story has not
	/// been started, this property is not returned.
	///
	/// This field is read only.
	#[serde(rename = "total_cycle_time")]
	pub cycle_time_total: u64,

	/// The number of times that the story has been in the delivered state.
	///
	/// This field is read only.
	pub delivered_count: u64,

	/// The total amount of time, in milliseconds, that the story was in the
	/// delivered state, excluding weekends.
	///
	/// This field is read only.
	#[serde(rename = "delivered_time")]
	pub delivered_duration: u64,

	/// The number of times that the story has been in the finished state.
	///
	/// This field is read only.
	pub finished_count: u64,

	/// The total amount of time, in milliseconds, that the story was in the
	/// finished state, excluding weekends.
	///
	/// This field is read only.
	#[serde(rename = "finished_time")]
	pub finished_duration: u64,

	/// This field is read only.
	pub kind: String,

	/// This field is read only.
	pub story_id: StoryID,

	/// The number of times that the story has been in the rejected state.
	///
	/// This field is read only.
	pub rejected_count: u64,

	/// The total amount of time, in milliseconds, that the story was in the
	/// rejected state, excluding weekends.
	///
	/// This field is read only.
	#[serde(rename = "rejected_time")]
	pub rejected_duration: u64,

	/// The number of times that the story has been in the started state.
	///
	/// This field is read only.
	pub started_count: u64,

	/// The total amount of time, in milliseconds, that the story was in the
	/// started state, excluding weekends.
	///
	/// This field is read only.
	#[serde(rename = "started_time")]
	pub started_duration: u64,
}
