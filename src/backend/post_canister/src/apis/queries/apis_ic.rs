use crate::utils::guards::*;

#[ic_cdk::query(guard = guard_parent_canister_only)]
fn ic_get_price(post_id: u128) -> crate::PostPrice {
    crate::with_read_state(|state| match state.posts.get(&post_id) {
        Some(post) => match post.price {
            Some(val) => val,
            None => 0,
        },
        None => 0,
    })
}
