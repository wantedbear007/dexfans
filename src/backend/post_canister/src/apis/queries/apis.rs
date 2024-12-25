use crate::utils::guards::*;

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_search_post(args: String) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        state
            .posts
            .iter()
            .filter_map(|(_, post)| {
                if post.content.to_lowercase().contains(&args.to_lowercase()) {
                    Some(post)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
fn api_get_post_by_id(post_id: core::types::PostId) -> Result<crate::models::post::Post, String> {
    crate::with_write_state(|state| match state.posts.get(&post_id) {
        Some(mut post) => {
            post.views.push(ic_cdk::api::caller());
            state.posts.insert(post.post_id, post.clone());
            Ok(crate::models::post::Post {
                like_count: post.likes.len(),
                views_count: post.views.len(),
                ..post
            })
        }
        None => return Err(String::from(core::constants::ERROR_POST_NOT_EXIST)),
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_user_post_ids(args: candid::Principal) -> Vec<core::types::PostId> {
    crate::with_read_state(|state| {
        state
            .posts
            .iter()
            .filter_map(|(id, post)| {
                if post.creator_id == args && post.post_status == core::types::PostStatus::Published {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    })
}

// #[ic_cdk::query(guard = guard_prevent_anonymous)]
// fn api_post_by_user_id(
//     user_id: candid::Principal,
//     args: Vec<core::types::PostId>,
// ) -> Vec<crate::models::post::Post> {
//     crate::with_read_state(|state| {
//         crate::utils::functions::selective_post(
//             args,
//             state,
//             core::types::PostStatus::Published,
//             true,
//             user_id,
//         )
//     })
// }
#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_post_by_user_id(
    user_id: candid::Principal,
    args: core::types::PaginationArgs0,
) -> Vec<crate::models::post::Post> {
    // core::functions::input_validator::<core::types::Pagination>(&page).unwrap();
    crate::with_read_state(|state| crate::utils::functions::filter_posts(args, state, core::types::PostStatus::Published, true, user_id))
 
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_by_status(
    args: crate::models::post::GetByPostStatusArgs,
) -> Vec<crate::models::post::Post> {

    crate::with_read_state(|state| crate::utils::functions::filter_posts(args.pagination, state, args.status, true, ic_cdk::api::caller()))
}

// #[ic_cdk::query(guard = guard_prevent_anonymous)]
// fn api_get_post_by_status(
//     args: crate::models::post::GetByPostStatusArgs,
// ) -> Vec<crate::models::post::Post> {
//     crate::with_read_state(|state| {
//         crate::utils::functions::selective_post(
//             args.ids,
//             state,
//             args.status,
//             true,
//             ic_cdk::api::caller(),
//         )
//     })
// }

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_post_ids() -> Vec<core::types::PostId> {
    let mut all_post_ids: Vec<core::types::PostId> = Vec::new();

    crate::with_read_state(|state| {
        for (id, pos) in state.posts.iter() {
            if pos.post_status == core::types::PostStatus::Published {

                all_post_ids.push(id);
            }
        }
    });

    all_post_ids.reverse();
    all_post_ids
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_comment_ids(args: core::types::PostId) -> Vec<core::types::CommentId> {
    let mut all_comments = crate::with_read_state(|state| match state.comments.get(&args) {
        Some(com) => com.comments.iter().map(|val| val.comment_id).collect(),
        None => vec![],
    });

    all_comments.reverse();
    all_comments
}

// #[ic_cdk::query(guard = guard_prevent_anonymous)]
// fn api_get_latest_posts(
//     args: std::collections::HashSet<core::types::PostId>,
// ) -> Vec<crate::models::post::Post> {
//     crate::with_read_state(|state| {
//         state
//             .posts
//             .iter()
//             .filter_map(
//                 |(id, post)| {
//                     if args.contains(&id) {
//                         Some(post)
//                     } else {
//                         None
//                     }
//                 },
//             )
//             .collect::<Vec<_>>()
//     })
// }
#[ic_cdk::query(guard = guard_prevent_anonymous)]
async fn api_get_post(args: Vec<core::types::PostId>) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        crate::utils::functions::selective_post(
            args,
            state,
            core::types::PostStatus::Published,
            false,
            ic_cdk::api::caller(),
        )
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
async fn api_get_my_posts(args: core::types::PaginationArgs) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        crate::utils::functions::selective_post(
            args.ids,
            state,
            args.post_status
                .unwrap_or(core::types::PostStatus::Published),
            true,
            ic_cdk::api::caller(),
        )
    })
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_subscribed_posts_ids() -> std::collections::HashSet<core::types::PostId> {
    match kaires::call_inter_canister::<
        candid::Principal,
        std::collections::HashSet<candid::Principal>,
    >(
        "ic_get_subscribed_list",
        ic_cdk::api::caller(),
        crate::utils::functions::get_parent_canister()
            .expect(core::constants::ERROR_FAILED_INTER_CANISTER),
    )
    .await
    {
        Ok(val) => crate::with_read_state(|state| {
            let subscribed_acc: std::collections::HashSet<_> = val.iter().collect();
            let mut all_post_ids: std::collections::HashSet<core::types::CommentId> =
                std::collections::HashSet::new();

            for (id, post) in state.posts.iter() {
                if subscribed_acc.contains(&post.creator_id)
                    && post.post_status == core::types::PostStatus::Published
                {
                    all_post_ids.insert(id);
                }
            }

            all_post_ids
        }),
        Err(err) => ic_cdk::trap(err.as_str()),
    }
}

// #[candid::candid_method(update)]
// #[ic_cdk::update(guard = guard_prevent_anonymous)]
// async fn api_get_subscribed_posts(
// ) -> Vec<crate::models::post::Post> {
//     // core::functions::input_validator::<core::types::Pagination>(&page).unwrap();
//     match kaires::call_inter_canister::<
//         candid::Principal,
//         std::collections::HashSet<candid::Principal>,
//     >(
//         "ic_get_subscribed_list",
//         ic_cdk::api::caller(),
//         crate::utils::functions::get_parent_canister()
//             .expect(core::constants::ERROR_FAILED_INTER_CANISTER),
//     )
//     .await
//     {
//         Ok(val) => {
//             let mut all_posts: Vec<crate::models::post::Post> = Vec::new();
//             crate::with_read_state(|state| {
//                 let acc_set: std::collections::HashSet<_> = val.iter().collect();

//                 for (_, post) in state.posts.iter() {
//                     if acc_set.contains(&post.creator_id)
//                         && post.post_status == core::types::PostStatus::Published
//                     {
//                         all_posts.push(crate::models::post::Post {
//                             like_count: post.likes.len(),
//                             views_count: post.views.len(),
//                             ..post.clone()
//                         });
//                     }
//                 }
//             });

//             // let page = args.page.max(1);

//             // let limit = args.limit.min(100);
//             // let offset = (page - 1) * limit;
//             all_posts.reverse();
//             all_posts
//             // all_posts
//             //     .into_iter()
//             //     .skip(offset as usize)
//             //     .take(limit as usize)
//             //     .collect()
//         }
//         Err(err) => {
//             ic_cdk::println!("{}", err.to_string()); // for debug only

//             return Vec::new();
//         }
//     }
// }

// get comments
#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_comments(
    ids: Vec<core::types::CommentId>,
    post_id: core::types::PostId,
) -> Vec<crate::models::comment::CommentBody> {
    let mut all_comments: Vec<crate::models::comment::CommentBody> = Vec::new();

    crate::with_read_state(|state| match state.comments.get(&post_id) {
        Some(com) => {
            for comment in com.comments.iter() {
                if ids.contains(&comment.comment_id) {
                    all_comments.push(comment.clone());
                }
            }
        }
        None => ic_cdk::trap(core::constants::ERROR_POST_NOT_EXIST),
    });

    all_comments

    // vec![]

    // crate::with_read_state(|state| match state.comments.get(&args.post_id) {
    //     Some(com) => {
    //         let mut all_comments: Vec<crate::models::comment::CommentBody> = Vec::new();

    //         for comment_body in com.comments.iter() {
    //             all_comments.push(comment_body.to_owned());
    //         }

    //         all_comments.reverse();
    //         // all_comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    //         let ending = all_comments.len();

    //         if ending == 0 {
    //             return all_comments;
    //         }

    //         let start = args.start as usize;
    //         let end = args.end as usize;
    //         if start < ending {
    //             let end = end.min(ending);

    //             return all_comments[start..end].to_vec();
    //         }

    //         Vec::new()
    //     }
    //     None => Vec::new(),
    // })
}
