use crate::models::types::*;
mod apis;
mod models;
mod store;
mod utils;
use core::types::*;
use utils::init::*;

// dexfans v1.0: Index Canister
// author: Vinayak Kalra
// Repo: https://github.com/icP-hub/dexfans/

// init.rs contain the entry point of crate.

ic_cdk::export_candid!();
