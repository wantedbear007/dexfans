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
pub fn api_get_subscribed() -> Vec<crate::models::types::UserDetailsMinified> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            let mut subscribing: Vec<crate::models::types::UserDetailsMinified> = Vec::new();

            for sub in acc.subscribing.iter() {
                let user_prof = state
                    .account
                    .get(sub)
                    .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

                subscribing.push(crate::models::types::UserDetailsMinified {
                    avatar: user_prof.avatar,
                    user_id: user_prof.user_id,
                    username: user_prof.username,
                    cover: user_prof.cover_image
                });
            }

            subscribing
        }
        None => Vec::new(),
    })
}

// #[ic_cdk::query(guard = guard_prevent_anonymous)]
// fn api_get_user_ids(page: core::types::Pagination) -> Vec<candid::Principal> {
//     crate::with_read_state(|state| {
//         let mut ids: Vec<candid::Principal> = Vec::new();

//         for (id, _) in state.account.iter() {
//             ids.push(id);
//         }

//         let ending = ids.len();

//         if ending == 0 {
//             return ids;
//         }

//         let start = page.start as usize;
//         let end = page.end as usize;
//         if start < ending {
//             let end = end.min(ending);

//             return ids[start..end].to_vec();
//         }

//         Vec::new()
//     })
// }

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_get_subscribers() -> Vec<crate::models::types::UserDetailsMinified> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            let mut subscribers: Vec<crate::models::types::UserDetailsMinified> = Vec::new();

            for sub in acc.subscribers.iter() {
                let user_prof = state
                    .account
                    .get(sub)
                    .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

                subscribers.push(crate::models::types::UserDetailsMinified {
                    avatar: user_prof.avatar,
                    user_id: user_prof.user_id,
                    username: user_prof.username,
                    cover: user_prof.cover_image
                });
            }

            subscribers
        }
        None => Vec::new(),
    })
    // crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
    //     Some(acc) => acc.subscribers,
    //     None => std::collections::HashSet::new(),
    // })
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

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_user_minified(
    id: candid::Principal,
) -> Result<crate::models::types::UserDetailsMinified, String> {
    crate::with_read_state(|state| match state.account.get(&id) {
        Some(acc) => Ok(crate::models::types::UserDetailsMinified {
            avatar: acc.avatar,
            user_id: acc.user_id,
            username: acc.username,
            cover: acc.cover_image
        }),
        None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_user_details(
    id: candid::Principal,
) -> Result<crate::models::types::UserProfileLittleMinified, String> {
    crate::with_read_state(|state| match state.account.get(&id) {
        Some(acc) => Ok(crate::models::types::UserProfileLittleMinified {
            active_post_canister: acc.active_post_canister,
            asset_canister_id: acc.asset_canister_id,
            user_id: acc.user_id,
            all_post_canisters: acc.all_post_canisters,
            subscribers: acc.subscribers,
            subscribing: acc.subscribing,
            avatar: acc.avatar,
            bio: acc.bio,
            cover_image: acc.cover_image,
            created_at: acc.created_at,
            membership: acc.membership,
            username: acc.username,
        }),
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_my_collection(// args: core::types::Pagination,
) -> Result<Vec<core::types::Collection>, String> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => Ok(acc.collects),
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_suggested_user() -> Vec<crate::UserDetailsMinified> {
    crate::with_read_state(|state| {
        let mut accounts: Vec<crate::models::types::UserProfile> = state
            .account
            .iter()
            .map(|(_, acc)| acc)
            .filter(|acc| acc.user_id != ic_cdk::api::caller())
            .collect();

        if accounts.is_empty() {
            return Vec::new();
        }

        accounts.sort_by_key(|val| -(val.subscribers.len() as i32));

        let suggested_users: Vec<crate::UserDetailsMinified> = accounts
            .iter()
            .take(core::constants::ESSENTIAL_SUGGESTED_USER_THRESHOLD as usize)
            .map(|val| crate::UserDetailsMinified {
                avatar: val.avatar.clone(),
                user_id: val.user_id,
                username: val.username.clone(),
                cover: val.cover_image.clone(),
            })
            .collect();

        if suggested_users.is_empty() {
            return Vec::new();
        }

        let mut users: Vec<crate::UserDetailsMinified> = Vec::new();
        for _ in 0..core::constants::ESSENTIAL_SUGGESTED_USER_THRESHOLD {
            if let Some(random_user) = suggested_users
                .get(kaires::get_random_number(0, suggested_users.len() as u64) as usize)
            {
                users.push(random_user.clone());
            }
        }

        users
    })
}
