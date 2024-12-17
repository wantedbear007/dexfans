// use validator::Validate;

use crate::utils::guards::*;

// #[ic_cdk::query]
// fn greet(name: String) -> String {
//     format!(
//         "Hello, {}! from {}",
//         name,
//         core::constants::ESSENTIALS_APP_NAME
//     )
// }

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub async fn api_create_account(
    args: crate::models::types::UserInputArgs,
) -> Result<String, String> {
    // validation input args
    core::functions::input_validator(&args)?;

    // to verify captcha
    crate::utils::challanges::verify_captcha(ic_cdk::api::caller(), &args.captcha_solution)?;

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

    // match args.validate() {
    //     Ok(_) => {
    //         // to verify captcha
    //         crate::utils::challanges::verify_captcha(
    //             ic_cdk::api::caller(),
    //             &args.captcha_solution,
    //         )?;

    //         super::controllers::controller_create_account(args)
    //             .await
    //             .map_err(|err| {
    //                 format!(
    //                     "{}{}",
    //                     core::constants::ERROR_ACCOUNT_ERROR,
    //                     err.to_string()
    //                 )
    //             })?;

    //         Ok(String::from(core::constants::SUCCESS_ACCOUNT_CREATED))
    //     }
    //     Err(err) => Err(format!("Validation Error:  {}", err)),
    // }

    // // to verify captcha
    // crate::utils::challanges::verify_captcha(ic_cdk::api::caller(), &args.captcha_solution)?;

    // super::controllers::controller_create_account(args)
    //     .await
    //     .map_err(|err| {
    //         format!(
    //             "{}{}",
    //             core::constants::ERROR_ACCOUNT_ERROR,
    //             err.to_string()
    //         )
    //     })?;

    // Ok(String::from(core::constants::SUCCESS_ACCOUNT_CREATED))
}

// update profile
#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub async fn api_update_profile(
    // user_id: Principal,
    args: crate::models::types::UserInputArgs,
) -> core::types::Response {
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
        Err(err) => Err(err),
    }
}

// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// fn api_update_user_likes(
//     user_id: candid::Principal,
//     post_id: u128,
//     is_liked: bool,
// ) -> core::types::Response {
//     crate::utils::init::STATE.with(|state| {
//         let mut app_state = state.borrow_mut();

//         // Remove the user profile from the map, if it exists
//         if let Some(mut user_profile) = app_state.account.remove(&user_id) {
//             if is_liked {
//                 // If the post is already liked, remove it (unlike)
//                 user_profile.likes.retain(|&p| p != post_id);
//             } else {
//                 // If not liked, add the post_id to the likes list (like)
//                 user_profile.likes.push(post_id);
//             }

