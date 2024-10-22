use crate::models::types::{Membership, PostId, TimestampMillis};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub post_canister_id: Principal,
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
    pub membership: Membership,      // Membership level
    pub created_at: TimestampMillis, // Timestamp when the user was created
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
    // fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
    //     Cow::Owned(Encode!(self).unwrap())
    // }

    // fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
    //     Decode!(bytes.as_ref(), Self).unwrap()
    // }

    // const BOUND: Bound = Bound::Bounded {
    //     max_size: crate::utils::constants::STORABLE_USER_MAX_VALUE_SIZE,
    //     is_fixed_size: false,
    // };
}
