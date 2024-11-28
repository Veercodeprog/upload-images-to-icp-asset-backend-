// src/lib.rs
use candid::CandidType;
use candid::{candid_method, Principal};
use ic_cdk::api::{call::CallResult, data_certificate, set_certified_data, time, trap};
use ic_cdk_macros::{init, post_upgrade, query, update};
use std::cell::RefCell;
use std::collections::BTreeMap; // Import BTreeMap, which is Rust's implementation of RBTree
mod certification;
mod rc_bytes;
// mod http;
mod state;
mod types;
mod utils;

use crate::certification::get_root_hash;
use crate::state::State;
use crate::types::{AssetKey, HttpRequest, HttpResponse, StoreArg};
use types::Asset;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static ASSETS: RefCell<BTreeMap<String, Asset>> = RefCell::new(BTreeMap::new());
}

#[init]
fn init() {
    STATE.with(|state| {
        // Initialize the state if needed
    });
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|state| {
        // Handle post-upgrade logic if needed
    });
}

#[update]
fn store(arg: StoreArg) {
    STATE.with(|state| {
        if let Err(msg) = state.borrow_mut().store(arg, time()) {
            trap(&msg);
        }
        // Update certified data
        set_certified_data(&get_root_hash());
    });
}

#[query]
fn retrieve(key: AssetKey) -> Vec<u8> {
    STATE.with(|state| match state.borrow().retrieve(&key) {
        Ok(content) => content,
        Err(msg) => trap(&msg),
    })
}

#[query]
fn list_assets() -> Vec<AssetKey> {
    STATE.with(|state| state.borrow().list_assets())
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let certificate = data_certificate().unwrap_or_else(|| trap("No data certificate available"));

    STATE.with(|state| state.borrow().handle_http_request(req, &certificate))
}

#[query]
fn http_request_streaming_callback(
    token: types::StreamingCallbackToken,
) -> types::StreamingCallbackHttpResponse {
    STATE.with(|state| {
        state
            .borrow()
            .handle_streaming_callback(token)
            .unwrap_or_else(|e| {
                trap(&format!("Streaming callback failed: {}", e));
            })
    })
}

use ic_cdk_macros::query as export_query;

ic_cdk::export_candid!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
