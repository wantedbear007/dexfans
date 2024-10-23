use crate::with_write_state;

pub async fn controller_create_account(
    args: crate::models::types::UserInputArgs,
) -> Result<(), String> {
    match crate::with_write_state(|state| {
        // checking if user already exists
        if state.account.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                dexfans_types::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
            // to retrieve canister meta data
            let canister_meta_data = state
                .canister_meta_data
                .get(&0)
                .expect(dexfans_types::constants::ERROR_FAILED_CANISTER_DATA)
                .canister_ids;

            // add user details to stable storage
            state.account.insert(
                ic_cdk::api::caller(),
                crate::models::types::UserProfile {
                    avatar: args.avatar,
                    bio: args.bio,
                    all_post_canisters: std::collections::HashSet::from([canister_meta_data
                        [&dexfans_types::constants::ESSENTIAL_POST_CANISTER_ID_CODE]]),
                    cover_image: args.cover_image,
                    collects: Vec::new(),
                    likes: Vec::new(),
                    posts: Vec::new(),
                    created_at: ic_cdk::api::time(),
                    is_bot: false,
                    membership: dexfans_types::types::Membership::Guest,
                    subscribers: Vec::new(),
                    subscribing: Vec::new(),
                    user_id: ic_cdk::api::caller(),
                    username: args.username.clone(),
                    asset_canister_id: canister_meta_data
                        [&dexfans_types::constants::ESSENTIAL_ASSET_CANISTER_ID_CODE],
                    active_post_canister: canister_meta_data
                        [&dexfans_types::constants::ESSENTIAL_POST_CANISTER_ID_CODE],
                },
            );
        }

        Ok(())
    }) {
        Ok(()) => {
            match update_profile(crate::models::types::UserProfileInterCanister {
                user_id: ic_cdk::api::caller(),
                username: args.username,
                ..Default::default()
            })
            .await
            {
                Ok(()) => Ok(()),
                Err(err) => {
                    // roll back if the call fails
                    with_write_state(|state| state.account.remove(&ic_cdk::api::caller()));
                    Err(err)
                }
            }
        }
        Err(err) => Err(err),
    }

    // Ok(())

    // add profile details to post canister
    // match update_profile(crate::models::types::UserProfileInterCanister {
    //     user_id: ic_cdk::api::caller(),
    //     username: args.username,
    //     ..Default::default()
    // })
    // .await
    // {
    //     Ok(()) => Ok(()),
    //     Err(err) => {
    //         // roll back if the call fails
    //         with_write_state(|state| state.account.remove(&ic_cdk::api::caller()));
    //         Err(err)
    //     } // add roll back
    // }
}

pub async fn update_profile(
    args: crate::models::types::UserProfileInterCanister,
) -> Result<(), String> {
    match kaires::call_inter_canister::<crate::models::types::UserProfileInterCanister, ()>(
        "admin_add_user_profile",
        args,
        crate::utils::functions::get_post_canister()
            .expect(dexfans_types::constants::ERROR_FAILED_INTER_CANISTER),
    )
    .await
    {
        Ok(()) => (),
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}
