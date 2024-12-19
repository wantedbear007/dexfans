// function to get uuid
// pub async fn commons_get_uuid() -> String {
//     format!(
//         "{:x}",
//         sha2::Sha256::digest(
//             &ic_cdk::api::management_canister::main::raw_rand()
//                 .await
//                 .unwrap()
//                 .0
//         )
//     )
// }

// // get post id and increment it
// pub fn utils_get_new_post_id() -> u128 {
//     with_write_state(|state| {
//         let current_post_counter = state.post_counter;
//         state.post_counter += 1;
//         current_post_counter
//     })
// }

use crate::store::storage_state::ApplicationState;

#[ic_cdk::query]
pub fn get_canister_meta_data() -> Result<crate::models::types::CanisterMetaData, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val),
        None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    })
}

pub fn get_parent_canister() -> Result<candid::Principal, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val.canister_ids[core::constants::ESSENTIAL_POST_PARENT_CANISTER]),
        None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    })
}


pub(crate) fn filter_posts(args: core::types::PaginationArgs, state: &ApplicationState, status: core::types::PostStatus, caller_filter: bool, caller: candid::Principal)  -> Vec<crate::models::post::Post> {

    let page = args.page.max(1);
        
    let limit = args.limit.min(100);
    let offset = (page - 1) * limit;
    let mut all_post: Vec<_> = state.posts.iter().filter_map(|(_, pos)| {
        if pos.post_status == status {
            if !caller_filter || caller == pos.creator_id {
                return Some(crate::models::post::Post {
                    like_count: pos.likes.len(),
                    views_count: pos.views.len(),
                    ..pos.clone()
                });
            }
        }
        None
    }).collect();
    all_post.reverse();
    all_post.into_iter().skip(offset as usize).take(limit as usize).collect()
}