//             // Reinsert the modified profile back into the map
//             app_state.account.insert(user_id, user_profile);
//             Ok(())
//         } else {
//             Err("User not found.".to_string())
//         }
//     })
// }

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn api_subscribe_account(id: candid::Principal) -> core::types::Response {
    super::apis_ic::ic_subscribe_account(core::types::SubscribeAccountIC {
        subscribed_by: ic_cdk::api::caller(),
        subscribed_to: id,
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn api_unsubscribe_account(id: candid::Principal) -> core::types::Response {
    super::apis_ic::ic_unsubscribe_account(core::types::UnsubscribeAccountIC {
        unsubscribed_by: ic_cdk::api::caller(),
        unsubscribed_to: id,
    })
}

// to create notification for new post
#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn notify_subscribers_newpost(
    post_brief: Option<String>,
    post_id: core::types::PostId,
) -> core::types::Response {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(acc) => {
            for x in acc.subscribers.iter() {
                match state.notifications.get(&x) {
                    Some(mut usr) => {
                        usr.notifications.push(core::types::NotificationBody {
                            post_id: Some(post_id),
                            comment_content: None,
                            post_brief: post_brief.clone(),
                            by: {
                                let user_data = state
                                    .account
                                    .get(&ic_cdk::api::caller())
                                    .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);
                                Some(core::types::UserDetailsMinified {
                                    avatar: user_data.avatar,
                                    cover: user_data.cover_image,
                                    user_id: user_data.user_id,
                                    username: user_data.username,
                                })
                            },
                            category: core::types::NotificationType::NewPost,
                            created_on: ic_cdk::api::time(),
                            expiring_on: ic_cdk::api::time()
                                + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
                        });

                        state.notifications.insert(usr.acc, usr);
                    }
                    None => {}
                }
            }

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn notify_likes(args: core::types::LikeNotificationArgs) -> core::types::Response {
    // validating input
    core::functions::input_validator(&args)?;


    crate::with_write_state(|state| match state.notifications.get(&args.post_owner) {
        Some(mut val) => {
            val.notifications.push(core::types::NotificationBody {
                post_brief: Some(args.post_brief),
                comment_content: None,
                post_id: Some(args.post_id),
                by: {
                    let user_data = state
                        .account
                        .get(&ic_cdk::api::caller())
                        .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);
                    Some(core::types::UserDetailsMinified {
                        avatar: user_data.avatar,
                        cover: user_data.cover_image,
                        user_id: user_data.user_id,
                        username: user_data.username,
                    })
                },
                // subscriber_id: None,
                category: core::types::NotificationType::NewLike,
                created_on: ic_cdk::api::time(),
                expiring_on: ic_cdk::api::time() + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
            });

            state.notifications.insert(val.acc, val);

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

// notify comments
#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn notify_comments(args: core::types::CommentNotificationArgs) -> core::types::Response {
    // validating input
    core::functions::input_validator(&args)?;
    

    crate::with_write_state(|state| match state.notifications.get(&args.post_owner) {
        Some(mut val) => {
            val.notifications.push(core::types::NotificationBody {
                comment_content: Some(args.comment_content),
                post_brief: args.post_brief,
                post_id: Some(args.post_id),
                by: {
                    let user_data = state
                        .account
                        .get(&ic_cdk::api::caller())
                        .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);
                    Some(core::types::UserDetailsMinified {
                        avatar: user_data.avatar,
                        cover: user_data.cover_image,
                        user_id: user_data.user_id,
                        username: user_data.username,
                    })
                },
                category: core::types::NotificationType::NewComment,
                created_on: ic_cdk::api::time(),
                expiring_on: ic_cdk::api::time() + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
            });

            state.notifications.insert(val.acc, val);

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

// // notify new subscriber
#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn notify_new_subscriber(subscribed_to: candid::Principal) -> core::types::Response {
    crate::with_write_state(|state| match state.notifications.get(&subscribed_to) {
        Some(mut noti) => {
            noti.notifications.push(core::types::NotificationBody {
                comment_content: None,
                post_brief: None,
                post_id: None,
                by: {
                    let user_data = state
                        .account
                        .get(&ic_cdk::api::caller())
                        .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);
                    Some(core::types::UserDetailsMinified {
                        avatar: user_data.avatar,
                        cover: user_data.cover_image,
                        user_id: user_data.user_id,
                        username: user_data.username,
                    })
                },
                category: core::types::NotificationType::NewSubscriber,
                created_on: ic_cdk::api::time(),
                expiring_on: ic_cdk::api::time() + core::constants::ESSENTIAL_NOTIFICATION_EXPIRING,
                // subscriber_id:
            });

            state.notifications.insert(noti.acc, noti);

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_purchase_membership(args: core::types::Membership) -> core::types::Response {
    // canister meta data (ledger and plan prices)
    let meta_data = crate::with_read_state(|state| state.canister_meta_data.get(&0))
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    // to retrieve user profile
    let mut account = crate::with_read_state(|state| state.account.get(&ic_cdk::api::caller()))
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
        meta_data.membership_plans[&args].clone(),
        meta_data.payment_recipient,
        meta_data.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE],
    )
    .await
    {
        Ok(val) => {
            account.membership_ledger_block = {
                account.membership_ledger_block.insert(val);
                account.membership_ledger_block
            };
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
fn api_add_to_collection(args: core::types::Collection) -> core::types::Response {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(mut acc) => {
            acc.collects.push(args);

            Ok(())
        }
        None => Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_purchase_post(
    post_id: core::types::PostId,
    post_canister_id: candid::Principal,
) -> core::types::Response {
    // canister id validity
    crate::utils::guards::validate_post_canister(post_canister_id)?;

    // checking if user already owns the post
    crate::with_read_state(
        |state| match state.purchased_post.get(&ic_cdk::api::caller()) {
            Some(acc) => {
                let posts: Vec<u128> = acc.posts.iter().map(|e| e.post_id).collect();

                if posts.contains(&post_id) {
                    return Err(String::from(core::constants::WARNING_ALREADY_PURCHASED));
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        },
    )
    .map_err(|err| return format!("{}", err))?;

    // get post data (owner and price)
    let post_data = kaires::call_inter_canister::<
        core::types::PostPurchaseArgs,
        core::types::PurchaseUserMedia,
    >(
        core::constants::FUNCTION_GET_POST_PRICE,
        core::types::PostPurchaseArgs {
            created_by: ic_cdk::api::caller(),
            post_id,
        },
        post_canister_id,
    )
    .await
    .expect(core::constants::ERROR_FAILED_INTER_CANISTER);

    if post_data.amt == candid::Nat::default() {
        return Err(String::from(core::constants::WARNING_POST_IS_FREE));
    }

    let meta_data = crate::with_read_state(|state| state.canister_meta_data.get(&0))
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    // payment
    match crate::apis::updates::payment_controller::icp_transfer_handler(
        post_data.amt.clone(),
        post_data.owner.clone(),
        meta_data.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE],
    )
    .await
    {
        Ok(val) => {
            let new_noti = crate::models::types::PurchasePostBody {
                ledger_block: val,
                post_id: post_id,
            };

            crate::with_write_state(|state| {
                // increasing create token amount
                let mut creator = state.account.get(&post_data.owner).unwrap();

                creator.token_amount = creator.token_amount + post_data.amt;
                state.account.insert(post_data.owner, creator);

                match state.purchased_post.get(&ic_cdk::api::caller()) {
                    Some(mut notis) => {
                        notis.posts.push(new_noti);
                        state.purchased_post.insert(ic_cdk::api::caller(), notis);
                        Ok(())
                    }
                    None => {
                        state.purchased_post.insert(
                            ic_cdk::api::caller(),
                            crate::PurchasedPosts {
                                posts: vec![new_noti],
                            },
                        );
                        Ok(())
                    }
                }
            })
        }
        Err(err) => return Err(err),
    }
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_purchase_media(
    post_id: core::types::PostId,
    media_id: core::types::MediaID,
    canister_id: candid::Principal,
) -> core::types::Response {
    // to validate canister id
    crate::utils::guards::validate_post_canister(canister_id)?;

    // checking if user already owns the media
    crate::with_read_state(
        |state| match state.purchased_media.get(&ic_cdk::api::caller()) {
            Some(pos) => {
                let medias: Vec<core::types::MediaID> =
                    pos.medias.iter().map(|e| e.post_id.clone()).collect();

                if medias.contains(&media_id) {
                    return Err(String::from(core::constants::WARNING_ALREADY_PURCHASED));
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        },
    )
    .map_err(|err| return format!("{}", err))?;

    // get post data (owner and price)
    let post_data = kaires::call_inter_canister::<
        core::types::SinglePurchaseArgs,
        core::types::PurchaseUserMedia,
    >(
        core::constants::FUNCTION_GET_MEDIA_PRICE,
        core::types::SinglePurchaseArgs {
            created_by: ic_cdk::api::caller(),
            media_id: media_id.clone(),
            post_id,
        },
        canister_id,
    )
    .await
    .expect(core::constants::ERROR_FAILED_INTER_CANISTER);

    let meta_data = crate::with_read_state(|state| state.canister_meta_data.get(&0))
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    // payment
    match crate::apis::updates::payment_controller::icp_transfer_handler(
        post_data.amt.clone(),
        post_data.owner.clone(),
        meta_data.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE],
    )
    .await
    {
        Ok(val) => {
            let new_purchase = crate::models::types::PurchaseMediaBody {
                ledger_block: val,
                post_id: media_id,
            };

            crate::with_write_state(|state| {
                // increasing create token amount
                let mut creator = state.account.get(&post_data.owner).unwrap();

                creator.token_amount = creator.token_amount + post_data.amt;
                state.account.insert(post_data.owner, creator);

                match state.purchased_media.get(&ic_cdk::api::caller()) {
                    Some(mut posts) => {
                        posts.medias.push(new_purchase);
                        state.purchased_media.insert(ic_cdk::api::caller(), posts);
                        Ok(())
                    }
                    None => {
                        state.purchased_media.insert(
                            ic_cdk::api::caller(),
                            crate::PurchasedMedia {
                                medias: vec![new_purchase],
                            },
                        );

                        Ok(())
                    }
                }
            })
        }
        Err(err) => return Err(err),
    }
}
