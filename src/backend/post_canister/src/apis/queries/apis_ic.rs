use crate::utils::guards::*;

#[ic_cdk::query(guard = guard_parent_canister_only)]
fn ic_get_price(post_id: core::types::PostId) -> Result<core::types::PostPrice, String> {
    crate::with_read_state(|state| match state.posts.get(&post_id) {
        Some(post) => match post.price {
            Some(val) => Ok(val),
            None => Ok(0),
        },
        None => Err(String::from(core::constants::ERROR_POST_NOT_EXIST)),
    })
}

#[ic_cdk::query(guard = guard_parent_canister_only)]
fn ic_get_media_price(
    args: core::types::SinglePurchaseArgs,
) -> Result<core::types::PostPrice, String> {
    crate::with_read_state(|state| {
        if let Some(post) = state.posts.get(&args.post_id) {
            if let Some(images) = &post.image {
                for img in images.iter() {
                    if img.source == args.media_id as u32 && img.need_pay {
                        return Ok(img.price.unwrap_or(0));
                    }
                }
            }
        }
        Err(String::from(core::constants::ERROR_POST_NOT_EXIST))
    })
}
