use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use crate::models::types::{PostId, TimestampMillis, PostType, CommentId, Cycles};


#[derive(Serialize, Deserialize, Debug, Clone, CandidType, PartialEq)]
pub struct CyclesTopup {
    pub date: TimestampMillis,
    pub amount: Cycles,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
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
