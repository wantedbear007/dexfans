use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use crate::models::types::{PostId, TimestampMillis, PostType, CommentId, Cycles};


#[derive(Serialize, Deserialize, Debug, Clone, CandidType, PartialEq)]
pub struct CyclesTopup {
    pub date: TimestampMillis,
    pub amount: Cycles,
}


#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct CreatePostArgs {
    pub content: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub post_type: PostType,
    pub price: Option<u8>,
}





#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct Post {
    pub post_id: PostId,
    pub content: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub post_type: PostType,
    pub price: Option<u8>,  // Has a value only if post_type is Paid
    pub likes: Vec<Principal>,
    pub views: Vec<Principal>,
    pub comments: Vec<CommentId>,
    pub creator_id: Principal,
    pub created_at: TimestampMillis,
}


impl Storable for Post {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}