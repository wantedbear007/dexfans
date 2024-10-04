use candid::Principal;
use ic_cdk::{api, query, update};
use crate::models::post::{Post, CreatePostArgs};
use crate::store::storage_state::ApplicationState;
use crate::STATE;


use super::comment::Comment;
use super::user::UserProfile;

// Function to create and store a new post
#[update]
pub fn create_post(args: CreatePostArgs, creator_id: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut app_state = state.borrow_mut();
        
        let post_id = app_state.post_counter + 1;  // Generate a new post ID
        app_state.post_counter = post_id;          // Increment the counter

        let new_post = Post {
            post_id,
            content: args.content,
            image: args.image,
            video: args.video,
            post_type: args.post_type,
            price: args.price,
            likes: vec![],
            views: vec![],
            comments: vec![],
            creator_id,
            created_at: current_timestamp(),
        };

        // Insert the new post into the posts map
        if app_state.posts.insert(post_id, new_post).is_some() {
            Err("Failed to create post: Post with the same ID already exists".to_string())
        } else {
            Ok(())
        }
    })
}





fn current_timestamp() -> u64 {
    api::time() / 1_000_000  // Convert nanoseconds to milliseconds
}




#[query]
pub fn get_post_by_id(post_id: u128) -> Option<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        // Use map to clone the Post inside the Option if it exists
        app_state.posts.get(&post_id).map(|post| post.clone())
    })
}






#[query]
pub fn list_all_posts() -> Vec<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.get_all_posts()  // Return a Vec<Post>
    })
}


#[query]
pub fn list_all_accounts() -> Vec<UserProfile> {
    STATE.with(|state| {
        let app_state = state.borrow();
        app_state.get_all_accounts()  // Return a Vec<UserProfile>
    })
}





#[update]
pub fn like_unlike_post(post_id: u128, user: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut app_state = state.borrow_mut();

        // First, retrieve the post from the map
        if let Some(mut post) = app_state.posts.remove(&post_id) {
            // Check if the user has already liked the post
            if post.likes.contains(&user) {
                // If the user has already liked the post, remove the like (unlike)
                post.likes.retain(|&p| p != user);
            } else {
                // Otherwise, like the post by adding the user's Principal
                post.likes.push(user);
            }

            // Reinsert the modified post back into the StableBTreeMap
            app_state.posts.insert(post_id, post);
            Ok(())
        } else {
            // Return an error if the post doesn't exist
            Err("Post not found.".to_string())
        }
    })
}


#[update]
pub fn comment_on_post(post_id: u128, content: String, image: Option<String>, creator_id: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut app_state = state.borrow_mut();

        // Find the post by its post_id
        if let Some(mut post) = app_state.posts.remove(&post_id) {
            // Increment the comment counter to get a unique CommentId
            let comment_id = app_state.comment_counter + 1;
            app_state.comment_counter = comment_id;

            // Create a new comment using your existing structure
            let new_comment = Comment {
                comment_id,
                content,
                image,
                creator_id,
                created_at: current_timestamp(),
            };

            // Store the comment in the comments map
            app_state.comments.insert(comment_id, new_comment);

            // Add the comment ID to the post's comments list
            post.comments.push(comment_id);

            // Reinsert the modified post back into the posts map
            app_state.posts.insert(post_id, post);
            Ok(())
        } else {
            Err("Post not found.".to_string())
        }
    })
}