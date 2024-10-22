use std::borrow::Cow;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = u128;
pub type TimestampMillis = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterInitArgs {
    pub canister_ids: std::collections::HashMap<String, candid::Principal>,
    pub controllers: std::collections::HashSet<Principal>,
    // more to be added later
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub controllers: std::collections::HashSet<Principal>,
    pub canister_ids: std::collections::HashMap<u8, candid::Principal>,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    // more to be added later
}

impl ic_stable_structures::Storable for CanisterMetaData {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::into_writer(self, &mut buf).expect(dexfans_types::constants::ERROR_ENCODE_FAILED);
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        ciborium::from_reader(&bytes[..]).expect(dexfans_types::constants::ERROR_DECODE_FAILED)
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
