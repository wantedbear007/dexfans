// use candid::Principal;
// use serde::{Deserialize, Serialize};
// use crate::models::types::{CommentId, TimestampMillis};


// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Comment {
//     pub comment_id: CommentId,
//     pub content: String,
//     pub image: Option<String>,
//     pub creator_id: Principal,
//     pub created_at: TimestampMillis,
// }


use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use crate::models::types::{CommentId, TimestampMillis};


#[derive(Serialize, Deserialize, Debug, Clone,CandidType)]
pub struct Comment {
    pub comment_id: CommentId,
    pub content: String,
    pub image: Option<String>,
    pub creator_id: Principal,
    pub created_at: TimestampMillis,
}


impl Storable for Comment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Comment).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}