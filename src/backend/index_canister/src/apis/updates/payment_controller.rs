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


