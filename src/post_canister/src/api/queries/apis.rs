#[ic_cdk::query]
fn greet(name: String) -> String {
    format!(
        "Hello, {}! from {}",
        name,
        core::constants::ESSENTIALS_APP_NAME
    )
}

#[ic_cdk::query]
fn get_all_controllers() -> std::collections::HashSet<candid::Principal> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(data) => data.controllers,
        None => std::collections::HashSet::new(),
    })
}

// debug
#[ic_cdk::query]
fn debug_total_posts() -> u128 {
    crate::with_read_state(|state| state.post_counter)
}