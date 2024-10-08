use crate::with_read_state;

// for development only
#[ic_cdk::query]
pub fn api_get_all_post() -> Vec<crate::models::post::Post> {
    with_read_state(|state| state.get_all_posts())
}
