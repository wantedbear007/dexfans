use crate::utils::guards::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!(
        "Hello, {}! from {}",
        name,
        core::constants::ESSENTIALS_APP_NAME
    )
}

// debug
// #[ic_cdk::query]
// fn api_total_posts() -> u128 {
//     crate::with_read_state(|state| state.post_counter)
// }

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
    page: core::types::Pagination,
) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            if &pos.creator_id == &user_id && pos.post_status == core::types::PostStatus::Published
            {
                all_posts.push(crate::models::post::Post {
                    like_count: pos.likes.len(),
                    views_count: pos.views.len(),
                    ..pos.clone()
                });
            }
        }

        all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Pagination logic
        let ending = all_posts.len();

        if ending == 0 {
            return all_posts;
        }

        let start = page.start as usize;
        let end = page.end as usize;
        if start < ending {
            let end = end.min(ending);
            return all_posts[start..end].to_vec();
        }

        Vec::new()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_by_status(
    args: crate::models::post::GetByPostStatusArgs,
) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            if pos.post_status == args.status && pos.creator_id == ic_cdk::api::caller() {
                all_posts.push(pos);
            }
        }
        all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let ending = all_posts.len();

        if ending == 0 {
            return all_posts;
        }

        let start = args.pagination.start as usize;
        let end = args.pagination.end as usize;
        if start < ending {
            let end = end.min(ending);
            return all_posts[start..end].to_vec();
        }

        Vec::new()
    })

    // Vec::new()
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_latest_posts(page: core::types::Pagination) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            if pos.post_status == core::types::PostStatus::Published {
                all_posts.push(crate::models::post::Post {
                    like_count: pos.likes.len(),
                    views_count: pos.views.len(),
                    ..pos.clone()
                });
            }
        }

        all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Pagination logic
        let ending = all_posts.len();

        if ending == 0 {
            return all_posts;
        }

        let start = page.start as usize;
        let end = page.end as usize;
        if start < ending {
            let end = end.min(ending);
            return all_posts[start..end].to_vec();
        }

        Vec::new()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
async fn api_get_my_posts(args: core::types::Pagination) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            if &pos.creator_id == &ic_cdk::api::caller() {
                all_posts.push(crate::models::post::Post {
                    like_count: pos.likes.len(),
                    views_count: pos.views.len(),
                    ..pos
                });
            }
        }
        all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let ending = all_posts.len();

        if ending == 0 {
            return all_posts;
        }

        let start = args.start as usize;
        let end = args.end as usize;
        if start < ending {
            let end = end.min(ending);

            return all_posts[start..end].to_vec();
        }

        Vec::new()
    })
}

#[candid::candid_method(update)]
#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_get_subscribed_posts(page: core::types::Pagination) -> Vec<crate::models::post::Post> {
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
            all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

            // Pagination logic
            let ending = all_posts.len();

            if ending == 0 {
                return all_posts;
            }

            let start = page.start as usize;
            let end = page.end as usize;
            if start < ending {
                let end = end.min(ending);
                return all_posts[start..end].to_vec();
            }

            all_posts
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

            all_comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

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
