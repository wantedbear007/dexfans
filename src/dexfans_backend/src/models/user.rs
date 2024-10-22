use std::borrow::Cow;
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use crate::models::types::{CanisterId, PostId, TimestampMillis, Membership};


#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserInputArgs {
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub cover_image: Option<String>,
    // pub asset_canister_id: 
}

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserProfile {
    pub user_id: Principal,
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub asset_canister_id: Principal,
    pub cover_image: Option<String>,
    pub subscribers: Vec<Principal>,  // Subscribers for the user
    pub subscribing: Vec<Principal>,  // Users this user is subscribing to
    pub posts: Vec<PostId>,            // Created posts
    pub likes: Vec<PostId>,            // Liked posts
    pub collects: Vec<PostId>,         // Collected posts
    // pub canister_id: CanisterId,       // Canister for handling user's data
    // pub cycles: Vec<CyclesTopup>,      // List of cycles top-ups
    // pub token_amount: u128,            // Amount of tokens the user owns
    pub is_bot: bool,                  // Is this user a bot?
    pub membership: Membership,        // Membership level
    pub created_at: TimestampMillis,   // Timestamp when the user was created
}

impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: crate::utils::constants::STORABLE_USER_MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
