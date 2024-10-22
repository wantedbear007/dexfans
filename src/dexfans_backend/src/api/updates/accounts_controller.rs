use crate::{utils::functions::get_asset_canister, with_write_state};

pub fn controller_create_account(
    args: crate::models::user::UserInputArgs,
) -> Result<(), String> {
    with_write_state(|state| {

      // checking if user already exists
        if state.account.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
        }
        state.account.insert(ic_cdk::api::caller(),crate::models::user::UserProfile {
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
            asset_canister_id: get_asset_canister().unwrap()
        });
        Ok(())
    })
}


