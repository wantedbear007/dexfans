use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = String;
pub type TimestampMillis = u64;
// pub struct Error(String);

// impl Default for Error {
//     fn default() -> Self {
//         Error(String::from("None"))
//     }
// }

// impl Error {
//     fn created() -> Self {
//         Error(String::from("Create operation was success"))
//     }

//     fn failed() -> Self {
//         Error(String::from("Operation failed"))
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    // pub asset_canister
    }

