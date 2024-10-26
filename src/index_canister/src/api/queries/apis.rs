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

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_get_subscribed() -> std::collections::HashSet<candid::Principal> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => acc.subscribing,
        None => std::collections::HashSet::new(),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_get_subscribers() -> std::collections::HashSet<candid::Principal> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => acc.subscribers,
        None => std::collections::HashSet::new(),
    })
}

// TODO add guard
#[ic_cdk::query]
fn api_get_notifications() -> Vec<crate::NotificationBody> {
    crate::with_read_state(
        |state| match state.notifications.get(&ic_cdk::api::caller()) {
            Some(noti) => noti.notifications,
            None => Vec::new(),
        },
    )
}
