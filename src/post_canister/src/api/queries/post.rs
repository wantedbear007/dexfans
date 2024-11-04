use crate::utils::guards::*;
use candid::Principal;
use ic_cdk::{api::time, call, query, update};

use crate::with_read_state;
use crate::{models::post::Post, STATE};

#[query]
pub fn api_get_post_by_id(post_id: u128) -> Option<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.posts.get(&post_id).map(|post| post.clone())
    })
}

#[query(guard=guard_prevent_anonymous)]
pub fn api_list_all_posts() -> Vec<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.get_all_posts()
    })
}

#[query(guard = guard_prevent_anonymous)]
pub fn api_post_by_user_id(
    user_id: candid::Principal,
    page: crate::models::types::Pagination,
) -> Vec<Post> {
    with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();
        for (_, pos) in state.posts.iter() {
            if &pos.creator_id == &user_id {
                all_posts.push(pos);
            }
        }

        // all_posts.sort_by(|a,b| b.created_at.cmp(&a.created_at));

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

    // STATE.with(|state| {
    //     let app_state = state.borrow();

    //     let mut all_posts: Vec<Post> = app_state.get_all_posts();
    //     all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    //     let batch_pagination = core::types::Pagination {
    //         page: page / 5,
    //         page_size: 50,
    //     };
    //     let batch_start = batch_pagination.page * batch_pagination.page_size;
    //     let batch_end = std::cmp::min(batch_start + batch_pagination.page_size, all_posts.len());

    //     let mut current_batch: Vec<Post> = all_posts[batch_start..batch_end].to_vec();

    //     let seed = time() as u64;
    //     let mut rng = StdRng::seed_from_u64(seed);
    //     current_batch.shuffle(&mut rng);

    //     let inner_pagination = core::types::Pagination {
    //         page: page % 5,
    //         page_size: 10,
    //     };
    //     let start = inner_pagination.page * inner_pagination.page_size;
    //     let end = std::cmp::min(start + inner_pagination.page_size, current_batch.len());

    //     current_batch[start..end].to_vec()
    // })
}

#[query(guard = guard_prevent_anonymous)]
pub fn api_get_latest_posts(page: crate::models::types::Pagination) -> Vec<Post> {
    with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            all_posts.push(pos);
        }

        // all_posts.sort_by(|a,b| b.created_at.cmp(&a.created_at));

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

    // STATE.with(|state| {
    //     let app_state = state.borrow();

    //     let mut all_posts: Vec<Post> = app_state.get_all_posts();
    //     all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    //     let batch_pagination = core::types::Pagination {
    //         page: page / 5,
    //         page_size: 50,
    //     };
    //     let batch_start = batch_pagination.page * batch_pagination.page_size;
    //     let batch_end = std::cmp::min(batch_start + batch_pagination.page_size, all_posts.len());

    //     let mut current_batch: Vec<Post> = all_posts[batch_start..batch_end].to_vec();

    //     let seed = time() as u64;
    //     let mut rng = StdRng::seed_from_u64(seed);
    //     current_batch.shuffle(&mut rng);

    //     let inner_pagination = core::types::Pagination {
    //         page: page % 5,
    //         page_size: 10,
    //     };
    //     let start = inner_pagination.page * inner_pagination.page_size;
    //     let end = std::cmp::min(start + inner_pagination.page_size, current_batch.len());

    //     current_batch[start..end].to_vec()
    // })
}

#[update(guard = guard_prevent_anonymous)]
pub async fn api_get_my_posts(
    args: crate::models::types::Pagination,
) -> Vec<crate::models::post::Post> {
    crate::with_read_state(|state| {
        let mut all_posts: Vec<crate::models::post::Post> = Vec::new();

        for (_, pos) in state.posts.iter() {
            if &pos.creator_id == &ic_cdk::api::caller() {
                all_posts.push(pos);
            }
        }

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

    // const POSTS_PER_PAGE: usize = 5;

    // let (post_ids_result,): (Result<Vec<u128>, String>,) =
    //     call(dexfans_backend_canister_id, "get_user_posts", (user_id,))
    //         .await
    //         .map_err(|_| "Failed to communicate with dexfans_backend canister".to_string())?;

    // let mut post_ids = match post_ids_result {
    //     Ok(ids) => ids,
    //     Err(e) => return Err(format!("User not found or error fetching posts: {}", e)),
    // };

    // post_ids.reverse();

    // let start = (page as usize) * POSTS_PER_PAGE;
    // let end = start + POSTS_PER_PAGE;

    // if start >= post_ids.len() {
    //     return Ok(vec![]);
    // }

    // STATE.with(|state| {
    //     let app_state = state.borrow();
    //     let paginated_posts: Vec<Post> = post_ids[start..post_ids.len().min(end)]
    //         .iter()
    //         .filter_map(|&post_id| app_state.posts.get(&post_id).map(|post| post.clone()))
    //         .collect();

    //     Ok(paginated_posts)
    // })
}

#[update]
pub async fn get_latest_subscribed_posts(
    dexfans_backend_canister_id: Principal,
    user_id: Principal,
    page: u32,
) -> Result<Vec<Post>, String> {
    const POSTS_PER_PAGE: usize = 10;

    let (post_ids_result,): (Result<Vec<u128>, String>,) = call(
        dexfans_backend_canister_id,
        "get_subscribed_users_posts",
        (user_id,),
    )
    .await
    .map_err(|_| "Failed to communicate with dexfans_backend canister".to_string())?;

    let post_ids = match post_ids_result {
        Ok(ids) => ids,
        Err(e) => return Err(format!("Error fetching subscribed users' posts: {}", e)),
    };

    STATE.with(|state| {
        let app_state = state.borrow();

        let mut posts: Vec<Post> = post_ids
            .iter()
            .filter_map(|&post_id| app_state.posts.get(&post_id).map(|post| post.clone()))
            .collect();

        posts.sort_by(|a, b| b.post_id.cmp(&a.post_id));

        let start = (page as usize) * POSTS_PER_PAGE;
        let end = std::cmp::min(start + POSTS_PER_PAGE, posts.len());

        let paginated_posts = if start < posts.len() {
            posts[start..end].to_vec()
        } else {
            vec![]
        };

        Ok(paginated_posts)
    })
}
