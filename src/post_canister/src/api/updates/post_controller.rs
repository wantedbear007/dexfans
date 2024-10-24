// to create post
pub(super) async fn controller_create_post(args: crate::CreatePostArgs) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(val) => {
            let post_id = state.post_counter;
            state.post_counter += 1;

            state.posts.insert(
                post_id.clone(),
                crate::Post {
                    content: args.content,
                    image: args.image,
                    post_type: args.post_type,
                    price: args.price,
                    video: args.video,
                    created_at: ic_cdk::api::time(),
                    creator_id: val.user_id,
                    post_id,
                    ..Default::default()
                },
            );
            Ok(())
        }
        None => {
            return Err(String::from(
                dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED,
            ))
        }
    })
}

// to update post
pub(super) fn controller_update_post(
    args: crate::models::post::UpdatePostArgs,
) -> Result<(), String> {
    crate::with_write_state(|state| match state.posts.get(&args.id) {
        Some(mut val) => {
            if &val.content != &args.content {
                val.content = args.content;
            }
            state.posts.insert(val.post_id, val);

            Ok(())
        }
        None => return Err(String::from(dexfans_types::constants::ERROR_POST_NOT_EXIST)),
    })
}

