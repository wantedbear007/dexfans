use crate::utils::guards::*;
use crate::with_read_state;

#[ic_cdk::query(guard=guard_prevent_anonymous)]
pub fn api_get_my_account() -> Result<crate::models::user_types::UserProfile, String> {
    with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc),
        None => Err(String::from(
            crate::utils::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}
