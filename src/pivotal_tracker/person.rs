use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  pub email: String,
  pub id: u64,
  pub initials: String,
  pub kind: String,
  pub name: String,
  pub username: String,
}
