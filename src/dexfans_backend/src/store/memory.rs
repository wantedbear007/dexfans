use std::cell::RefCell;

use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl};

const ACCOUNT_DATA: MemoryId = MemoryId::new(0);

pub type StoreMemory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;


thread_local! {
  static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
      RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_account_data_memory() -> StoreMemory {
  MEMORY_MANAGER.with(|m| m.borrow().get(ACCOUNT_DATA))
}
