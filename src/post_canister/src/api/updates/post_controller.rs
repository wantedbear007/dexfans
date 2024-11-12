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

pub(super) fn controller_comment_on_post(post_id: u128, content: String) -> Result<(), String> {
    crate::with_write_state(|state| match state.posts.get(&post_id) {
        Some(_) => {
            let comment_id = state.comment_counter + 1;
            state.comment_counter = comment_id;

            let mut comments = state
                .comments
                .get(&post_id)
                .expect(core::constants::ERROR_POST_NOT_EXIST);

            comments.comments.push(crate::models::comment::CommentBody {
                comment_id,
                content,
                created_at: ic_cdk::api::time(),
            });

            state.comments.insert(post_id, comments);

            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_POST_NOT_EXIST)),
    })
}
