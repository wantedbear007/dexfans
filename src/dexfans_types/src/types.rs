
use candid::{CandidType, Decode, Encode, Principal};

use serde::{Deserialize, Serialize};

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