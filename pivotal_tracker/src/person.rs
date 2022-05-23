extern crate proc_macro;

use serde::{Deserialize, Serialize};

/// [Pivotal Tracker API](https://www.pivotaltracker.com/help/api/rest/v5#person_resource)
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub email: String,

    /// This field is read only.
    pub id: PersonID,
    pub initials: String,

    /// This field is read only.
    pub kind: String,

    /// The full name of the person.
    pub name: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonID(u64);
