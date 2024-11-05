use crate::utils::guards::*;
use crate::{models::post::Post, STATE};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!(
        "Hello, {}! from {}",
        name,
        core::constants::ESSENTIALS_APP_NAME
    )
}

#[ic_cdk::query]
fn get_all_controllers() -> std::collections::HashSet<candid::Principal> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(data) => data.controllers,
        None => std::collections::HashSet::new(),
    })
}

// debug
#[ic_cdk::query]
fn debug_total_posts() -> u128 {
    crate::with_read_state(|state| state.post_counter)
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
fn api_get_post_ids(page: core::types::Pagination) -> Vec<u128> {
    crate::with_read_state(|state| {
        let mut ids: Vec<u128> = Vec::new();

        for (id, _) in state.posts.iter() {
            ids.push(id);
        }

        let ending = ids.len();

        if ending == 0 {
            return ids;
        }

        let start = page.start as usize;
        let end = page.end as usize;
        if start < ending {
            let end = end.min(ending);

            return ids[start..end].to_vec();
        }

        Vec::new()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_get_post_by_id(post_id: u128) -> Option<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.posts.get(&post_id).map(|post| post.clone())
    })
}

#[ic_cdk::query(guard=guard_prevent_anonymous)]
pub fn api_list_all_posts() -> Vec<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.get_all_posts()
    })
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_post_by_user_id(user_id: candid::Principal, page: core::types::Pagination) -> Vec<Post> {
    crate::with_read_state(|state| {
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
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub fn api_get_latest_posts(page: core::types::Pagination) -> Vec<Post> {
    crate::with_read_state(|state| {
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
}

#[ic_cdk::query(guard = guard_prevent_anonymous)]
pub async fn api_get_my_posts(args: core::types::Pagination) -> Vec<crate::models::post::Post> {
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
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
pub async fn api_get_subscribed_posts(page: core::types::Pagination) -> Vec<crate::Post> {
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
                    if acc_set.contains(&post.creator_id) {
                        all_posts.push(post.clone());
                    }
                }
            });

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

            // val
        }
        Err(err) => {
            ic_cdk::println!("{}", err.to_string()); // for debug only

            return Vec::new();
        }
    }
}
