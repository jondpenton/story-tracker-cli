use pivotal_tracker_derive::BrandedInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationExternalID(pub String);

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct IntegrationID(pub u64);
