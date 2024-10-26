// to subscribe
pub(super) async fn controller_subscribe(to: candid::Principal) -> Result<(), String> {
    let parent_canister_id = crate::utils::functions::get_parent_canister()
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(_val) => {
            ic_cdk::spawn(async move {
                let _ =
                    kaires::call_inter_canister::<core::types::SubscribeAccountIC, ()>(
                        "ic_subscribe_account",
                        core::types::SubscribeAccountIC {
                            subscribed_by: ic_cdk::api::caller(),
                            subscribed_to: to,
                        },
                        parent_canister_id,
                    )
                    .await
                    .map_err(|err| {
                        return format!("{}", err);
                    });
            });

            Ok(())
        }
        None => {
            return Err(String::from(
                core::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

pub(super) async fn controller_unsubscribe(to: candid::Principal) -> Result<(), String> {
    let parent_canister_id = crate::utils::functions::get_parent_canister()
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA);

    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(_val) => {
            ic_cdk::spawn(async move {
                let _ =
                    kaires::call_inter_canister::<core::types::UnsubscribeAccountIC, ()>(
                        "ic_unsubscribe_account",
                        core::types::UnsubscribeAccountIC {
                            unsubscribed_by: ic_cdk::api::caller(),
                            unsubscribed_to: to,
                        },
                        parent_canister_id,
                    )
                    .await
                    .map_err(|err| format!("{}", err));
            });

            Ok(())
        }
        None => Err(String::from(
            core::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}

// use candid::Principal;
// use ic_cdk::call;

// use crate::{models::notification::Notification, STATE};

// use super::post::current_timestamp;

// #[ic_cdk::update]
// async fn subscribe(
//     // dexfans_backend_canister_id: Principal,
//     // subscriber: Principal,
//     target_user: Principal,
// ) -> Result<(), String> {

//     let caller_principal: Principal = ic_cdk::api::caller();

//     let dexfans_backend_canister = crate::utils::functions::get_canister_meta_data().unwrap();

//     let (subscriber_profile,): (Result<crate::models::user::UserProfile, String>,) = call(
//         dexfans_backend_canister.parent_canister,
//         "api_get_user_account",
//         // (subscriber,),
//         (caller_principal.clone(),),

//     )
//     .await
//     .map_err(|_| "Failed to fetch subscriber profile".to_string())?;

//     let (target_profile,): (Result<crate::models::user::UserProfile, String>,) = call(
//         dexfans_backend_canister.parent_canister,
//         "api_get_user_account",
//         (target_user,),
//     )
//     .await
//     .map_err(|_| "Failed to fetch target user profile".to_string())?;

//     if let (Ok(mut subscriber_profile), Ok(mut target_profile)) =
//         (subscriber_profile, target_profile)
//     {
//         // if !target_profile.subscribers.contains(&subscriber) {
//             if !target_profile.subscribers.contains(&caller_principal) {

//             // target_profile.subscribers.push(subscriber);
//             target_profile.subscribers.push(caller_principal.clone());

//         }

//         if !subscriber_profile.subscribing.contains(&target_user) {
//             subscriber_profile.subscribing.push(target_user);
//         }

//         let (update_result_subscriber,): (Result<(), String>,) = call(
//             dexfans_backend_canister.parent_canister,
//             "update_user_profile",
//             // (subscriber, subscriber_profile.clone()),
//             (caller_principal, subscriber_profile.clone()),

//         )
//         .await
//         .map_err(|_| "Failed to update subscriber profile".to_string())?;

//         let (update_result_target,): (Result<(), String>,) = call(
//             dexfans_backend_canister.parent_canister,
//             "update_user_profile",
//             (target_user, target_profile.clone()),
//         )
//         .await
//         .map_err(|_| "Failed to update target user profile".to_string())?;

//         if update_result_subscriber.is_ok() && update_result_target.is_ok() {
//             Ok(())
//         } else {
//             Err("Failed to update one or both user profiles.".to_string())
//         }
//     } else {
//         Err("One or both users not found.".to_string())
//     }
// }

// #[ic_cdk::update]
// async fn unsubscribe(
//     dexfans_backend_canister_id: Principal,
//     subscriber: Principal,
//     target_user: Principal,
// ) -> Result<(), String> {
//     let (subscriber_profile,): (Result<crate::models::user::UserProfile, String>,) = call(
//         dexfans_backend_canister_id,
//         "api_get_user_account",
//         (subscriber,),
//     )
//     .await
//     .map_err(|_| "Failed to fetch subscriber profile".to_string())?;

//     let (target_profile,): (Result<crate::models::user::UserProfile, String>,) = call(
//         dexfans_backend_canister_id,
//         "api_get_user_account",
//         (target_user,),
//     )
//     .await
//     .map_err(|_| "Failed to fetch target user profile".to_string())?;

//     if let (Ok(mut subscriber_profile), Ok(mut target_profile)) =
//         (subscriber_profile, target_profile)
//     {
//         target_profile.subscribers.retain(|&s| s != subscriber);

//         subscriber_profile.subscribing.retain(|&s| s != target_user);

//         let (update_result_subscriber,): (Result<(), String>,) = call(
//             dexfans_backend_canister_id,
//             "update_user_profile",
//             (subscriber, subscriber_profile.clone()),
//         )
//         .await
//         .map_err(|_| "Failed to update subscriber profile".to_string())?;

//         let (update_result_target,): (Result<(), String>,) = call(
//             dexfans_backend_canister_id,
//             "update_user_profile",
//             (target_user, target_profile.clone()),
//         )
//         .await
//         .map_err(|_| "Failed to update target user profile".to_string())?;

//         if update_result_subscriber.is_ok() && update_result_target.is_ok() {
//             Ok(())
//         } else {
//             Err("Failed to update one or both user profiles.".to_string())
//         }
//     } else {
//         Err("One or both users not found.".to_string())
//     }
// }

// #[ic_cdk::update]
// async fn notify_subscribers(
//     dexfans_backend_canister_id: Principal,
//     user: Principal,
//     message: String,
// ) -> Result<(), String> {
//     let (user_profile,): (Result<crate::models::user::UserProfile, String>,) =
//         call(dexfans_backend_canister_id, "api_get_user_account", (user,))
//             .await
//             .map_err(|_| "Failed to fetch user profile".to_string())?;

//     if let Ok(user_profile) = user_profile {
//         let current_time = current_timestamp();
//         let notification = Notification {
//             recipient: user,
//             message: message.clone(),
//             created_at: current_time,
//         };

//         STATE.with(|state| {
//             let mut app_state = state.borrow_mut();
//             for subscriber in &user_profile.subscribers {
//                 app_state
//                     .notifications
//                     .entry(*subscriber)
//                     .or_insert_with(Vec::new)
//                     .push(notification.clone());
//             }
//         });

//         Ok(())
//     } else {
//         Err("User not found.".to_string())
//     }
// }
