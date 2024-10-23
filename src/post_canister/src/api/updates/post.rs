// use candid::Principal;
// use ic_cdk::{call, update};

// use crate::{
//     models::{
//         comment::Comment,
//         post::{CreatePostArgs, Post},
//     },
//     STATE,
// };

// #[update]
// pub async fn create_post(
//     // dexfans_backend_canister_id: Principal,
//     args: CreatePostArgs,
//     // creator_id: Principal,
// ) -> Result<(), String> {
//     let post_id = STATE.with(|state| {
//         let mut app_state = state.borrow_mut();

//         let new_post_id = app_state.post_counter + 1;
//         app_state.post_counter = new_post_id;

//         let new_post = Post {
//             post_id: new_post_id.to_string(),
//             content: args.content,
//             image: args.image,
//             video: args.video,
//             post_type: args.post_type,
//             price: args.price,
//             likes: vec![],
//             views: vec![],
//             comments: vec![],
//             // creator_id,
//             creator_id: ic_cdk::api::caller(),
//             created_at: current_timestamp(),
//         };

//         if app_state.posts.insert(new_post_id, new_post).is_some() {
//             return Err("Failed to create post: Post with the same ID already exists".to_string());
//         }

//         Ok(new_post_id)
//     })?;

//     let parent_canister = crate::utils::functions::get_canister_meta_data().unwrap();

//     let (result,): (Result<(), String>,) = call(
//         parent_canister.parent_canister,
//         "add_post_id_to_user",
//         // (creator_id, post_id),
//         (ic_cdk::api::caller(), post_id),
//     )
//     .await
//     .map_err(|_| "Failed to add post_id to user profile in dexfans_backend".to_string())?;

//     result
// }

// pub fn current_timestamp() -> u64 {
//     ic_cdk::api::time() / 1_000_000
// }

// #[update]
// pub async fn like_unlike_post(
//     // dexfans_backend_canister_id: Principal,
//     post_id: u128,
//     // user: Principal,
// ) -> Result<(), String> {
//     let caller = ic_cdk::api::caller();
//     let is_liked = STATE.with(|state| {
//         let mut app_state = state.borrow_mut();

//         if let Some(mut post) = app_state.posts.remove(&post_id) {
//             // let is_liked = post.likes.contains(&user);
//             let is_liked = post.likes.contains(&caller);

//             if is_liked {
//                 // post.likes.retain(|&p| p != user);
//                 post.likes.retain(|&p| p != caller.clone());
//             } else {
//                 // post.likes.push(user);
//                 post.likes.push(caller);
//             }

//             app_state.posts.insert(post_id, post);
//             Ok(is_liked)
//         } else {
//             Err("Post not found.".to_string())
//         }
//     })?;

//     let parent_canister = crate::utils::functions::get_canister_meta_data().unwrap();

//     let (result,): (Result<(), String>,) = call(
//         parent_canister.parent_canister,
//         "update_user_likes",
//         // (user, post_id, is_liked),
//         (ic_cdk::api::caller(), post_id, is_liked),
//     )
//     .await
//     .map_err(|_| "Failed to update likes list in dexfans_backend".to_string())?;

//     result
// }

// #[update]
// pub fn comment_on_post(
//     post_id: u128,
//     content: String,
//     image: Option<String>,
//     // creator_id: Principal,
// ) -> Result<(), String> {
//     STATE.with(|state| {
//         let mut app_state = state.borrow_mut();

//         if let Some(mut post) = app_state.posts.remove(&post_id) {
//             let comment_id = app_state.comment_counter + 1;
//             app_state.comment_counter = comment_id;

//             let new_comment = Comment {
//                 comment_id,
//                 content,
//                 image,
//                 // creator_id,
//                 creator_id: ic_cdk::api::caller(),
//                 created_at: current_timestamp(),
//             };

//             app_state.comments.insert(comment_id, new_comment);

//             post.comments.push(comment_id);

//             app_state.posts.insert(post_id, post);
//             Ok(())
//         } else {
//             Err("Post not found.".to_string())
//         }
//     })
// }

// // use candid::Principal;
// // use ic_cdk::{api, call, update};

// // use crate::{
// //     models::{
// //         comment::Comment,
// //         post::{CreatePostArgs, Post},
// //     },
// //     STATE,
// // };

// // #[update]
// // pub async fn create_post(
// //     dexfans_backend_canister_id: Principal,
// //     args: CreatePostArgs,
// //     creator_id: Principal,
// // ) -> Result<(), String> {
// //     let post_id = STATE.with(|state| {
// //         let mut app_state = state.borrow_mut();

// //         let new_post_id = app_state.post_counter + 1;
// //         app_state.post_counter = new_post_id;

// //         let new_post = Post {
// //             post_id: new_post_id,
// //             content: args.content,
// //             image: args.image,
// //             video: args.video,
// //             post_type: args.post_type,
// //             price: args.price,
// //             likes: vec![],
// //             views: vec![],
// //             comments: vec![],
// //             creator_id,
// //             created_at: current_timestamp(),
// //         };

// //         if app_state.posts.insert(new_post_id, new_post).is_some() {
// //             return Err("Failed to create post: Post with the same ID already exists".to_string());
// //         }

// //         Ok(new_post_id)
// //     })?;

// //     let (result,): (Result<(), String>,) = call(
// //         dexfans_backend_canister_id,
// //         "add_post_id_to_user",
// //         (creator_id, post_id),
// //     )
// //     .await
// //     .map_err(|_| "Failed to add post_id to user profile in dexfans_backend".to_string())?;

// //     result
// // }

// // pub fn current_timestamp() -> u64 {
// //     api::time() / 1_000_000
// // }

// // #[update]
// // pub async fn like_unlike_post(
// //     dexfans_backend_canister_id: Principal,
// //     post_id: u128,
// //     user: Principal,
// // ) -> Result<(), String> {
// //     let is_liked = STATE.with(|state| {
// //         let mut app_state = state.borrow_mut();

// //         if let Some(mut post) = app_state.posts.remove(&post_id) {
// //             let is_liked = post.likes.contains(&user);

// //             if is_liked {
// //                 post.likes.retain(|&p| p != user);
// //             } else {
// //                 post.likes.push(user);
// //             }

// //             app_state.posts.insert(post_id, post);
// //             Ok(is_liked)
// //         } else {
// //             Err("Post not found.".to_string())
// //         }
// //     })?;

// //     let (result,): (Result<(), String>,) = call(
// //         dexfans_backend_canister_id,
// //         "update_user_likes",
// //         (user, post_id, is_liked),
// //     )
// //     .await
// //     .map_err(|_| "Failed to update likes list in dexfans_backend".to_string())?;

// //     result
// // }

// // #[update]
// // pub fn comment_on_post(
// //     post_id: u128,
// //     content: String,
// //     image: Option<String>,
// //     creator_id: Principal,
// // ) -> Result<(), String> {
// //     STATE.with(|state| {
// //         let mut app_state = state.borrow_mut();

// //         if let Some(mut post) = app_state.posts.remove(&post_id) {
// //             let comment_id = app_state.comment_counter + 1;
// //             app_state.comment_counter = comment_id;

// //             let new_comment = Comment {
// //                 comment_id,
// //                 content,
// //                 image,
// //                 creator_id,
// //                 created_at: current_timestamp(),
// //             };

// //             app_state.comments.insert(comment_id, new_comment);

// //             post.comments.push(comment_id);

// //             app_state.posts.insert(post_id, post);
// //             Ok(())
// //         } else {
// //             Err("Post not found.".to_string())
// //         }
// //     })
// // }
