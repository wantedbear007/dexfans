thread_local! {
  static STATE: std::cell::RefCell<crate::store::storage_state::ApplicationState> = std::cell::RefCell::new(crate::store::storage_state::ApplicationState::new());
}

// to get mutable reference
pub(crate) fn with_write_state<R>(
    f: impl FnOnce(&mut crate::store::storage_state::ApplicationState) -> R,
) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

// to get inmutable reference
pub(crate) fn with_read_state<R>(
    f: impl FnOnce(&crate::store::storage_state::ApplicationState) -> R,
) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

// init args
#[ic_cdk::init]
async fn init(args: core::types::PostCanisterInitArgs) {
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
