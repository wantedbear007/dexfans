use crate::utils::guards::*;
// to add subscribers
// add post canister guardx
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


// to add controller of canister
#[ic_cdk::update(guard=guard_only_admin)]
pub fn admin_add_controller(id: candid::Principal) -> Result<(), String> {
    crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            state.canister_meta_data.insert(
                0,
                crate::models::types::CanisterMetaData {
                    controllers: {
                        let mut controllers = val.controllers;
                        controllers.insert(id);
                        controllers
                    },
                    ..val
                },
            );

            Ok(())
        }
        None => return Err(String::from(dexfans_types::constants::ERROR_FAILED_CALL)),
    })
}

// to remove controller of canister
#[ic_cdk::update(guard=guard_only_admin)]
pub fn admin_remove_controller(id: candid::Principal) -> Result<(), String> {
    crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            let controllers: std::collections::HashSet<candid::Principal> = val
                .controllers
                .iter()
                .filter(|&&controller| controller != id)
                .cloned()
                .collect();

            state.canister_meta_data.insert(
                0,
                crate::models::types::CanisterMetaData { controllers, ..val },
            );

            Ok(())
        }
        None => return Err(String::from(dexfans_types::constants::ERROR_FAILED_CALL)),
    })
}

// to update the current post canister
#[ic_cdk::update(guard = guard_only_admin)]
pub fn admin_set_post_canister(id: candid::Principal) -> Result<candid::Principal, String> {
    crate::with_write_state(|state| match state.canister_meta_data.get(&0) {
        Some(mut canister_meta_data) => {
            canister_meta_data.all_post_canisters.insert(id);

            canister_meta_data.canister_ids.insert(
                dexfans_types::constants::ESSENTIAL_POST_CANISTER_ID_CODE,
                id,
            );
            state.canister_meta_data.insert(0, canister_meta_data);

            Ok(id)
        }
        None => return Err(String::from(dexfans_types::constants::ERROR_CANISTER_ID)),
    })
}

#[ic_cdk::query]
pub fn get_canister_meta_data() -> Result<crate::models::types::CanisterMetaData, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val),
        None => {
            return Err(String::from(
                dexfans_types::constants::ERROR_FAILED_CANISTER_DATA,
            ))
        }
    })
}
