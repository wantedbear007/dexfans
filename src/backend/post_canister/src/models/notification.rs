use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub recipient: Principal,
    pub message: String,
    pub created_at: core::types::TimestampMillis,
}
