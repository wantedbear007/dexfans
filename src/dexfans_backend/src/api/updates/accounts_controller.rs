pub fn controller_create_account(args: crate::models::user::UserInputArgs) -> Result<(), String> {
    crate::with_write_state(|state| {
        // checking if user already exists
        if state.account.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                dexfans_types::constants::WARNING_ACCOUNT_EXISTS,
            ));
        }

        // to retrieve canister meta data
        let canister_meta_data = state
            .canister_meta_data
            .get(&0)
            .expect(dexfans_types::constants::ERROR_FAILED_CANISTER_DATA)
            .canister_ids;

        state.account.insert(
            ic_cdk::api::caller(),
            crate::models::user::UserProfile {
                avatar: args.avatar,
                bio: args.bio,
                cover_image: args.cover_image,
                collects: Vec::new(),
                likes: Vec::new(),
                posts: Vec::new(),
                created_at: ic_cdk::api::time(),
                is_bot: false,
                membership: crate::models::types::Membership::Guest,
                subscribers: Vec::new(),
                subscribing: Vec::new(),
                user_id: ic_cdk::api::caller(),
                username: args.username,
                asset_canister_id: canister_meta_data
                    [&dexfans_types::constants::ESSENTIAL_ASSET_CANISTER_ID_CODE],
                post_canister_id: canister_meta_data
                    [&dexfans_types::constants::ESSENTIAL_POST_CANISTER_ID_CODE],
            },
        );
        Ok(())
    })
}
