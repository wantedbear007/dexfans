use ic_stable_structures::StableBTreeMap;

use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub account: StableBTreeMap<candid::Principal, crate::models::types::UserProfile, StoreMemory>,
    pub canister_meta_data: StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory>,
    pub notifications: StableBTreeMap<candid::Principal, crate::models::types::Notification, StoreMemory>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            account: init_account_state(),
            canister_meta_data: init_canister_meta_data_state(),
            notifications: init_notifications_state()
        }
    }

    // // Function to retrieve all posts as a Vec<Post> for serialization
    // pub fn get_all_posts(&self) -> Vec<Post> {
    //     self.posts.iter().map(|(_, post)| post.clone()).collect()
    // }

    // // Function to retrieve all accounts as a Vec<UserProfile> for serialization
    // pub fn get_all_accounts(&self) -> Vec<UserProfile> {
    //     self.account.iter().map(|(_, account)| account.clone()).collect()
    // }
}

fn init_account_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::UserProfile, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_account_data_memory())
}

// fn init_post_state() -> StableBTreeMap<u128, Post, StoreMemory> {
//     StableBTreeMap::init(crate::store::memory::get_post_data_memory())
// }

// fn init_comment_state() -> StableBTreeMap<CommentId, Comment, StoreMemory> {
//     StableBTreeMap::init(crate::store::memory::get_comment_data_memory())
// }

fn init_canister_meta_data_state(
) -> StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_canister_metadata_memory())
}

fn init_notifications_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::Notification, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_notification_data_memory())
}
