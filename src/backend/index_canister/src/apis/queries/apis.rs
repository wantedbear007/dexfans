
use crate::utils::guards::*;

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_my_profile() -> Result<crate::models::types::UserProfile, String> {
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

fn api_get_subscribed() -> Vec<core::types::UserDetailsMinified> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            let mut subscribing: Vec<core::types::UserDetailsMinified> = Vec::new();

            for sub in acc.subscribing.iter() {
                let user_prof = state
                    .account
                    .get(sub)
                    .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

                subscribing.push(core::types::UserDetailsMinified {
                    avatar: user_prof.avatar,
                    user_id: user_prof.user_id,
                    username: user_prof.username,
                    cover: user_prof.cover_image,
                });
            }

            subscribing
        }
        None => Vec::new(),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_subscribers() -> Vec<core::types::UserDetailsMinified> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            let mut subscribers: Vec<core::types::UserDetailsMinified> = Vec::new();

            for sub in acc.subscribers.iter() {
                let user_prof = state
                    .account
                    .get(sub)
                    .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

                subscribers.push(core::types::UserDetailsMinified {
                    avatar: user_prof.avatar,
                    user_id: user_prof.user_id,
                    username: user_prof.username,
                    cover: user_prof.cover_image,
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

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn api_get_notifications() -> Vec<crate::NotificationBody> {

    crate::with_write_state(|state| {
        if let Some(mut f_notifications) = state.notifications.get(&ic_cdk::api::caller()) {
            f_notifications.notifications.retain(|noti| {
                noti.expiring_on > ic_cdk::api::time()
            });

        state.notifications.insert(ic_cdk::api::caller(), f_notifications.clone());
        f_notifications.notifications
        } else {
            Vec::new()
        }
    })

}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_user_minified(
    id: candid::Principal,
) -> Result<core::types::UserDetailsMinified, String> {
    crate::with_read_state(|state| match state.account.get(&id) {
        Some(acc) => Ok(core::types::UserDetailsMinified {
            avatar: acc.avatar,
            user_id: acc.user_id,
            username: acc.username,
            cover: acc.cover_image,
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
            token_amount: acc.token_amount,
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

        // let mut random = rand::thread_rng();
        // suggested_users.to_owned().shuffle(&mut random);

        suggested_users
    })
}

// search user
#[ic_cdk::query]
fn api_search_user(args: String) -> Vec<crate::UserDetailsMinified> {
    if args.len() > core::constants::VALIDATOR_USERNAME_LENGTH as usize || args.len() < 3 {
        ic_cdk::trap("Provided param is too large")
    }

    crate::with_read_state(|state| {
        state
            .account
            .iter()
            .filter_map(|(_, mut acc)| {
                if acc.username.to_lowercase().contains(&args.to_lowercase())
                    || acc
                        .bio
                        .get_or_insert(String::from(""))
                        .contains(&args.to_lowercase())
                // if (strsim::levenshtein(&acc.username, &args)
                //     <= core::constants::ESSENTIAL_FUZZY_SEARCH_THRESHOLD)
                //     || (strsim::levenshtein(&acc.bio.get_or_insert("".to_string()), &args)
                //         <= core::constants::ESSENTIAL_FUZZY_SEARCH_THRESHOLD)
                {
                    Some(core::types::UserDetailsMinified {
                        avatar: acc.avatar,
                        cover: acc.cover_image,
                        user_id: acc.user_id,
                        username: acc.username,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_purchased_posts_ids() -> Vec<crate::models::types::PurchasePostBody> {
    crate::with_read_state(
        |state| match state.purchased_post.get(&ic_cdk::api::caller()) {
            Some(posts) => posts.posts,
            None => Vec::new(),
        },
    )
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_purchased_media_ids() -> Vec<crate::models::types::PurchaseMediaBody> {
    crate::with_read_state(
        |state| match state.purchased_media.get(&ic_cdk::api::caller()) {
            Some(posts) => posts.medias,
            None => Vec::new(),
        },
    )
}
