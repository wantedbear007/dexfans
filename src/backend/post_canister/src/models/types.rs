use std::borrow::Cow;

use candid::{CandidType, Principal};

use serde::{Deserialize, Serialize};

// pub type CanisterId = Principal;
pub type PostId = u128;
pub type Id = u128;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum IdType {
    PostID,
    CommentID,
}

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserInputArgs {
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub cover_image: Option<String>,
}

#[derive(Clone, CandidType, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserProfileIC {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: core::types::Membership,
}

impl Default for UserProfileIC {
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

impl ic_stable_structures::Storable for UserProfileIC {
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

impl ic_stable_structures::Storable for IdType {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub enum CanisterMeta {
    AssetCanister,
    ParentCanister,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub canister_ids: std::collections::HashMap<String, candid::Principal>,
    pub controllers: std::collections::HashSet<candid::Principal>,
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
    // fn to_bytes(&self) -> Cow<[u8]> {
    //     Cow::Owned(Encode!(self).unwrap())
    // }

    // fn from_bytes(bytes: Cow<[u8]>) -> Self {
    //     Decode!(bytes.as_ref(), Self).unwrap()
    // }

    // const BOUND: ic_stable_structures::storable::Bound =
    //     ic_stable_structures::storable::Bound::Unbounded;
}
