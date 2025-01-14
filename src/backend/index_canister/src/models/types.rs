use std::borrow::Cow;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type PostId = u128;

pub type TimestampMillis = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, CandidType)]
pub struct DexFansCanisterInitArgs {
    pub canister_ids: std::collections::HashMap<String, candid::Principal>,
    pub controllers: std::collections::HashSet<Principal>,
    pub payment_recipient: candid::Principal,
    pub membership_plans:
        std::collections::HashMap<core::types::Membership, core::types::ICPAmount>,
    pub active_post_canister: candid::Principal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub controllers: std::collections::HashSet<Principal>,
    pub canister_ids: std::collections::HashMap<u8, candid::Principal>,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    pub payment_recipient: candid::Principal,
    pub membership_plans:
        std::collections::HashMap<core::types::Membership, core::types::ICPAmount>,
    pub active_post_canister: candid::Principal,
    // pub active_asset_canister : candid::Principal
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize, Validate)]
pub(crate) struct UserInputArgs {
    #[validate(length(min = 3,max = core::constants::VALIDATOR_USERNAME_LENGTH, message = "Username size not allowed"))]
    pub username: String,
    #[validate(length(min = 3, max = core::constants::VALIDATOR_BIO_LENGTH, message = "BIO size is not applicable"))]
    pub bio: Option<String>,
    #[validate(length(max = 10, message = "Invalid avatar length"))]
    pub avatar: Option<String>,
    #[validate(length(max = 10, message = "Invalid cover image length"))]
    pub cover_image: Option<String>,
    #[validate(length(min = 3, max = 6, message = "Invalid Captha size"))]
    pub captcha_solution: String, // pub asset_canister_id:
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub(crate) struct UserProfile {
    pub user_id: Principal,
    pub active_post_canister: Principal,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub asset_canister_id: Principal,
    pub cover_image: Option<String>,
    pub subscribers: std::collections::HashSet<Principal>, // Subscribers for the user
    pub subscribing: std::collections::HashSet<Principal>, // Users this user is subscribing to
    // pub posts: Vec<PostId>,                                // Created posts
    pub likes: Vec<PostId>,                     // Liked posts
    pub collects: Vec<core::types::Collection>, // Collected posts
    pub is_bot: bool,                           // Is this user a bot?
    pub membership: core::types::Membership,    // Membership level
    pub created_at: TimestampMillis,            // Timestamp when the user was created
    pub membership_till: u64,
    pub membership_ledger_block:
        std::collections::HashSet<icrc_ledger_types::icrc1::transfer::BlockIndex>,
    pub token_amount: candid::Nat,
}

#[derive(Clone, CandidType)]
pub(crate) struct UserProfileLittleMinified {
    pub user_id: Principal,
    pub active_post_canister: Principal,
    pub all_post_canisters: std::collections::HashSet<Principal>,
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub token_amount: candid::Nat,
    pub asset_canister_id: Principal,
    pub cover_image: Option<String>,
    pub subscribers: std::collections::HashSet<Principal>, // Subscribers for the user
    pub subscribing: std::collections::HashSet<Principal>, // Users this user is subscribing to
    pub membership: core::types::Membership,               // Membership level
    pub created_at: TimestampMillis,                       // Timestamp when the user was created
                                                           // pub membership_till: u64,
                                                           // pub membership_ledger_block: Option<icrc_ledger_types::icrc1::transfer::BlockIndex>,
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserProfileInterCanister {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: core::types::Membership,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PurchasedPosts {
    pub posts: Vec<PurchasePostBody>,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PurchasePostBody {
    pub post_id: u128,
    pub ledger_block: icrc_ledger_types::icrc1::transfer::BlockIndex,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PurchasedMedia {
    pub medias: Vec<PurchaseMediaBody>,
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub(crate) struct PurchaseMediaBody {
    pub post_id: core::types::MediaID,
    pub ledger_block: icrc_ledger_types::icrc1::transfer::BlockIndex,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub(crate) struct Notification {
    pub acc: candid::Principal,
    pub notifications: Vec<core::types::NotificationBody>,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            notifications: Vec::new(),
            acc: ic_cdk::api::caller(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub(crate) struct NotifySubscribersArgs {
    pub title_of_post: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct CaptchaSolution {
    pub created_by: candid::Principal,
    pub created_at: core::types::TimestampMillis,
    pub data: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub struct Captchas {
    pub max: u8, // max captcha to store before deleting
    pub all: Vec<CaptchaSolution>,
}


impl Default for UserProfileInterCanister {
    fn default() -> Self {
        Self {
            user_id: candid::Principal::anonymous(),
            username: String::new(),
            posts: Vec::new(),
            likes: Vec::new(),
            collects: Vec::new(),
            membership: core::types::Membership::Guest,
        }
    }
}

impl ic_stable_structures::Storable for Captchas {
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

impl ic_stable_structures::Storable for Notification {
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

impl ic_stable_structures::Storable for UserProfileInterCanister {
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

impl ic_stable_structures::Storable for UserProfile {
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

impl ic_stable_structures::Storable for CanisterMetaData {
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

impl ic_stable_structures::Storable for PurchasedPosts {
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

impl ic_stable_structures::Storable for PurchasedMedia {
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
