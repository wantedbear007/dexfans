use crate::models::user::UserProfile;
use candid::Principal;
use models::types::CanisterMetaData;
use store::storage_state::ApplicationState;
// use crate::models::post::;
mod api;
mod models;
mod store;
mod utils;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!(
        "Hello, {}! from {}",
        name,
        crate::utils::constants::ESSENTIALS_APP_NAME
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
async fn init(args: crate::models::types::CanisterInitArgs) {
    with_write_state(|state| {
        state.canister_meta_data.insert(
            0,
            CanisterMetaData {
                asset_canister: args.asset_canister,
                controllers: args.controllers,
                post_canister: args.post_canister,
                all_post_canisters: {
                    let mut post_canisters = std::collections::HashSet::new();
                    post_canisters.insert(args.post_canister);
                    post_canisters
                },
            },
        );
    });

}

ic_cdk::export_candid!();
