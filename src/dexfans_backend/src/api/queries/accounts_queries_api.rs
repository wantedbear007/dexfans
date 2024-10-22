

use crate::utils::guards::*;

#[ic_cdk::query(guard=guard_prevent_anonymous)]
pub fn api_get_my_profile() -> Result<crate::models::types::UserProfile, String> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}


// #[ic_cdk::query]
// pub fn api_get_user_account(user_id: Principal) -> Result<crate::models::user::UserProfile, String> {
//     with_read_state(|state| {
//         state.account.get(&user_id)
//             .map(|acc| acc.clone())
//             .ok_or_else(|| String::from(crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED))
//     })
// }



// #[ic_cdk::query(guard=guard_only_admin)]
// pub fn list_all_accounts() -> Vec<UserProfile> {
//     STATE.with(|state| {
//         let app_state = state.borrow();
//         app_state.get_all_accounts()  
//     })
// }