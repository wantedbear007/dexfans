use candid::Principal;
use ic_cdk::query;

use crate::STATE;



#[query]
pub fn get_user_posts(user_id: Principal) -> Result<Vec<u128>, String> {
    STATE.with(|state| {
        let app_state = state.borrow();
        
        // Attempt to retrieve the user's profile
        let user_profile = app_state.account.get(&user_id)
            .ok_or_else(|| "User not found".to_string())?;
        
        // Return the list of post IDs associated with this user
        Ok(user_profile.posts.clone())
    })
}

#[query]
pub fn get_subscribed_users_posts(user_id: Principal) -> Result<Vec<u128>, String> {
    STATE.with(|state| {
        let app_state = state.borrow();

        // Retrieve the user profile to get their subscriptions
        let user_profile = app_state.account.get(&user_id)
            .ok_or_else(|| "User not found".to_string())?;

        let mut all_post_ids = Vec::new();

        // Loop through each subscribed creator and gather their post IDs
        for &creator_id in &user_profile.subscribing {
            if let Some(creator_profile) = app_state.account.get(&creator_id) {
                all_post_ids.extend(&creator_profile.posts);
            }
        }

        Ok(all_post_ids)
    })
}