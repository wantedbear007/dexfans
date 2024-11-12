use core::types::Membership;

use crate::{utils::guards::*, with_read_state};

use super::controllers::ic_update_membership;

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub async fn api_create_account(
    args: crate::models::types::UserInputArgs,
) -> Result<String, String> {
    super::controllers::controller_create_account(args)
        .await
        .map_err(|err| {
            format!(
                "{}{}",
                core::constants::ERROR_ACCOUNT_ERROR,
                err.to_string()
            )
        })?;

    Ok(String::from(core::constants::SUCCESS_ACCOUNT_CREATED))
}

// update profile
#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub async fn api_update_profile(
    // user_id: Principal,
    args: crate::models::types::UserInputArgs,
) -> Result<(), String> {
    match crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(val) => {
            state.account.insert(
                ic_cdk::api::caller(),
                crate::models::types::UserProfile {
                    username: args.username.clone(),
                    bio: args.bio,
                    avatar: args.avatar,
                    cover_image: args.cover_image,
                    ..val
                },
            );

            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_PROFILE_UPDATE)),
    }) {
        Ok(()) => {
            match super::controllers::ic_update_profile(core::types::UpdateUserProfileArgsIC {
                user_id: ic_cdk::api::caller(),
                username: args.username,
            })
            .await
            {
                Ok(()) => Ok(()),
                Err(err) => {
                    // add rollback if call fails

                    return Err(err);
                }
            }
        }
        Err(err) => return Err(err),
    }

    // Ok(())
}

// update membership
// TODO add frontend canister guard
#[ic_cdk::update]
pub async fn api_update_membership(args: core::types::Membership) -> Result<(), String> {
    // msp to save the current state of membership
    let mut msp: Membership = Membership::Guest;
    match crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(mut val) => {
            if &val.membership == &args {
                return Err(String::from(core::constants::WARNING_SAME_VALUE));
            }
            msp = val.membership;
            val.membership = args.clone();
            state.account.insert(ic_cdk::api::caller(), val);
            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
    }) {
        Ok(()) => {
            // to update membership on post canister
            match ic_update_membership(args).await {
                Ok(()) => Ok(()),
                Err(err) => {
                    // roll back
                    super::controllers::rb_membership_update(msp)
                        .expect(core::constants::ERROR_FAILED_CALL);
                    return Err(err);
                }
            }
        }
        Err(err) => return Err(err),
    }
}

// #[ic_cdk::update]
// pub fn api_add_post_id_to_user(user_id: Principal, post_id: u128) -> Result<(), String> {
//     STATE.with(|state| {
//         let mut app_state = state.borrow_mut();

//         if let Some(mut user_profile) = app_state.account.remove(&user_id) {
//             user_profile.posts.push(post_id); // Modify the profile
//             app_state.account.insert(user_id, user_profile); // Reinsert the modified profile
//             Ok(())
//         } else {
//             Err("User not found.".to_string())
//         }
//     })
// }

