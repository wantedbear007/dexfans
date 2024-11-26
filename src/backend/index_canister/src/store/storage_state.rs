use ic_stable_structures::StableBTreeMap;

use super::memory::StoreMemory;

pub(crate) struct ApplicationState {
    pub account: StableBTreeMap<candid::Principal, crate::models::types::UserProfile, StoreMemory>,
    pub canister_meta_data: StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory>,
    pub notifications:
        StableBTreeMap<candid::Principal, crate::models::types::Notification, StoreMemory>,
    pub purchased_post:
        StableBTreeMap<candid::Principal, crate::models::types::PurchasedPosts, StoreMemory>,
    pub purchased_media:
        StableBTreeMap<candid::Principal, crate::models::types::PurchasedMedia, StoreMemory>,
}

pub(crate) struct CaptchaState {
    pub captchas: StableBTreeMap<u8, crate::models::types::Captchas, StoreMemory>,
}

// pub(crate) struct ServicesState {
//     pub notifications:
//     StableBTreeMap<candid::Principal, crate::models::types::Notification, StoreMemory>,
// }

// impl ServicesState {
//     pub fn new() -> Self {
//         Self {
//             notifications: init_notifications_state()
//         }
//     }
// }

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            account: init_account_state(),
            canister_meta_data: init_canister_meta_data_state(),
            notifications: init_notifications_state(),
            purchased_post: init_purchased_state(),
            purchased_media: init_purchased_media_state(),
        }
    }
}

impl CaptchaState {
    pub fn new() -> Self {
        Self {
            captchas: init_captcha_state(),
        }
    }
}

fn init_account_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::UserProfile, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_account_data_memory())
}

fn init_canister_meta_data_state(
) -> StableBTreeMap<u8, crate::models::types::CanisterMetaData, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_canister_metadata_memory())
}

fn init_notifications_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::Notification, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_notification_data_memory())
}

fn init_purchased_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::PurchasedPosts, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_purchased_post_memory())
}

fn init_purchased_media_state(
) -> StableBTreeMap<candid::Principal, crate::models::types::PurchasedMedia, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_purchased_media_memory())
}

fn init_captcha_state() -> StableBTreeMap<u8, crate::models::types::Captchas, StoreMemory> {
    StableBTreeMap::init(crate::store::memory::get_captcha_memory())
}
