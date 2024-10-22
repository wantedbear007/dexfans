use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use serde::{Deserialize, Serialize};


pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = u128;
pub type TimestampMillis = u64;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterInitArgs {
    pub asset_canister: Principal,
    pub controllers: std::collections::HashSet<Principal>,
    pub post_canister: Principal,
    
    // more to be added later
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub asset_canister: Principal,
    pub controllers: std::collections::HashSet<Principal>,
    pub post_canister: Principal,
    pub all_post_canisters: std::collections::HashSet<Principal>
    // more to be added later
}


impl ic_stable_structures::Storable for CanisterMetaData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}


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
