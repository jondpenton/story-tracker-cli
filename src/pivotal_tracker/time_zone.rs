use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZone {
  // The type of this object: time_zone. This field is read only.
  pub kind: String,

  // The offset, from UTC, of the time zone. This is a string containing a formatted representation of the time zone offset. First, and optional + or - sign (no sign is equivalent to '+'), then a number of hours, a colon, and a number of minutes. Only valid, internationally-recognized time zone offsets should be used when sending zone information for the client. For example, "-01:03" and "+23:00" are not valid values, even though they match the encoding pattern.
  pub offset: String,

  // The Olson name for the time zone.
  pub olson_name: String,
}
