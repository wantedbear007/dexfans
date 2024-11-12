use crate::models::types::*;
mod api;
mod models;
mod store;
mod utils;
use core::types::*;
use utils::init::{with_read_state, with_write_state};

// dexfans v1.0: Index Canister
// author: Vinayak Kalra
// Repo: https://github.com/icP-hub/dexfans/

// init.rs contain the entry point of crate.

ic_cdk::export_candid!();
