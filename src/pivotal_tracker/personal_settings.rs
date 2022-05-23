use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalSettings {
  /// Controls the state of the header on the project pages.
  pub header_display_mode: HeaderDisplayMode,

  /// This field is read only.
  pub kind: String,

  /// Boolean representing whether or not autorefresh should be enabled
  /// in reports.
  pub reports_autorefresh: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HeaderDisplayMode {
  Collapsed,
  Expanded,
}
