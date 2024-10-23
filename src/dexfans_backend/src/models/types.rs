use std::borrow::Cow;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = u128;
pub type TimestampMillis = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct DexFansCanisterInitArgs {
    pub canister_ids: std::collections::HashMap<String, candid::Principal>,
    pub controllers: std::collections::HashSet<Principal>,
    pub payment_recipient: candid::Principal, // more to be added later
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub controllers: std::collections::HashSet<Principal>,
    pub canister_ids: std::collections::HashMap<u8, candid::Principal>,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    pub payment_recipient: candid::Principal, // more to be added later
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserInputArgs {
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub cover_image: Option<String>,
    // pub asset_canister_id:
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub(crate) struct UserProfile {
    pub user_id: Principal,
    pub active_post_canister: Principal,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub asset_canister_id: Principal,
    pub cover_image: Option<String>,
    pub subscribers: Vec<Principal>, // Subscribers for the user
    pub subscribing: Vec<Principal>, // Users this user is subscribing to
    pub posts: Vec<PostId>,          // Created posts
    pub likes: Vec<PostId>,          // Liked posts
    pub collects: Vec<PostId>,       // Collected posts
    pub is_bot: bool,                // Is this user a bot?
    pub membership: dexfans_types::types::Membership, // Membership level
    pub created_at: TimestampMillis, // Timestamp when the user was created
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserProfileInterCanister {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: dexfans_types::types::Membership,
}

impl Default for UserProfileInterCanister {
    fn default() -> Self {
        Self {
            user_id: candid::Principal::anonymous(),
            username: String::from("NA"),
            posts: Vec::new(),
            likes: Vec::new(),
            collects: Vec::new(),
            membership: dexfans_types::types::Membership::Gold,
        }
    }
}

impl ic_stable_structures::Storable for UserProfileInterCanister {
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

impl ic_stable_structures::Storable for UserProfile {
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
