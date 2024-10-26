use candid::{CandidType, Principal};

use serde::{Deserialize, Serialize};

pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;

pub type PostId = u128;
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
    pub page_size: usize,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct UpdateMembershipIC {
    pub user: Principal,
    pub membership: Membership,
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

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserProfileArgsIC {
    pub user_id: Principal,
    pub username: String,
    // pub membership: Membership,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct SubscribeAccountIC {
    pub subscribed_to: Principal,
    pub subscribed_by: Principal,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UnsubscribeAccountIC {
    pub unsubscribed_to: Principal,
    pub unsubscribed_by: Principal,
}


#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct NotificationBody {
    pub category: NotificationType,
    pub description: Option<String>,
    pub title: String,
    pub created_on: TimestampMillis,
    pub expiring_on: TimestampMillis,
    pub by: Option<Principal>,
}

#[derive(Clone, CandidType, Serialize, Copy, Deserialize)]
pub enum NotificationType {
    NewPost,
    NewComment,
    NewSubscriber,
    NewLike,
    NewSubscribingPost,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct LikeNotificationArgs {
    pub post_url: String,
    pub post_owner: Principal
}