use candid::{types::leb128, Principal};

use crate::{utils::functions::get_post_canister, with_write_state};

pub async fn controller_create_account(
    args: crate::models::types::UserInputArgs,
) -> Result<(), String> {
    match crate::with_write_state(|state| {
        // checking if user already exists
        if state.account.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(core::constants::WARNING_ACCOUNT_EXISTS));
        } else {
            // to retrieve canister meta data
            let canister_meta_data = state
                .canister_meta_data
                .get(&0)
                .expect(core::constants::ERROR_FAILED_CANISTER_DATA)
                .canister_ids;

            // add user details to stable storage
            state.account.insert(
                ic_cdk::api::caller(),
                crate::models::types::UserProfile {
                    avatar: args.avatar,
                    bio: args.bio,
                    all_post_canisters: std::collections::HashSet::from([
                        canister_meta_data[&core::constants::ESSENTIAL_POST_CANISTER_ID_CODE]
                    ]),
                    cover_image: args.cover_image,
                    collects: Vec::new(),
                    likes: Vec::new(),
                    posts: Vec::new(),
                    created_at: ic_cdk::api::time(),
                    is_bot: false,
                    membership: core::types::Membership::Guest,
                    subscribers: std::collections::HashSet::new(),
                    subscribing: std::collections::HashSet::new(),
                    user_id: ic_cdk::api::caller(),
                    username: args.username.clone(),
                    asset_canister_id: canister_meta_data
                        [&core::constants::ESSENTIAL_ASSET_CANISTER_ID_CODE],
                    active_post_canister: canister_meta_data
                        [&core::constants::ESSENTIAL_POST_CANISTER_ID_CODE],
                    membership_till: 0,
                    membership_ledger_block: None
                },
            );

            state
                .notifications
                .insert(ic_cdk::api::caller(), crate::Notification::default());
        }

        Ok(())
    }) {
        Ok(()) => {
            match ic_create_profile(crate::models::types::UserProfileInterCanister {
                user_id: ic_cdk::api::caller(),
                username: args.username,
                ..Default::default()
            })
            .await
            {
                Ok(()) => Ok(()),
                Err(err) => {
                    // roll back if the call fails
                    crate::with_write_state(|state| state.account.remove(&ic_cdk::api::caller()));
                    Err(err)
                }
            }
        }
        Err(err) => Err(err),
    }
}

// intercanister create profile
pub async fn ic_create_profile(
    args: crate::models::types::UserProfileInterCanister,
) -> Result<(), String> {
    match kaires::call_inter_canister::<crate::models::types::UserProfileInterCanister, ()>(
        "admin_add_user_profile",
        args,
        crate::utils::functions::get_post_canister()
            .expect(core::constants::ERROR_FAILED_INTER_CANISTER),
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(err) => return Err(err),
    }
}

// intercanister update profile
pub async fn ic_update_profile(args: core::types::UpdateUserProfileArgsIC) -> Result<(), String> {
    match kaires::call_inter_canister::<core::types::UpdateUserProfileArgsIC, ()>(
        "admin_update_user_profile",
        args,
        get_post_canister().expect(core::constants::ERROR_FAILED_CANISTER_DATA),
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(err) => return Err(err),
    }
}

// intercanister update membership
#[ic_cdk::update]
pub async fn ic_update_membership(
    // args: core::types::UpdateMembershipIC,
    args: core::types::Membership,
) -> Result<(), String> {
    match kaires::call_inter_canister::<core::types::UpdateMembershipIC, ()>(
        "admin_update_membership",
        core::types::UpdateMembershipIC {
            user: ic_cdk::api::caller(),
            membership: args,
        },
        get_post_canister().expect(core::constants::ERROR_FAILED_CANISTER_DATA),
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(err) => return Err(err),
    }
}

// rollbacks
pub fn rb_membership_update(args: core::types::Membership) -> Result<(), String> {
    with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(mut val) => {
            val.membership = args;
            state.account.insert(ic_cdk::api::caller(), val);
            Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_FAILED_CALL)),
    })
}

// purchase subscription
pub fn controller_membership(args: core::types::Membership) -> Result<(), String> {
    crate::with_write_state(|state| match state.account.get(&ic_cdk::api::caller()) {
        Some(mut acc) => {
            // check if user is trying to purchase same membership
            if &acc.membership == &args {
                return Err(String::from(core::constants::WARNING_SAME_MEMBERSHIP));
            }

            // check if user still have higher class membership
            if &acc.membership > &args {
                ic_cdk::println!("inside higher membership");
                return Err(String::from(core::constants::WARNING_HIGHER_MEMBERSHIP));
            }

            // if &acc.membership < &args {
            //     ic_cdk::println!("inside lower but paid membership");

            //     // TODO ask about higher membership
            //     // upgrade membership
            //     // calculate difference
            // }

            if &acc.membership == &core::types::Membership::Guest {
                // debug print
                ic_cdk::println!("inside free membership");

                let mut ledger: candid::Principal = Principal::anonymous();
                let mut recipient: candid::Principal = Principal::anonymous();
                let mut payable_amount: u64 = 0;
                // membership amount
                match state.canister_meta_data.get(&0) {
                    Some(val) => {
                        // payable_amt =
                        ledger =
                            val.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE];
                        recipient = val.payment_recipient;
                        payable_amount = val.membership_plans[&args];
                    }
                    None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
                };

                // payment process
                ic_cdk::spawn(async move {
                    super::payment_controller::icp_transfer_handler(
                        payable_amount,
                        recipient,
                        ledger,
                    )
                    .await
                    .unwrap();
                });

                // upgrading membership
                acc.membership = args;
                acc.membership_till =
                    ic_cdk::api::time() + core::constants::ESSENTIAL_MEMBERSHIP_VALIDITY;
                state.account.insert(acc.user_id, acc);
                Ok(())
            } else {
                return Err(String::from("adfgdfgfdgfdgfd"));
            }
            // Ok(())
        }
        None => return Err(String::from(core::constants::ERROR_ACCOUNT_NOT_REGISTERED)),
    })

}
