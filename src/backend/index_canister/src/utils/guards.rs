// to prevent anonymous calls
pub fn guard_prevent_anonymous() -> Result<(), String> {
    if ic_cdk::api::caller() == candid::Principal::anonymous() {
        return Err(String::from(core::constants::WARNING_ANONYMOUS_CALL));
    }

    Ok(())
}

pub fn guard_super_controller() -> core::types::Response {
    if ic_cdk::api::is_controller(&ic_cdk::api::caller()) {
        return Ok(());
    }

    Err(String::from(core::constants::WARNING_ADMIN_ONLY))
}


// controllers accesss
pub fn guard_only_admin() -> Result<(), String> {
    guard_prevent_anonymous()?;

    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(_val) => {
            let x = _val.controllers.contains(&ic_cdk::api::caller());
            if x {
                return Ok(());
            }
            Err(String::from(core::constants::WARNING_ADMIN_ONLY))
        }
        None => Err(String::from(core::constants::WARNING_ADMIN_ONLY)),
    })
}

// post canister only
pub fn guard_post_canister_exclusive() -> Result<(), String> {
    guard_prevent_anonymous()?;

    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            for canister in val.all_post_canisters.iter() {
                if canister == &ic_cdk::api::caller() {
                    return Ok(());
                }
            }
            Err(String::from(core::constants::ERROR_UNAUTHORIZED))
        }
        None => Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA)),
    })
}

pub fn validate_post_canister(id: candid::Principal) -> core::types::Response {
    let meta_data = crate::with_read_state(|state| state.canister_meta_data.get(&0))
        .expect(core::constants::ERROR_FAILED_CANISTER_DATA)
        .all_post_canisters;

    if meta_data.contains(&id) {
        Ok(())
    } else {
        Err(String::from(core::constants::ERROR_INVALID_CANISTER))
    }
}
