use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
	account::{Account, AccountID},
	client::{Client, RequestError},
	membership_summary::MembershipSummary,
	person::Person,
	personal_settings::PersonalSettings,
	project::ProjectID,
	time_zone::TimeZone,
	workspace::WorkspaceID,
};

impl Client {
	pub async fn get_me(&self) -> Result<Me, RequestError> {
		self
			.request::<Me, _>(|client, base_url| {
				client.get(format!("{}/me", base_url))
			})
			.await
	}
}

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#me_resource)
#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
	#[serde(flatten)]
	pub person: Person,

	/// This field is read only.
	pub account_ids: Option<Vec<AccountID>>,
	pub accounts: Vec<Account>,

	/// A string that can be used as the API authentication token
	/// (`X-TrackerToken`) to authenticate future API requests as being on
	/// behalf of the current user.
	///
	/// This field is read only.
	pub api_token: String,
	pub created_at: DateTime<Utc>,

	/// `true` if the authenticated user's profile is associated with a Google
	/// Email identity and the current request is authenticated with that
	/// identity.
	///
	/// This field is read only.
	pub has_google_identity: bool,
	pub personal_settings: Option<PersonalSettings>,

	/// This field is read only.
	pub project_ids: Option<Vec<ProjectID>>,
	pub projects: Vec<MembershipSummary>,

	/// This field is read only.
	pub receives_in_app_notifications: bool,

	/// This field is read only.
	pub time_zone: TimeZone,
	pub updated_at: DateTime<Utc>,

	/// This field is read only.
	pub workspace_ids: Option<Vec<WorkspaceID>>,
}
