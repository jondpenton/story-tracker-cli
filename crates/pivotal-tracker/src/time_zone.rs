use serde::{Deserialize, Serialize};

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#time_zone_resource)
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZone {
	/// This field is read only.
	pub kind: String,

	/// The offset, from UTC, of the time zone. This is a string containing a
	/// formatted representation of the time zone offset. First, and optional +
	/// or - sign (no sign is equivalent to '+'), then a number of hours, a
	/// colon, and a number of minutes. Only valid, internationally-recognized
	/// time zone offsets should be used when sending zone information for the
	/// client. For example, "-01:03" and "+23:00" are not valid values, even
	/// though they match the encoding pattern.
	pub offset: String,

	/// The Olson name for the time zone.
	pub olson_name: String,
}
