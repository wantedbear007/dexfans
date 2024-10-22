// #[ic_cdk::update]
// pub fn add_admin()

// pub fn update_asset_canister_id()

// pub fn remove_admin()

use super::guards::*;
use crate::{with_read_state, with_write_state};

#[ic_cdk::query]
pub fn get_asset_canister() -> Result<candid::Principal, String> {
    with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.asset_canister),
        None => return Err(String::from(super::constants::ERROR_CANISTER_ID)),
    })
}

#[ic_cdk::query]
pub fn get_post_canister() -> Result<candid::Principal, String> {
    with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.post_canister),
        None => return Err(String::from(super::constants::ERROR_CANISTER_ID)),
    })
}

#[ic_cdk::update(guard = guard_only_admin)]
pub fn set_post_canister(id: candid::Principal) -> Result<candid::Principal, String> {
    with_write_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            state.canister_meta_data.insert(
                0,
                crate::models::types::CanisterMetaData {
                    post_canister: id,
                    ..val
                },
            );

            Ok(id)
        }
        None => return Err(String::from(super::constants::ERROR_CANISTER_ID)),
    })
}
