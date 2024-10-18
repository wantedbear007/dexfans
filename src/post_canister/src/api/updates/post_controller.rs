// use candid::Principal;

// use crate::with_write_state;

// pub(crate) async fn controller_create_post(args: crate::CreatePostArgs) -> Result<(), String> {
//     let post_id = crate::utils::functions::commons_get_uuid().await;
//     let id = match args.image.clone() {
//         Some(val) => {
//             ic_cdk::println!("Inside image upload");
//             match kaires::upload_image_to_asset_canister(kaires::ImageData {
//                 asset_canister: Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").unwrap(),
//                 content: val.image_content,
//                 content_type: String::from("jpeg"),
//                 name: String::from("name"),
//             })
//             .await
//             {
//                 Ok(val) => Some(val),
//                 Err(val) => return Err(val),
//             }
//         }
//         None => None,
//     };

//     with_write_state(|state| {
//         state.posts.insert(
//             post_id.clone(),
//             crate::Post {
//                 content: args.content,
//                 image: id,
//                 post_type: args.post_type,
//                 price: args.price,
//                 video: args.video,
//                 post_id,
//                 ..Default::default()
//             },
//         )
//     });

//     Ok(())
// }
