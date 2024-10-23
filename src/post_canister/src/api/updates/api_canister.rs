use crate::utils::guards::*;

// to add user details (Intercanister)
#[ic_cdk::update(guard = guard_parent_canister_only)]
pub fn admin_add_user_profile(
    args: crate::models::types::UserProfileInterCanister,
) -> Result<(), String> {
    crate::with_write_state(|state| state.account.insert(args.user_id, args));

    Ok(())
}

// for debug only
#[ic_cdk::query]
pub fn debug_get_user_profile() -> Result<crate::models::types::UserProfileInterCanister, String> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(val) => Ok(val),
        None => return Err(String::from("failed to get user profile")),
    })
}

#[ic_cdk::query]
pub fn debug_get_all_profile() -> Vec<crate::models::types::UserProfileInterCanister> {
    crate::with_read_state(|state| {
        let mut acc: Vec<crate::models::types::UserProfileInterCanister> =
            Vec::with_capacity(state.account.len() as usize);
        for x in state.account.iter() {
            acc.push(x.1);
        }

        acc
    })
}
