use candid::{CandidType, Principal};

use serde::{Deserialize, Serialize};

pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = u128;
pub type TimestampMillis = u64;
pub type PostPrice = u32;
pub type ImageVideoId = u128;
pub type Response = Result<(), String>;
// pub type MediaID = u32;
pub type MediaID = String;
pub type Counters = usize;
pub type CaptchaKey = u32;
pub type ICPAmount = candid::Nat;
pub type UserID = candid::Principal;
pub type Milliseconds = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, CandidType, Eq, PartialOrd, Ord)]
pub enum PostType {
    Free,
    PaidPost,
    PaidImgs,
    Diamond,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, CandidType, PartialOrd)]
pub enum Membership {
    Guest = 0,
    Diamond = 1,
}

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostVisibility {
    Everyone,
    DiamondUser,
}

#[derive(Serialize, Deserialize, Clone, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostStatus {
    Published,
    Archived,
    Draft,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct UpdateMembershipIC {
    pub user: Principal,
    pub membership: Membership,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct PurchaseUserMedia {
    pub owner: Principal,
    pub amt: ICPAmount,
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: Principal,
    pub username: String,
    // pub posts: Vec<PostId>,
    // pub likes: Vec<PostId>,
    // pub collects: Vec<PostId>,
    pub membership: Membership,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct UserDetailsMinified {
    pub user_id: UserID,
    pub username: String,
    pub avatar: Option<String>,
    pub cover: Option<String>,
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
    pub created_on: TimestampMillis,
    pub expiring_on: TimestampMillis,
    pub by: Option<UserDetailsMinified>,
    pub post_brief: Option<String>,
    pub comment_content: Option<String>,
    pub post_id: Option<PostId>
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
    // pub post_url: String,
    pub post_owner: UserID,
    // pub username: String,
    pub post_brief: String,
    pub post_id: PostId

}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct CommentNotificationArgs {
    // pub post_url: String,
    pub post_owner: UserID,
    pub post_brief: Option<String>,
    pub comment_content: String,
    pub post_id: PostId
    // pub username: String,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct PostCanisterInitArgs {
    pub accounts: Vec<UserProfile>,
    pub canister_ids: std::collections::HashMap<String, candid::Principal>,
    pub controllers: std::collections::HashSet<Principal>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct ICAddPostCanisterProfile {
    pub post_canister: candid::Principal,
    pub caller: UserID,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Pagination {
    pub start: Counters,
    pub end: Counters,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub post_id: PostId,
    pub asset_canister: candid::Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct SinglePurchaseArgs {
    pub post_id: PostId,
    pub media_id: MediaID,
    pub created_by: UserID,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct PostPurchaseArgs {
    pub post_id: PostId,
    pub created_by: UserID,
}
