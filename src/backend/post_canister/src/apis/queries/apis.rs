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
fn api_post_by_user_id(
    user_id: candid::Principal,
    args: core::types::PaginationArgs,
) -> Vec<crate::models::post::Post> {
    // core::functions::input_validator::<core::types::Pagination>(&page).unwrap();
    crate::with_read_state(|state| {
        crate::utils::functions::filter_posts(
            args,
            state,
            core::types::PostStatus::Published,
            true,
            user_id,
        )
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_by_status(
    args: crate::models::post::GetByPostStatusArgs,
) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        crate::utils::functions::filter_posts(
            args.pagination,
            state,
            args.status,
            true,
            ic_cdk::api::caller(),
        )
    })
}

// DO NOT REMOVE
// #[ic_cdk::query(guard = guard_prevent_anonymous)]
// pub fn api_get_latest_posts(args: core::types::PaginationArgs) -> Vec<crate::models::post::Post> {
//     crate::with_read_state(|state| crate::utils::functions::filter_posts(args, state, core::types::PostStatus::Published, false, ic_cdk::api::caller()))
// }

// Yet to be added in main stream (Test functions)
#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_post_ids() -> std::collections::HashSet<core::types::PostId> {
    crate::with_read_state(|state| state.posts.iter().map(|(id, _)| id).collect())
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_latest_posts(
    args: std::collections::HashSet<core::types::PostId>,
) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        state
            .posts
            .iter()
            .filter_map(
                |(id, post)| {
                    if args.contains(&id) {
                        Some(post)
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
async fn api_get_my_posts(args: core::types::PaginationArgs) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        crate::utils::functions::filter_posts(
            args.clone(),
            state,
            args.post_status
                .unwrap_or(core::types::PostStatus::Published),
            true,
            ic_cdk::api::caller(),
        )
    })
}

#[candid::candid_method(update)]
#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_get_subscribed_posts(
    args: core::types::PaginationArgs,
) -> Vec<crate::models::post::Post> {
    // core::functions::input_validator::<core::types::Pagination>(&page).unwrap();
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
        Ok(val) => {
            let mut all_posts: Vec<crate::models::post::Post> = Vec::new();
            crate::with_read_state(|state| {
                let acc_set: std::collections::HashSet<_> = val.iter().collect();

                for (_, post) in state.posts.iter() {
                    if acc_set.contains(&post.creator_id)
                        && post.post_status == core::types::PostStatus::Published
                    {
                        all_posts.push(crate::models::post::Post {
                            like_count: post.likes.len(),
                            views_count: post.views.len(),
                            ..post.clone()
                        });
                    }
                }
            });

            let page = args.page.max(1);

            let limit = args.limit.min(100);
            let offset = (page - 1) * limit;
            all_posts.reverse();

            all_posts
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect()
        }
        Err(err) => {
            ic_cdk::println!("{}", err.to_string()); // for debug only

            return Vec::new();
        }
    }
}

// get comments
#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_comments(
    args: crate::models::post::PostPagination,
) -> Vec<crate::models::comment::CommentBody> {
    crate::with_read_state(|state| match state.comments.get(&args.post_id) {
        Some(com) => {
            let mut all_comments: Vec<crate::models::comment::CommentBody> = Vec::new();

            for comment_body in com.comments.iter() {
                all_comments.push(comment_body.to_owned());
            }

            all_comments.reverse();
            // all_comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

            let ending = all_comments.len();

            if ending == 0 {
                return all_comments;
            }

            let start = args.start as usize;
            let end = args.end as usize;
            if start < ending {
                let end = end.min(ending);

                return all_comments[start..end].to_vec();
            }

            Vec::new()
        }
        None => Vec::new(),
    })
}
