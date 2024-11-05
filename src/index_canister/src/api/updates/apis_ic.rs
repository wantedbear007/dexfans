use crate::utils::guards::*;

#[ic_cdk::update(guard = guard_post_canister_exclusive)]
pub fn ic_subscribe_account(args: core::types::SubscribeAccountIC) -> Result<(), String> {
    // TODO add validation if user has already subscribed
    crate::with_write_state(|state| {
        let mut subscribed_by = state
            .account
            .get(&args.subscribed_by)
            .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);
        let mut subscribed_to = state
            .account
            .get(&args.subscribed_to)
            .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        if subscribed_by.subscribing.contains(&args.subscribed_to) {
            return Err(String::from(core::constants::WARNING_ALERADY_EXIST));
        }

        subscribed_by.subscribing.insert(subscribed_to.user_id);
        subscribed_to.subscribers.insert(subscribed_by.user_id);

        state.account.insert(subscribed_by.user_id, subscribed_by);
        state.account.insert(subscribed_to.user_id, subscribed_to);

        Ok(())
    })
}

// check for user existance
// add bost to and accounts

#[ic_cdk::update(guard = guard_post_canister_exclusive)]
pub fn ic_unsubscribe_account(args: core::types::UnsubscribeAccountIC) -> Result<(), String> {
    crate::with_write_state(|state| {
        let mut unsubscribed_by = state
            .account
            .get(&args.unsubscribed_by)
            .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        let mut unsubscribed_to = state
            .account
            .get(&args.unsubscribed_to)
            .expect(core::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        // Remove the unsubscribed_to from the unsubscribed_by's subscriptions
        unsubscribed_by
            .subscribing
            .retain(|&subscribed_user_id| subscribed_user_id != unsubscribed_to.user_id);

        // Remove the unsubscribed_by from the unsubscribed_to's subscribers
        unsubscribed_to
            .subscribers
            .retain(|&subscriber_id| subscriber_id != unsubscribed_by.user_id);

        // Update the state with the modified profiles
        state
            .account
            .insert(unsubscribed_by.user_id, unsubscribed_by);
        state
            .account
            .insert(unsubscribed_to.user_id, unsubscribed_to);
    });

    Ok(())
}

// to update current post canister in user profile
#[ic_cdk::update(guard = guard_post_canister_exclusive)]
pub fn admin_profile_post_canister(
    args: core::types::ICAddPostCanisterProfile,
) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&args.caller) {
        Some(mut acc) => {
            acc.all_post_canisters.insert(args.post_canister);
            state.account.insert(acc.user_id, acc);
            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })
}

// // to get subscribing ic
#[ic_cdk::query(guard = guard_post_canister_exclusive)]
pub fn ic_get_subscribed_list(
    id: candid::Principal,
) -> Result<std::collections::HashSet<candid::Principal>, String> {
    crate::with_read_state(|state| match state.account.get(&id) {
        Some(acc) => Ok(acc.subscribing),
        None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}
