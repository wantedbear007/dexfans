use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};


pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = u128;
pub type TimestampMillis = u64;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PostType {
    Free,
    Silver,
    Gold,
    Platinum,
    Paid,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub enum Membership {
    Guest,
    Silver,
    Gold,
    Platinum,
}
