use crate::models::post::CreatePostArgs;


#[ic_cdk::update]
async fn api_create_new_post(args: CreatePostArgs) -> Result<String, String> {
    super::post_controller::controller_create_post(args).await;
    Ok(String::from(crate::utils::constants::SUCCESS_POST_CREATED))
}


