use chrono::{DateTime, Utc};
use pivotal_tracker_derive::BrandedInt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::{
	blocker::BlockerID,
	branch::BranchID,
	client::{Client, RequestError},
	comment::CommentID,
	cycle_time_details::CycleTimeDetails,
	integration::{IntegrationExternalID, IntegrationID},
	label::Label,
	person::PersonID,
	project::ProjectID,
	pull_request::PullRequestID,
	review::ReviewID,
	story_transition::StoryTransition,
	task::TaskID,
};

const STORY_LINK_BASE: &str = "https://www.pivotaltracker.com/n/projects";
const STORY_TILE_LINK_BASE: &str = "https://www.pivotaltracker.com/story/show";

impl Client {
	pub async fn get_story(
		&self,
		options: GetStoryOptions,
	) -> Result<Story, RequestError> {
		self
			.request::<Story, _>(|client, base_url| {
				client.get(format!("{}/stories/{}", base_url, options.id))
			})
			.await
	}
}

pub struct GetStoryOptions {
	pub id: StoryID,
}

// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#story_resource)
#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
	pub accepted_at: Option<DateTime<Utc>>,

	/// Sum of estimates of accepted stories in a release (for a
	/// release-type story).
	///
	/// This field is read only.
	#[serde(rename = "points_accepted")]
	pub accepted_points: Option<f32>,

	/// Sum of all accepted stories in a release (for a release-type story).
	///
	/// This field is read only.
	#[serde(rename = "counts_accepted")]
	pub accepted_stories_count: Option<u64>,

	/// ID of the story that the current story is located after. `None` if story
	/// is the first one in the project.
	#[serde(rename = "after_id")]
	pub after_story_id: Option<StoryID>,

	/// ID of the story that the current story is located before. `None` if
	/// story is last one in the project.
	#[serde(rename = "before_id")]
	pub before_story_id: Option<StoryID>,

	/// IDs of other stories that are blocked by this story.
	///
	/// This field is read only.
	pub blocked_story_ids: Option<Vec<StoryID>>,
	pub blocker_ids: Option<Vec<BlockerID>>,
	pub branch_ids: Option<Vec<BranchID>>,
	pub comment_ids: Option<Vec<CommentID>>,
	pub created_at: DateTime<Utc>,

	/// All information regarding a story's cycle time and state transitions
	/// (duration and occurrences).
	///
	/// This field is read only.
	pub cycle_time_details: Option<CycleTimeDetails>,

	/// Due date/time (for a release-type story).
	#[serde(rename = "deadline")]
	pub deadline_at: Option<DateTime<Utc>>,

	/// In-depth explanation of the story requirements.
	pub description: Option<String>,

	/// Point value of the story.
	pub estimate: Option<f32>,
	pub follower_ids: Option<Vec<PersonID>>,

	/// This field is read only.
	pub id: StoryID,

	/// The integration's specific ID for the story.
	#[serde(rename = "external_id")]
	pub integration_external_id: Option<IntegrationExternalID>,
	pub integration_id: Option<IntegrationID>,

	/// This field is read only.
	pub kind: String,
	pub labels: Vec<Label>,
	pub name: String,
	pub owner_ids: Vec<PersonID>,

	/// Sum of estimates of all stories in a release (for a release-type story).
	///
	/// This field is read only.
	#[serde(rename = "points_total")]
	pub points_count: Option<f32>,
	pub project_id: ProjectID,
	pub pull_request_ids: Option<Vec<PullRequestID>>,

	/// This field is read only.
	pub projected_completion_at: Option<DateTime<Utc>>,
	pub requested_by_id: PersonID,
	pub review_ids: Option<Vec<ReviewID>>,

	/// Story's state of completion.
	#[serde(rename = "current_state")]
	pub state: StoryState,

	/// Sum of all stories in a release (for a release-type story).
	///
	/// This field is read only.
	#[serde(rename = "counts_total")]
	pub stories_count: Option<u64>,
	pub story_type: StoryType,
	pub task_ids: Option<TaskID>,

	/// All state transitions for the story.
	///
	/// This field is read only.
	pub transitions: Option<Vec<StoryTransition>>,

	/// This field is read only.
	pub updated_at: DateTime<Utc>,
	pub url: url::Url,
}

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct StoryID(pub u64);

impl FromStr for StoryID {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let normalized_str = match s {
			// #181439777
			s if s.starts_with('#') => &s[1..],

			// https://www.pivotaltracker.com/story/show/181439777
			s if s.starts_with(STORY_TILE_LINK_BASE) => {
				let matcher_link_example =
					format!("{}/<story-id>", STORY_TILE_LINK_BASE);
				let matcher =
					Regex::new(&format!(r"^{}/(?P<story_id>\d+)", STORY_TILE_LINK_BASE))
						.expect(&format!(
							"Failed to create regex for story tile link ({})",
							matcher_link_example
						));

				matcher
					.captures(s)
					.expect(&format!(
						"Failed to match story tile link ({}) with {}",
						matcher_link_example, s
					))
					.name("story_id")
					.expect(&format!(
						"Failed to get story ID from story tile link ({}) with {}",
						matcher_link_example, s
					))
					.as_str()
			}

			// 181439777 or
			// https://www.pivotaltracker.com/n/projects/2553178/stories/181439777
			s => {
				let matcher_link_example =
					format!("{}/<project-id>/stories/<story-id>", STORY_LINK_BASE);
				let matcher = Regex::new(&format!(
					r"{}/\d+/stories/(?P<story_id>\d+)",
					STORY_LINK_BASE
				))
				.expect(&format!(
					"Failed to create regex for story link ({})",
					matcher_link_example
				));

				match matcher.captures(s) {
					Some(captures) => captures
						.name("story_id")
						.expect(&format!(
							"Failed to get story ID from story link ({}) with {}",
							matcher_link_example, s
						))
						.as_str(),
					None => s,
				}
			}
		};
		let story_id = normalized_str.parse()?;

		Ok(StoryID(story_id))
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StoryState {
	Accepted,
	Delivered,
	Finished,
	Planned,
	Rejected,
	Started,
	Unscheduled,
	Unstarted,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StoryType {
	Bug,
	Chore,
	Feature,
	Release,
}

impl Display for StoryType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let story_type_str = format!("{:?}", self).to_lowercase();

		write!(f, "{}", story_type_str)
	}
}
