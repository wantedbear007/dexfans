use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};

use serde::{Deserialize, Serialize};

pub type CanisterId = Principal;
pub type CommentId = u128;
pub type Cycles = u128;
pub type PostId = String;
pub type TimestampMillis = u64;

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserInputArgs {
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub cover_image: Option<String>,
}

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserProfileInterCanister {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: Membership,
}

impl ic_stable_structures::Storable for UserProfileInterCanister {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::into_writer(self, &mut buf).expect(dexfans_types::constants::ERROR_ENCODE_FAILED);
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        ciborium::from_reader(&bytes[..]).expect(dexfans_types::constants::ERROR_DECODE_FAILED)
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub enum PostType {
    Free,
    Silver,
    Gold,
    Platinum,
    Paid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub enum Membership {
    Guest,
    Silver,
    Gold,
    Platinum,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub enum CanisterMeta {
    AssetCanister,
    ParentCanister,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, CandidType)]
pub struct CanisterMetaData {
    pub asset_canister: Vec<Principal>,
    pub parent_canister: Principal,
    // pub post_canister: Principal
    
    // more to be added later
}

impl ic_stable_structures::Storable for CanisterMetaData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}
