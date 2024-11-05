use crate::utils::guards::*;

#[ic_cdk::query(guard=guard_prevent_anonymous)]
pub fn api_get_my_profile() -> Result<crate::models::types::UserProfile, String> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(crate::models::types::UserProfile {
            active_post_canister: state
                .canister_meta_data
                .get(&0)
                .unwrap()
                .active_post_canister,
            ..acc
        }),
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
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
fn api_get_user_ids(page: core::types::Pagination) -> Vec<candid::Principal> {
    crate::with_read_state(|state| {
        let mut ids: Vec<candid::Principal> = Vec::new();

        for (id, _) in state.account.iter() {
            ids.push(id);
        }

        let ending = ids.len();

        if ending == 0 {
            return ids;
        }

        let start = page.start as usize;
        let end = page.end as usize;
        if start < ending {
            let end = end.min(ending);

            return ids[start..end].to_vec();
        }

        Vec::new()
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
#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_notifications() -> Vec<crate::NotificationBody> {
    crate::with_read_state(
        |state| match state.notifications.get(&ic_cdk::api::caller()) {
            Some(noti) => noti.notifications,
            None => Vec::new(),
        },
    )
}

#[ic_cdk::query]
fn api_get_user_minified(
    id: candid::Principal,
) -> Result<crate::models::types::UserDetailsMinified, String> {
    crate::with_read_state(|state| match state.account.get(&id) {
        Some(acc) => Ok(crate::models::types::UserDetailsMinified {
            avatar: acc.avatar,
            user_id: acc.user_id,
            username: acc.username,
        }),
        None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}