#[ic_cdk::update]
pub fn api_update_user_likes(
    user_id: candid::Principal,
    post_id: u128,
    is_liked: bool,
) -> Result<(), String> {
    crate::utils::init::STATE.with(|state| {
        let mut app_state = state.borrow_mut();

        // Remove the user profile from the map, if it exists
        if let Some(mut user_profile) = app_state.account.remove(&user_id) {
            if is_liked {
                // If the post is already liked, remove it (unlike)
                user_profile.likes.retain(|&p| p != post_id);
            } else {
                // If not liked, add the post_id to the likes list (like)
                user_profile.likes.push(post_id);
            }

            // Reinsert the modified profile back into the map
            app_state.account.insert(user_id, user_profile);
            Ok(())
        } else {
            Err("User not found.".to_string())
        }
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn api_subscribe_account(id: candid::Principal) -> Result<(), String> {
    super::apis_ic::ic_subscribe_account(core::types::SubscribeAccountIC {
        subscribed_by: ic_cdk::api::caller(),
        subscribed_to: id,
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn api_unsubscribe_account(id: candid::Principal) -> Result<(), String> {
    super::apis_ic::ic_unsubscribe_account(core::types::UnsubscribeAccountIC {
        unsubscribed_by: ic_cdk::api::caller(),
        unsubscribed_to: id,
    })
}

// to create notification for new post
#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn notify_subscribers_newpost() -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            for x in acc.subscribers.iter() {
                match state.notifications.get(&x) {
                    Some(mut usr) => {
                        usr.notifications.push(core::types::NotificationBody {
                            by: None,

                            category: core::types::NotificationType::NewPost,
                            created_on: ic_cdk::api::time(),
                            expiring_on: ic_cdk::api::time()
                                + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
                            description: None,
                            title: format!("{} has recently posted", ic_cdk::api::caller()),
                        });

                        state.notifications.insert(usr.acc, usr);
                    }
                    None => {}
                }
            }

            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

// likes notification
// TODO add guard
#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn notify_likes(args: core::types::LikeNotificationArgs) -> Result<(), String> {
    crate::with_write_state(|state| match state.notifications.get(&args.post_owner) {
        Some(mut val) => {
            val.notifications.push(core::types::NotificationBody {
                by: Some(ic_cdk::api::caller()),
                category: core::types::NotificationType::NewLike,
                created_on: ic_cdk::api::time(),
                expiring_on: ic_cdk::api::time() + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
                description: None,
                title: format!(
                    "{} liked your post {}",
                    ic_cdk::api::caller(),
                    args.post_url
                ),
            });

            state.notifications.insert(val.acc, val);

            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

// notify comments
#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn notify_comments(args: core::types::CommentNotificationArgs) -> Result<(), String> {
    crate::with_write_state(|state| match state.notifications.get(&args.post_owner) {
        Some(mut val) => {
            val.notifications.push(core::types::NotificationBody {
                by: Some(ic_cdk::api::caller()),
                category: core::types::NotificationType::NewComment,
                created_on: ic_cdk::api::time(),
                expiring_on: ic_cdk::api::time() + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
                description: None,
                title: format!(
                    "{} commented on your post, {}{}",
                    ic_cdk::api::caller(),
                    args.description,
                    args.post_url
                ),
            });

            state.notifications.insert(val.acc, val);

            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

// #[ic_cdk::update]
// pub async fn debug_complete_payment(amt: u64) -> Result<Nat, String> {
//     super::payment_controller::icp_transfer_handler(amt).await
// }

// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// #[ic_cdk::update]
// pub fn api_purchase_membership(args: core::types::Membership) -> Result<(), String> {
//     super::controllers::controller_membership(args)
// }

// TODO COMPLETE BELOW
// // notify new subscriber
// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// pub fn notify_new_subscriber(id: candid::Principal) -> Result<(), String> {
//     crate::with_write_state(|state| match state.notifications.get(&ic_cdk::api::caller()) {
//         Some(mut val) => {
//             val.notifications
//                 .push(core::types::NotificationBody {
//                     by: Some(ic_cdk::api::caller()),
//                     category: core::types::NotificationType::NewComment,
//                     created_on: ic_cdk::api::time(),
//                     expiring_on: ic_cdk::api::time()
//                         + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
//                     description: None,
//                     title: format!(
//                         "{} has subscribed you",
//                         ic_cdk::api::,

//                     ),
//                 });

//             state.notifications.insert(val.acc, val);

//             Ok(())
//         }
//         None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
//     })
// }

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_purchase_membership(args: core::types::Membership) -> Result<(), String> {
    // canister meta data (ledger and plan prices)
    let meta_data = with_read_state(|state| state.canister_meta_data.get(&0))
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    // to retrieve user profile
    let mut account = with_read_state(|state| state.account.get(&ic_cdk::api::caller()))
        .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

    // checks
    if &account.membership == &args {
        return Err(String::from(core::constants::WARNING_SAME_MEMBERSHIP));
    }

    if &account.membership > &args {
        return Err(String::from(core::constants::WARNING_HIGHER_MEMBERSHIP));
    }

    // payment
    match super::payment_controller::icp_transfer_handler(
        meta_data.membership_plans[&args],
        meta_data.payment_recipient,
        meta_data.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE],
    )
    .await
    {
        Ok(val) => {
            account.membership_ledger_block = Some(val);
            account.membership = args;
            account.membership_till =
                ic_cdk::api::time() + core::constants::ESSENTIAL_MEMBERSHIP_VALIDITY;
            crate::with_write_state(|state| state.account.insert(ic_cdk::api::caller(), account));
        }
        Err(err) => return Err(err),
    };

    Ok(())
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn api_add_to_collection(args: core::types::Collection) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(mut acc) => {
            acc.collects.push(args);

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}
