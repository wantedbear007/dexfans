use crate::models::types::*;
use candid::Principal;
use models::types::CanisterMetaData;
use store::storage_state::ApplicationState;
mod api;
mod models;
mod store;
mod utils;
use dexfans_types::types::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!(
        "Hello, {}! from {}",
        name,
        dexfans_types::constants::ESSENTIALS_APP_NAME
    )
}

thread_local! {
    static STATE: std::cell::RefCell<ApplicationState> = std::cell::RefCell::new(ApplicationState::new());
}

// to get mutable reference
pub(crate) fn with_write_state<R>(f: impl FnOnce(&mut ApplicationState) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

// to get inmutable reference
pub(crate) fn with_read_state<R>(f: impl FnOnce(&ApplicationState) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

// init args
#[ic_cdk::init]
async fn init(args: crate::models::types::DexFansCanisterInitArgs) {
    with_write_state(|state| {
        state.canister_meta_data.insert(
            0,
            CanisterMetaData {
                canister_ids: {
                    let mut canister_ids: std::collections::HashMap<u8, candid::Principal> =
                        std::collections::HashMap::with_capacity(args.canister_ids.len());

                    canister_ids.insert(
                        dexfans_types::constants::ESSENTIAL_ASSET_CANISTER_ID_CODE,
                        args.canister_ids["asset_canister"],
                    );
                    canister_ids.insert(
                        dexfans_types::constants::ESSENTIAL_POST_CANISTER_ID_CODE,
                        args.canister_ids["post_canister"],
                    );
                    canister_ids.insert(
                        dexfans_types::constants::ESSENTIAL_LEDGER_CANISTER_ID_CODE,
                        args.canister_ids["ledger_canister"],
                    );
                    canister_ids
                },
                controllers: args.controllers,
                payment_recipient: args.payment_recipient,
                all_post_canisters: {
                    let mut post_canisters = std::collections::HashSet::new();
                    post_canisters.insert(args.canister_ids["post_canister"]);
                    post_canisters
                },
            },
        );
    });
}

ic_cdk::export_candid!();
