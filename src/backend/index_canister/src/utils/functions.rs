#[ic_cdk::query]
pub fn get_asset_canister() -> Result<candid::Principal, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.active_asset_canister),
        // Some(val) => Ok(val.canister_ids[&core::constants::ESSENTIAL_IC_OSS_BUCKET_ID_CODE]),
        None => return Err(String::from(core::constants::ERROR_CANISTER_ID)),
    })
}

#[ic_cdk::query]
pub fn get_post_canister() -> Result<candid::Principal, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.canister_ids[&core::constants::ESSENTIAL_POST_CANISTER_ID_CODE]),
        None => return Err(String::from(core::constants::ERROR_CANISTER_ID)),
    })
}



// #[ic_cdk::query]
// pub fn get_ledger_canister() -> Result<candid::Principal, String> {
//     crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
//         Some(val) => Ok(val.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE]),
//         None => return Err(String::from(core::constants::ERROR_CANISTER_ID)),
//     })
// }

// // to add controller of canister
// #[ic_cdk::update(guard=guard_only_admin)]
// pub fn add_controller(id: candid::Principal) -> Result<(), String> {
//     crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
//         Some(val) => {
//             state.canister_meta_data.insert(
//                 0,
//                 crate::models::types::CanisterMetaData {
//                     controllers: {
//                         let mut controllers = val.controllers;
//                         controllers.insert(id);
//                         controllers
//                     },
//                     ..val
//                 },
//             );

//             Ok(())
//         }
//         None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
//     })
// }

// // to remove controller of canister
// #[ic_cdk::update(guard=guard_only_admin)]
// pub fn remove_controller(id: candid::Principal) -> Result<(), String> {
//     crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
//         Some(val) => {
//             let controllers: std::collections::HashSet<candid::Principal> = val
//                 .controllers
//                 .iter()
//                 .filter(|&&controller| controller != id)
//                 .cloned()
//                 .collect();

//             state.canister_meta_data.insert(
//                 0,
//                 crate::models::types::CanisterMetaData { controllers, ..val },
//             );

//             Ok(())
//         }
//         None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
//     })
// }

// // to update the current post canister
// #[ic_cdk::update(guard = guard_only_admin)]
// pub fn set_post_canister(id: candid::Principal) -> Result<candid::Principal, String> {
//     crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
//         Some(mut canister_meta_data) => {
//             canister_meta_data.all_post_canisters.insert(id);

//             canister_meta_data.canister_ids.insert(
//                 core::constants::ESSENTIAL_POST_CANISTER_ID_CODE,
//                 id,
//             );
//             state.canister_meta_data.insert(0, canister_meta_data);

//             Ok(id)
//         }
//         None => return Err(String::from(core::constants::ERROR_CANISTER_ID)),
//     })
// }

// #[ic_cdk::query]
// pub fn get_canister_meta_data() -> Result<crate::models::types::CanisterMetaData, String> {
//     crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
//         Some(val) => Ok(val),
//         None => {
//             return Err(String::from(
//                 core::constants::ERROR_FAILED_CANISTER_DATA,
//             ))
//         }
//     })
// }
