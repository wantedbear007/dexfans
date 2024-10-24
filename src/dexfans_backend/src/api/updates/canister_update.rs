use crate::utils::guards::*;
// to add subscribers
// add post canister guard
#[ic_cdk::update(guard = guard_post_canister_exclusive)]
pub fn ic_subscribe_account(args: dexfans_types::types::SubscribeAccountIC) -> Result<(), String> {
    crate::with_write_state(|state| {
        let mut subscribed_by = state
            .account
            .get(&args.subscribed_by)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);
        let mut subscribed_to = state
            .account
            .get(&args.subscribed_to)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        subscribed_by.subscribing.push(subscribed_to.user_id);
        subscribed_to.subscribers.push(subscribed_by.user_id);

        state.account.insert(subscribed_by.user_id, subscribed_by);
        state.account.insert(subscribed_to.user_id, subscribed_to);
    });

    Ok(())
}

// check for user existance
// add bost to and accounts
