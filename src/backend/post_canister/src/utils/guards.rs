use crate::with_read_state;

// to prevent anonymous calls
pub fn guard_prevent_anonymous() -> Result<(), String> {
    if ic_cdk::api::caller() == candid::Principal::anonymous() {
        return Err(String::from(core::constants::WARNING_ANONYMOUS_CALL));
    }

    Ok(())
}

// to allow parent canister only (for intercanister calls)
pub fn guard_parent_canister_only() -> Result<(), String> {
    guard_prevent_anonymous()?;
    with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            if val.canister_ids[core::constants::ESSENTIAL_POST_PARENT_CANISTER]
                == ic_cdk::api::caller()
            {
                return Ok(());
            } else {
                return Err(String::from(core::constants::WARNING_ADMIN_ONLY));
            };
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    })
}

// validate length of images
pub(crate) fn checks_image_validation(
    args: crate::models::post::CreatePostArgs,
) -> core::types::Response {
    match args.image {
        Some(img) => {
            if img.len() > core::constants::ESSENTIAL_IMAGE_COUNT_LIMIT {
                return Err(String::from(core::constants::WARNING_IMAGES_LIMIT));
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}

// pub(crate) fn check
