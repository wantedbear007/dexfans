use icrc_ledger_types::{
    icrc1::{account::Account, transfer::BlockIndex},
    icrc2::transfer_from::{TransferFromArgs, TransferFromError},
};

// for handling icp payment
pub(super) async fn icp_transfer_handler(
    tokens: u64,
    recipient: candid::Principal,
    ledger_id: candid::Principal,
) -> Result<BlockIndex, String> {
    // let mut recipient: candid::Principal = candid::Principal::anonymous();
    // let mut ledger_canister: candid::Principal = candid::Principal::anonymous();

    // fetching values of payment recipient and ledger canister id
    // let _x = crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
    //     Some(val) => {
    //         ledger_canister = val.canister_ids[&core::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE];

    //         recipient = val.payment_recipient;
    //         Ok(())
    //     }
    //     None => return Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    // });

    let transfer_args = TransferFromArgs {
        amount: tokens.into(),
        to: Account {
            owner: recipient,
            subaccount: None,
        },
        fee: None,
        memo: None,
        created_at_time: Some(ic_cdk::api::time()),
        spender_subaccount: None,
        from: Account {
            owner: ic_cdk::api::caller(),
            subaccount: None,
        },
    };

    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        ledger_id,
        "icrc2_transfer_from",
        (transfer_args,),
    )
    .await
    .map_err(|e| format!("{}{:?}", core::constants::ERROR_FAILED_INTER_CANISTER, e))?
    .0
    .map_err(|e| format!("{} {:?}", core::constants::ERROR_PAYMENT_FAILED, e))
}

// purchase membership
// pub async fn upgrade_membership(
//     args: core::types::Membership,
//     amt: u64,
//     recipient: candid::Principal,
//     ledger: candid::Principal,
// ) -> Result<BlockIndex, String> {

    

//     icp_transfer_handler(amt, recipient, ledger).await
// }
