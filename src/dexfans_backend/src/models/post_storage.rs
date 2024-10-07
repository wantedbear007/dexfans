use candid::Principal;
use ic_cdk::{api, query, update};
use crate::models::post::{Post, CreatePostArgs};
use crate::store::storage_state::ApplicationState;
use crate::STATE;
use rand::seq::SliceRandom;


use super::comment::Comment;
use super::pagination::Pagination;
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

        
        if let Some(mut post) = app_state.posts.remove(&post_id) {
            
            if post.likes.contains(&user) {
                post.likes.retain(|&p| p != user);
            } else {
                
                post.likes.push(user);
            }

            
            app_state.posts.insert(post_id, post);
            Ok(())
        } else {
           
            Err("Post not found.".to_string())
        }
    })
}


#[update]
pub fn comment_on_post(post_id: u128, content: String, image: Option<String>, creator_id: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let mut app_state = state.borrow_mut();

        
        if let Some(mut post) = app_state.posts.remove(&post_id) {
            
            let comment_id = app_state.comment_counter + 1;
            app_state.comment_counter = comment_id;

            
            let new_comment = Comment {
                comment_id,
                content,
                image,
                creator_id,
                created_at: current_timestamp(),
            };

            
            app_state.comments.insert(comment_id, new_comment);

            
            post.comments.push(comment_id);

            
            app_state.posts.insert(post_id, post);
            Ok(())
        } else {
            Err("Post not found.".to_string())
        }
    })
}






#[query]
pub fn latest_posts(page: usize) -> Vec<Post> {
    STATE.with(|state| {
        let app_state = state.borrow();

        
        let mut all_posts: Vec<Post> = app_state.get_all_posts();

        
        all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        
        let batch_size = 50;

        
        let batch_start = (page / 5) * batch_size;  // Each batch is 50 posts, with 10 posts per page
        let batch_end = std::cmp::min(batch_start + batch_size, all_posts.len());

        
        let mut current_batch: Vec<Post> = all_posts[batch_start..batch_end].to_vec();

        
        let mut rng = rand::thread_rng();
        current_batch.shuffle(&mut rng);

        
        let batch_page = page % 5;
        let pagination = Pagination { page: batch_page, page_size: 10 };

       
        let start = pagination.page * pagination.page_size;
        let end = std::cmp::min(start + pagination.page_size, current_batch.len());

        
        current_batch[start..end].to_vec()
    })
}