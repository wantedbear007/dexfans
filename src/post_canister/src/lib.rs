use crate::models::post::{CreatePostArgs, Post};
use store::storage_state::ApplicationState;
mod api;
mod models;
mod store;
mod utils;
use utils::init::{with_read_state, with_write_state};

// dexfans v1.0: Post Canister
// author: Vinayak Kalra
// Repo: https://github.com/icP-hub/dexfans/

// init.rs contain the entry point of project.

ic_cdk::export_candid!();
