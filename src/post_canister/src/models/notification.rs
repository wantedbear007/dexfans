use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::types::TimestampMillis;

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub recipient: Principal,
    pub message: String,
    pub created_at: TimestampMillis,
}