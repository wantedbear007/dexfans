use core::types::{PostStatus, PostVisibility};
use std::borrow::Cow;

use crate::models::types::{Cycles, PostId, TimestampMillis};
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
//     pub post_type: core::types::PostType,
//     pub price: Option<u8>,
// }

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image{
    pub source: String,
    pub need_pay: bool,
    pub price: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct Video{
    pub source: String,
    pub need_pay: bool,
    pub price: Option<u8>,
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
    pub price: Option<u8>, // Has a value only if post_type is Paid
    pub likes: Vec<Principal>,
    pub views: Vec<Principal>,
    pub like_count: usize,
    pub views_count: usize,
    // pub comments: Vec<crate::models::comment::Comment>,
    pub creator_id: Principal,
    pub comments_count: u32,
    pub created_at: TimestampMillis,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct PostPagination {
    pub start: u32,
    pub end: u32,
    pub post_id: PostId,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct CreatePostArgs {
    pub content: String,
    //pub image: Option<String>,
    pub image: Option<Vec<Image>>,
    //pub video: Option<String>,
    pub video: Option<Video>,
    //pub post_type: core::types::PostType,
    pub post_visibility: core::types::PostVisibility,
    pub post_status: core::types::PostStatus,
    pub price: Option<u8>,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct UpdatePostArgs {
    pub id: u128,
    pub content: String,
    //pub image: Option<String>,
    pub image: Option<Vec<Image>>,
    //pub video: Option<String>,
    pub video: Option<Video>,
    //pub post_type: core::types::PostType,
    pub post_visibility: core::types::PostVisibility,
    pub post_status: core::types::PostStatus,
    pub price: Option<u8>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            content: String::from(""),
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
