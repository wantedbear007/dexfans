use crate::models::types::{Membership, PostId};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserInputArgs {
    pub username: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub cover_image: Option<String>,
}

#[derive(Clone, CandidType, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct UserProfile {
    pub user_id: Principal,
    pub username: String,
    pub posts: Vec<PostId>,
    pub likes: Vec<PostId>,
    pub collects: Vec<PostId>,
    pub membership: Membership,
}

impl ic_stable_structures::Storable for UserProfile {
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
