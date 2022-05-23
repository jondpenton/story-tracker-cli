use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  email: String,
  id: u64,
  initials: String,
  kind: String,
  name: String,
  username: String,
}
