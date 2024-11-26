use rand::Rng;

#[ic_cdk::query]
pub fn get_asset_canister() -> Result<candid::Principal, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.canister_ids[&core::constants::ESSENTIAL_ASSET_CANISTER_ID_CODE]),
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

#[ic_cdk::query]
pub fn get_ledger_canister() -> Result<candid::Principal, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE]),
        None => return Err(String::from(core::constants::ERROR_CANISTER_ID)),
    })
}

pub(crate) fn random_number(low: u32) -> u32 {
    super::init::RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        let rng = rng.as_mut().expect("RNG error");

        rng.gen_range(low, 100)
    })

    
}

pub(crate) fn shuffle_vec<T>(vec: &mut Vec<T>, seed: &mut u32) {
    fn next_random(seed: &mut u32) -> usize {
        *seed = (*seed).wrapping_mul(1664525).wrapping_add(1013904223); // Linear congruential generator (LCG)
        (*seed >> 16) as usize // Extract pseudo-random number
    }

    let mut n = vec.len();
    while n > 1 {
        n -= 1;
        let random_index = next_random(seed) % (n + 1); // Get random index
        vec.swap(n, random_index); // Swap current element with random index
    }
}

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
