use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationExternalID(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationID(pub u64);
