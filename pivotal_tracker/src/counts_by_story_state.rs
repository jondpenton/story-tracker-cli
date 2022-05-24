use serde::{Deserialize, Serialize};

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#counts_by_story_state_resource)
#[derive(Debug, Serialize, Deserialize)]
pub struct CountsByStoryState {
  /// Accepted stories in the Current iteration.
  pub accepted: u64,
  pub delivered: u64,
  pub finished: u64,

  /// This field is read only.
  pub kind: String,
  pub planned: u64,
  pub rejected: u64,
  pub started: u64,
  pub unscheduled: u64,
  pub unstarted: u64,
}
