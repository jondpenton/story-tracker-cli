use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  pub email: String,
  pub id: PersonID,
  pub initials: String,
  pub kind: String,
  pub name: String,
  pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonID(u64);
