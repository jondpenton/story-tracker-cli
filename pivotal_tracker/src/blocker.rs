use pivotal_tracker_derive::BrandedInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BrandedInt)]
pub struct BlockerID(pub u64);
