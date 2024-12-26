use crate::utils::guards::*;

use super::post_controller::{
    controller_comment_on_post, controller_delete_post, controller_like_unlike_post,
    controller_update_post,
};

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_create_new_post(
    args: crate::models::post::CreatePostArgs,
) -> Result<core::types::PostId, String> {
    // validating input
    core::functions::input_validator(&args)?;
    super::post_controller::controller_create_post(args)
        .await
        .map_err(|err| return format!("{}", err))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_update_post(args: crate::models::post::UpdatePostArgs) -> Result<String, String> {
    core::functions::input_validator(&args)?;
    controller_update_post(args).map_err(|err| format!("{}", err))?;
    Ok(String::from(core::constants::SUCESSS_POST_UPDATED))
}

// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// async fn api_save_post(args: crate::models::post::UpdatePostArgs) -> Result<String, String> {
//     controller_save_post(args).map_err(|err| format!("{}", err))?;
//     Ok(String::from(core::constants::SUCCESS_POST_SAVED))
// }

// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// async fn api_archive_post(args: crate::models::post::UpdatePostArgs) -> Result<String, String> {
//     controller_archive_post(args).map_err(|err| format!("{}", err))?;
//     Ok(String::from(core::constants::SUCCESS_POST_ARCHIVED))
// }

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_delete_post(post_id: core::types::PostId) -> Result<String, String> {
    controller_delete_post(post_id).map_err(|err| format!("{}", err))?;
    Ok(String::from(core::constants::SUCCESS_POST_DELETED))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_like_unlike_post(post_id: u128) -> Result<String, String> {
    let is_liked = controller_like_unlike_post(post_id).map_err(|err| format!("{}", err))?;
    let action = if is_liked { "liked" } else { "unliked" };
    Ok(format!("Post successfully {}", action))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn api_comment_on_post(post_id: core::types::PostId, content: String) -> Result<String, String> {
    if content.len() > core::constants::VALIDATOR_COMMENT_SIZE as usize {
        return Err(String::from(
            "Validator Error: Comment size is not applicable",
        ));
    }

    controller_comment_on_post(post_id, content).map_err(|err| format!("{}", err))?;
    Ok(String::from("Comment added successfully"))
}
