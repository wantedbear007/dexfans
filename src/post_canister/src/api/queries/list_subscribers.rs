// use candid::Principal;
// use ic_cdk::call;

// #[ic_cdk::update]
// async fn list_subscribers(dexfans_backend_canister_id: Principal, user_id: Principal) -> Result<Vec<Principal>, String> {

//     let (user_profile,): (Result<crate::models::user::UserProfile, String>,) = 
//         call(dexfans_backend_canister_id, "api_get_user_account", (user_id,))
//             .await
//             .map_err(|_| "Failed to fetch user profile".to_string())?;

//     match user_profile {
//         Ok(profile) => Ok(profile.subscribers),
//         Err(_) => Err("User not found.".to_string()),
//     }
// }