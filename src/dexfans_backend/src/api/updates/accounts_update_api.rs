use crate::utils::guards::*;

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub fn api_create_account(args: crate::models::user::UserInputArgs) -> Result<String, String> {
    super::accounts_controller::controller_create_account(crate::models::user::UserProfile {
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
    })
    .map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_ACCOUNT_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_ACCOUNT_CREATED,
    ))
}
