use crate::utils::guards::*;

#[ic_cdk::query(guard = guard_parent_canister_only)]
fn ic_get_price(
    args: core::types::PostPurchaseArgs,
) -> Result<core::types::PurchaseUserMedia, String> {
    crate::with_read_state(|state| match state.posts.get(&args.post_id) {
        Some(post) => {
            if post.creator_id == args.created_by {
                return Err(String::from(core::constants::WARNING_POST_OWNER));
            };

            match post.price {
                Some(val) => Ok(core::types::PurchaseUserMedia {
                    amt: val,
                    owner: post.creator_id,
                }),
                None => Err(String::from(core::constants::WARNING_POST_IS_FREE)),
            }
        }
        None => Err(String::from(core::constants::ERROR_POST_NOT_EXIST)),
    })
}

#[ic_cdk::query(guard = guard_parent_canister_only)]
fn ic_get_media_price(
    args: core::types::SinglePurchaseArgs,
) -> Result<core::types::PurchaseUserMedia, String> {
    crate::with_read_state(|state| {
        if let Some(post) = state.posts.get(&args.post_id) {
            if post.creator_id == args.created_by {
                return Err(String::from(core::constants::WARNING_POST_OWNER));
            }

            if let Some(images) = &post.image {
                for img in images.iter() {
                    if img.source == args.media_id && img.need_pay {
                        return Ok(core::types::PurchaseUserMedia {
                            amt: img.price.clone().unwrap_or(candid::Nat::default()),
                            owner: post.creator_id,
                        });
                    }
                }
            }
        }

        Err(String::from(core::constants::ERROR_POST_NOT_EXIST))
    })
}
