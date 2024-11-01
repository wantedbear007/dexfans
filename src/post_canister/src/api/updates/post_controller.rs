// to create post
pub(super) async fn controller_create_post(args: crate::CreatePostArgs) -> Result<(), String> {
    match kaires::call_inter_canister::<core::types::ICAddPostCanisterProfile, ()>(
        "admin_profile_post_canister",
        core::types::ICAddPostCanisterProfile {
            caller: ic_cdk::api::caller(),
            post_canister: ic_cdk::api::id(),
        },
        crate::utils::functions::get_parent_canister()
            .expect(core::constants::ERROR_FAILED_CANISTER_DATA),
    )
    .await
    {
        Ok(()) => {
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
                None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
            })
        }
        Err(err) => {
            return Err(err);
        }
    }
    // crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
    //     Some(val) => {
    //         let post_id = state.post_counter;
    //         state.post_counter += 1;

    //         state.posts.insert(
    //             post_id.clone(),
    //             crate::Post {
    //                 content: args.content,
    //                 image: args.image,
    //                 post_type: args.post_type,
    //                 price: args.price,
    //                 video: args.video,
    //                 created_at: ic_cdk::api::time(),
    //                 creator_id: val.user_id,
    //                 post_id,
    //                 ..Default::default()
    //             },
    //         );

    //         Ok(())
    //     }
    //     None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    // })
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
        None => return Err(String::from(core::constants::ERROR_POST_NOT_EXIST)),
    })
}

//to delete post
pub(super) fn controller_delete_post(post_id: u128) -> Result<(), String> {
    crate::with_write_state(|state| {
        if state.posts.remove(&post_id).is_some() {
            Ok(())
        } else {
            Err(String::from(core::constants::ERROR_POST_NOT_EXIST))
        }
    })
}

pub(super) fn controller_like_unlike_post(post_id: u128) -> Result<bool, String> {
    let caller = ic_cdk::api::caller();

    crate::with_write_state(|state| {
        if let Some(mut post) = state.posts.remove(&post_id) {
            let is_liked = post.likes.contains(&caller);

            if is_liked {
                post.likes.retain(|&p| p != caller);
            } else {
                post.likes.push(caller);
            }

            state.posts.insert(post_id, post);
            Ok(!is_liked)
        } else {
            Err("Post not found.".to_string())
        }
    })
}

pub(super) fn controller_comment_on_post(
    post_id: u128,
    content: String,
    image: Option<String>,
) -> Result<(), String> {
    let caller = ic_cdk::api::caller();

    crate::with_write_state(|state| {
        if let Some(mut post) = state.posts.remove(&post_id) {
            let comment_id = state.comment_counter + 1;
            state.comment_counter = comment_id;

            let now_ms = ic_cdk::api::time() / 1_000_000;
            let new_comment = crate::models::comment::Comment {
                comment_id,
                content,
                image,
                creator_id: caller, // Set the creator ID to the caller
                created_at: now_ms,
            };

            state.comments.insert(comment_id, new_comment);
            post.comments.push(comment_id); // Add the new comment ID to the post

            state.posts.insert(post_id, post); // Reinsert the updated post
            Ok(())
        } else {
            Err("Post not found.".to_string())
        }
    })
}
