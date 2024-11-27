use ic_stable_structures::StableBTreeMap;

use crate::models::{comment::Comment, post::Post, types::UserProfileIC};

use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub account: StableBTreeMap<candid::Principal, UserProfileIC, StoreMemory>,
    pub posts: StableBTreeMap<u128, Post, StoreMemory>,
    pub comments: StableBTreeMap<crate::models::types::PostId, Comment, StoreMemory>,
    pub post_counter: u128,
    pub comment_counter: u128,
    // pub notifications: HashMap<Principal, Vec<Notification>>,
    pub canister_meta_data: StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            account: init_account_state(),
            posts: init_post_state(),
            comments: init_comment_state(),
            post_counter: 0,
            comment_counter: 0,
            canister_meta_data: init_canister_meta_data_state(),
            // notifications: HashMap::new(),
        }
    }

    // Function to retrieve all posts as a Vec<Post> for serialization
    // pub fn get_all_posts(&self) -> Vec<Post> {
    //     self.posts.iter().map(|(_, post)| post.clone()).collect()
    // }

    // Function to retrieve all accounts as a Vec<UserProfile> for serialization
    // pub fn get_all_accounts(&self) -> Vec<UserProfileIC> {
    //     self.account
    //         .iter()
    //         .map(|(_, account)| account.clone())
    //         .collect()
    // }
}

fn init_account_state() -> StableBTreeMap<candid::Principal, UserProfileIC, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_account_data_memory())
}

fn init_post_state() -> StableBTreeMap<u128, Post, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_post_data_memory())
}

fn init_comment_state() -> StableBTreeMap<core::types::CommentId, Comment, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_comment_data_memory())
}

fn init_canister_meta_data_state(
) -> StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_canister_metadata_memory())
}
