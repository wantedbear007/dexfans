use crate::utils::guards::*;

use super::post_controller::{
    controller_comment_on_post, controller_delete_post, controller_like_unlike_post,
    controller_update_post,
};


#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub(self) async fn api_create_new_post(
    args: crate::models::post::CreatePostArgs,
) -> Result<String, String> {
    let _x = super::post_controller::controller_create_post(args)
        .await
        .map_err(|err| return format!("{}", err));
    Ok(String::from(core::constants::SUCCESS_POST_CREATED))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub(self) async fn api_update_post(
    args: crate::models::post::UpdatePostArgs,
) -> Result<String, String> {
    controller_update_post(args).map_err(|err| format!("{}", err))?;
    Ok(String::from(core::constants::SUCESSS_POST_UPDATED))
}

// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// pub(self) async fn delete_post(id: &u12)

// pending apis
// delete post
//

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_delete_post(post_id: u128) -> Result<String, String> {
    controller_delete_post(post_id).map_err(|err| format!("{}", err))?;
    Ok(String::from(core::constants::SUCCESS_POST_DELETED))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_like_unlike_post(post_id: u128) -> Result<String, String> {
    let is_liked = controller_like_unlike_post(post_id).map_err(|err| format!("{}", err))?;
    let action = if is_liked { "liked" } else { "unliked" };
    Ok(format!("Post successfully {}", action))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub fn api_comment_on_post(
    post_id: u128,
    content: String,
    image: Option<String>,
) -> Result<String, String> {
    controller_comment_on_post(post_id, content, image).map_err(|err| format!("{}", err))?;
    Ok(String::from("Comment added successfully"))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_subscribe_account(to: candid::Principal) -> Result<(), String> {
    super::subscription_controllers::controller_subscribe(to).await
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_unsubscribe_account(to: candid::Principal) -> Result<(), String> {
    super::subscription_controllers::controller_unsubscribe(to).await
}

