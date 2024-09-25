use ic_stable_structures::StableBTreeMap;

use crate::models::user_types::UserProfile;

use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub account: StableBTreeMap<candid::Principal, UserProfile, StoreMemory>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            account: init_account_state(),
        }
    }
}

fn init_account_state() -> StableBTreeMap<candid::Principal, UserProfile, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_account_data_memory())
}
