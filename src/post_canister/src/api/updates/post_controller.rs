use candid::Principal;
use serde_bytes::ByteBuf;

use crate::with_write_state;

pub(crate) async fn controller_create_post(args: crate::CreatePostArgs) -> Result<(), String> {
    let post_id = crate::utils::functions::commons_get_uuid().await;

    // let id = match args.image.clone() {
    //     Some(val) => {
    //         match kaires::upload_image_to_asset_canister(kaires::ImageData {
    //             asset_canister: Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").unwrap(),
    //             content: val.image_content,
    //             content_type: String::from("jpeg"),
    //             name: String::from("name"),
    //         })
    //         .await
    //         {
    //             Ok(val) => Ok(val),
    //             Err(val) => return Err(val),
    //         }
    //     }
    //     None => Err(String::from("h")),
    // };

    with_write_state(|state| {
        state.posts.insert(
            post_id,
            crate::Post {
                content: args.content,
                image: args.image,
                post_type: args.post_type,
                price: args.price,
                video: args.video,
                ..Default::default()
            },
        )
    })
    .unwrap();

    Ok(())
}
