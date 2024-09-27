use candid::Principal;
use serde::{Deserialize, Serialize};
use crate::models::types::{CommentId, TimestampMillis};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: CommentId,
    pub content: String,
    pub image: Option<String>,
    pub creator_id: Principal,
    pub created_at: TimestampMillis,
}
