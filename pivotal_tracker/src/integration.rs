use pivotal_tracker_derive::{Branded, BrandedInt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Branded)]
pub struct IntegrationExternalID(pub String);

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct IntegrationID(pub u64);
