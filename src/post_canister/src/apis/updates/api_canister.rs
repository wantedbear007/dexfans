use crate::utils::guards::*;

// to add user details (Intercanister)
#[ic_cdk::update(guard = guard_parent_canister_only)]
pub fn admin_add_user_profile(args: crate::models::types::UserProfileIC) -> Result<(), String> {
    crate::with_write_state(|state| state.account.insert(args.user_id, args));

    Ok(())
}

// to update user details
#[ic_cdk::update(guard = guard_parent_canister_only)]
pub fn admin_update_user_profile(
    args: core::types::UpdateUserProfileArgsIC,
) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&args.user_id) {
        Some(mut val) => {
            if &val.username != &args.username {
                val.username = args.username
            }

            // if &val.membership != &args.membership {
            //     val.membership = args.membership
            // }

            state.account.insert(args.user_id, val);

            Ok(())
        }
        None => Err(String::from(
            core::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}

// to update membership
// add parent canister guard
#[ic_cdk::update(guard = guard_parent_canister_only)]
pub fn admin_update_membership(
    args: core::types::UpdateMembershipIC,
) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&args.user) {
        Some(mut val) => {
            val.membership = args.membership;
            state.account.insert(args.user, val);

            Ok(())
        }
        None => Err(String::from(
            core::constants::ERROR_ACCOUNT_NOT_REGISTERED,
        )),
    })
}

// for debug only
#[ic_cdk::query]
pub fn debug_get_user_profile() -> Result<crate::models::types::UserProfileIC, String> {
    crate::with_read_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(val) => Ok(val),
        None => return Err(String::from("failed to get user profile")),
    })
}

#[ic_cdk::query]
pub fn debug_get_all_profile() -> Vec<crate::models::types::UserProfileIC> {
    crate::with_read_state(|state| {
        let mut acc: Vec<crate::models::types::UserProfileIC> =
            Vec::with_capacity(state.account.len() as usize);
        for x in state.account.iter() {
            acc.push(x.1);
        }

        acc
    })
}
