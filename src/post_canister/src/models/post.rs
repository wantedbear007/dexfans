use std::borrow::Cow;

use crate::models::types::{CommentId, Cycles, PostId, TimestampMillis};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, CandidType, PartialEq)]
pub struct CyclesTopup {
    pub date: TimestampMillis,
    pub amount: Cycles,
}

// #[derive(Serialize, Deserialize, Clone, CandidType)]
// pub struct CreatePostArgs {
//     pub content: String,
//     pub image: Option<String>,
//     pub video: Option<String>,
//     pub post_type: dexfans_types::types::PostType,
//     pub price: Option<u8>,
// }

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct Post {
    pub post_id: PostId,
    pub content: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub post_type: dexfans_types::types::PostType,
    pub price: Option<u8>, // Has a value only if post_type is Paid
    pub likes: Vec<Principal>,
    pub views: Vec<Principal>,
    pub comments: Vec<CommentId>,
    pub creator_id: Principal,
    pub created_at: TimestampMillis,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct CreatePostArgs {
    pub content: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub post_type: dexfans_types::types::PostType,
    pub price: Option<u8>,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct UpdatePostArgs {
    pub id: u128,
    pub content: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub post_type: dexfans_types::types::PostType,
    pub price: Option<u8>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            content: String::from(""),
            image: None,
            video: None,
            post_type: dexfans_types::types::PostType::Free,
            price: None,
            comments: Vec::new(),
            created_at: ic_cdk::api::time(),
            creator_id: Principal::anonymous(),
            likes: Vec::new(),
            post_id: 0,
            views: Vec::new(),
        }
    }
}

// impl Storable for Post {
//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: ic_stable_structures::storable::Bound =
//         ic_stable_structures::storable::Bound::Unbounded;
// }

impl ic_stable_structures::Storable for Post {
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
