use crate::models::post::{CreatePostArgs, Post};
use store::storage_state::ApplicationState;
mod api;
mod models;
mod store;
mod utils;
use candid::Principal;

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
async fn init(args: dexfans_types::types::PostCanisterInitArgs) {
    with_write_state(|state| {
        for x in args.accounts.iter() {
            state.account.insert(
                x.user_id,
                crate::models::types::UserProfileIC {
                    user_id: x.user_id,
                    membership: x.membership.to_owned(),
                    username: x.username.to_owned(),

                    ..Default::default()
                },
            );
        }

        state.canister_meta_data.insert(
            0,
            crate::models::types::CanisterMetaData {
                canister_ids: args.canister_ids,
                controllers: args.controllers,
            },
        )
    });
}

// // for development only
// #[ic_cdk::query]
// fn get_canister_meta_data() -> crate::models::types::CanisterMetaData {
//     with_read_state(|state| state.canister_meta_data.get(&0).unwrap())
// }

ic_cdk::export_candid!();
