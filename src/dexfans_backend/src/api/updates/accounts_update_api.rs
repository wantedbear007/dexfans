use candid::Principal;

use crate::{
    utils::{functions::get_post_canister, guards::*},
    with_write_state, STATE,
};

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub async fn api_create_account(
    args: crate::models::types::UserInputArgs,
) -> Result<String, String> {
    super::accounts_controller::controller_create_account(args)
        .await
        .map_err(|err| {
            format!(
                "{}{}",
                dexfans_types::constants::ERROR_ACCOUNT_ERROR,
                err.to_string()
            )
        })?;

    Ok(String::from(
        dexfans_types::constants::SUCCESS_ACCOUNT_CREATED,
    ))
}

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub fn api_update_profile(
    // user_id: Principal,
    args: crate::models::types::UserInputArgs,
) -> Result<(), String> {
    with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(val) => {
            state.account.insert(
                ic_cdk::api::caller(),
                crate::models::types::UserProfile {
                    username: args.username,
                    bio: args.bio,
                    avatar: args.avatar,
                    cover_image: args.cover_image,
                    ..val
                },
            );

            Ok(())
        }
        None => return Err(String::from(dexfans_types::constants::ERROR_PROFILE_UPDATE)),
    })
}

#[ic_cdk::update]
pub fn api_add_post_id_to_user(user_id: Principal, post_id: u128) -> Result<(), String> {
    STATE.with(|state| {
        let mut app_state = state.borrow_mut();

        if let Some(mut user_profile) = app_state.account.remove(&user_id) {
            user_profile.posts.push(post_id); // Modify the profile
            app_state.account.insert(user_id, user_profile); // Reinsert the modified profile
            Ok(())
        } else {
            Err("User not found.".to_string())
        }
    })
}

#[ic_cdk::update]
pub fn api_update_user_likes(
    user_id: Principal,
    post_id: u128,
    is_liked: bool,
) -> Result<(), String> {
    STATE.with(|state| {
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
