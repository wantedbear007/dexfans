
use candid::{CandidType, Decode, Encode, Principal};

use serde::{Deserialize, Serialize};

pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = String;
pub type TimestampMillis = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, CandidType)]
pub enum PostType {
    Free,
    Silver,
    Gold,
    Platinum,
    Paid,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, CandidType)]
pub enum Membership {
    Guest,
    Silver,
    Gold,
    Platinum,
}


#[derive(Serialize, Deserialize)]
pub struct Pagination {
  pub page: usize,
  pub page_size: usize
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: Membership,
}
