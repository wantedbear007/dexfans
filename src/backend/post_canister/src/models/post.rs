use core::types::{PostStatus, PostVisibility};
use std::borrow::Cow;

use crate::models::types::PostId;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    pub source: core::types::MediaID,
    pub need_pay: bool,
    pub price: Option<core::types::ICPAmount>,
}

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct Video {
    pub source: core::types::MediaID,
    pub need_pay: bool,
    pub price: Option<core::types::ICPAmount>,
}

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub post_id: PostId,
    pub content: String,
    //pub image: Option<String>,
    pub image: Option<Vec<Image>>,
    //pub video: Option<String>,
    pub video: Option<Video>,
    //pub post_type: core::types::PostType,
    pub post_visibility: PostVisibility,
    pub post_status: PostStatus,
    pub price: Option<core::types::ICPAmount>, // Has a value only if post_type is Paid
    pub likes: Vec<Principal>,
    pub views: Vec<Principal>,
    pub like_count: core::types::Counters,
    pub views_count: core::types::Counters,
    // pub comments: Vec<crate::models::comment::Comment>,
    pub creator_id: Principal,
    pub comments_count: core::types::Counters,
    pub created_at: core::types::TimestampMillis,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct PostPagination {
    pub start: core::types::Counters,
    pub end: core::types::Counters,
    pub post_id: PostId,
}

#[derive(Deserialize, CandidType, Clone, Validate)]
pub struct CreatePostArgs {
    #[validate(length(min = 3, max = core::constants::VALIDATOR_POST_CONTENT_SIZE, message = "Post content is too large"))]
    pub content: String,
    //pub image: Option<String>,
    pub image: Option<Vec<Image>>,
    //pub video: Option<String>,
    pub video: Option<Video>,
    //pub post_type: core::types::PostType,
    pub post_visibility: core::types::PostVisibility,
    pub post_status: core::types::PostStatus,
    pub price: Option<core::types::ICPAmount>,
}

#[derive(Deserialize, CandidType, Clone, Validate)]
pub struct UpdatePostArgs {
    #[validate(length(min = 3, max = core::constants::VALIDATOR_POST_CONTENT_SIZE, message = "Post content is too large"))]
    pub content: String,
    pub id: core::types::PostId,
    //pub image: Option<String>,
    pub image: Option<Vec<Image>>,
    //pub video: Option<String>,
    pub video: Option<Video>,
    //pub post_type: core::types::PostType,
    pub post_visibility: core::types::PostVisibility,
    pub post_status: core::types::PostStatus,
    pub price: Option<core::types::ICPAmount>,
}

#[derive(CandidType, Deserialize)]
pub struct GetByPostStatusArgs {
    pub status: core::types::PostStatus,
    pub pagination: core::types::PaginationArgs0,
    // pub status: core::types::PostStatus,
    // pub ids: Vec<core::types::PostId>,
}



impl Default for Post {
    fn default() -> Self {
        Self {
            content: String::new(),
            image: None,
            video: None,
            //post_type: core::types::PostType::Free,
            post_visibility: core::types::PostVisibility::Everyone,
            post_status: core::types::PostStatus::Published,
            price: None,
            // comments: Vec::new(),
            comments_count: 0,
            created_at: ic_cdk::api::time(),
            creator_id: Principal::anonymous(),
            likes: Vec::new(),
            post_id: 0,
            views: Vec::new(),
            like_count: 0,
            views_count: 0,
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
        ciborium::into_writer(self, &mut buf).expect(core::constants::ERROR_ENCODE_FAILED);
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        ciborium::from_reader(&bytes[..]).expect(core::constants::ERROR_DECODE_FAILED)
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}
