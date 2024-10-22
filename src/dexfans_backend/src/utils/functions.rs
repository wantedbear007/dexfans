// #[ic_cdk::update]
// pub fn add_admin()

// pub fn update_asset_canister_id()

// pub fn remove_admin()

use crate::with_read_state;

#[ic_cdk::query]
pub fn get_asset_canister() -> Result<candid::Principal, String> {
    with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.asset_canister),
        None => return Err(String::from(super::constants::ERROR_ASSET_CANISTER)),
    })
}
