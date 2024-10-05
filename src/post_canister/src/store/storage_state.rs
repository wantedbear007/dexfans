
use ic_stable_structures::StableBTreeMap;


use crate::models::{post::Post, user::UserProfile};

use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub account: StableBTreeMap<candid::Principal, UserProfile, StoreMemory>,
    pub posts: StableBTreeMap<u128, Post, StoreMemory>,
    pub post_counter: u128,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            account: init_account_state(),
            posts: init_post_state(),
            post_counter: 0,
        }
    }

    // Function to retrieve all posts as a Vec<Post> for serialization
    pub fn get_all_posts(&self) -> Vec<Post> {
        self.posts.iter().map(|(_, post)| post.clone()).collect()
    }

    // Function to retrieve all accounts as a Vec<UserProfile> for serialization
    pub fn get_all_accounts(&self) -> Vec<UserProfile> {
        self.account.iter().map(|(_, account)| account.clone()).collect()
    }

}

fn init_account_state() -> StableBTreeMap<candid::Principal, UserProfile, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_account_data_memory())
}

fn init_post_state() -> StableBTreeMap<u128, Post, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_post_data_memory())
}
