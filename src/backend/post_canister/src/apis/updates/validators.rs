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
