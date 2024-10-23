use crate::utils::guards::*;

use super::post_controller::controller_update_post;

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub(self) async fn api_create_new_post(
    args: crate::models::post::CreatePostArgs,
) -> Result<String, String> {
    let _x = super::post_controller::controller_create_post(args)
        .await
        .map_err(|err| return format!("{}", err));
    Ok(String::from(dexfans_types::constants::SUCCESS_POST_CREATED))
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub(self) async fn api_update_post(
    args: crate::models::post::UpdatePostArgs,
) -> Result<String, String> {
    controller_update_post(args).map_err(|err| format!("{}", err))?;
    Ok(String::from(dexfans_types::constants::SUCESSS_POST_UPDATED))
}



// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// pub(self) async fn delete_post(id: &u12)

// pending apis
// delete post
// 