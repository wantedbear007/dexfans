use crate::utils::guards::*;

#[ic_cdk::update(guard = guard_post_canister_exclusive)]
pub fn ic_subscribe_account(args: dexfans_types::types::SubscribeAccountIC) -> Result<(), String> {
    // TODO add validation if user has already subscribed
    crate::with_write_state(|state| {
        let mut subscribed_by = state
            .account
            .get(&args.subscribed_by)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);
        let mut subscribed_to = state
            .account
            .get(&args.subscribed_to)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        if subscribed_by.subscribing.contains(&args.subscribed_to) {
            return Err(String::from(
                dexfans_types::constants::WARNING_ALERADY_EXIST,
            ));
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
pub fn ic_unsubscribe_account(
    args: dexfans_types::types::UnsubscribeAccountIC,
) -> Result<(), String> {
    crate::with_write_state(|state| {
        let mut unsubscribed_by = state
            .account
            .get(&args.unsubscribed_by)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);

        let mut unsubscribed_to = state
            .account
            .get(&args.unsubscribed_to)
            .expect(dexfans_types::constants::ERROR_ACCOUNT_NOT_REGISTERED);

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


