use serde::{Deserialize, Serialize};

// TODO: Verify file structure is correct.
#[derive(Serialize, Deserialize, Debug)]
pub enum HeaderDisplayMode {
  Collapsed,
  Expanded,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalSettings {
  // Controls the state of the header on the project pages.
  pub header_display_mode: HeaderDisplayMode,

  // The type of this object: personal_settings. This field is read only.
  pub kind: String,

  // Boolean representing whether or not autorefresh should be enabled in reports.
  pub reports_autorefresh: bool,
}
