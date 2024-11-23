use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};

const ACCOUNT_DATA: MemoryId = MemoryId::new(0);

const CANISTER_DATA: MemoryId = MemoryId::new(1);

const NOTIFICATIONS_DATA: MemoryId = MemoryId::new(2);

const PURCHASED_POST: MemoryId = MemoryId::new(3);

const PURCHASED_IMAGE_OR_VIDEO: MemoryId = MemoryId::new(4);

pub type StoreMemory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;

thread_local! {
  static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
      RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_account_data_memory() -> StoreMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(ACCOUNT_DATA))
}

// pub fn get_post_data_memory() -> StoreMemory {
//   MEMORY_MANAGER.with(|m| m.borrow().get(POST_DATA))  // New function for post storage
// }

// pub fn get_comment_data_memory() -> StoreMemory {
//   MEMORY_MANAGER.with(|m| m.borrow().get(COMMENT_DATA_MEMORY_ID))
// }

pub fn get_notification_data_memory() -> StoreMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(NOTIFICATIONS_DATA))
}

pub fn get_purchased_post_memory() -> StoreMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(PURCHASED_POST)) // New function for post storage
}

pub fn get_purchased_media_memory() -> StoreMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(PURCHASED_IMAGE_OR_VIDEO)) // New function for post storage
}

pub fn get_canister_metadata_memory() -> StoreMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CANISTER_DATA)) // New function for post storage
}
