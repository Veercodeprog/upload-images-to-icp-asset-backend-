// src/lib.rs
use std::cell::RefCell;
use std::collections::HashMap;

use candid::Principal;
use ic_cdk::api::caller;
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
mod rc_bytes;
mod types;
use types::{Asset, CarCollection};

// Global state for assets and collections
thread_local! {
    static ASSETS: RefCell<HashMap<String, Asset>> = RefCell::default();
    static COLLECTIONS: RefCell<HashMap<u64, CarCollection>> = RefCell::default();
}

#[init]
fn init() {
    // Initialization logic can be added here if needed
}

#[pre_upgrade]
fn pre_upgrade() {
    let collections = COLLECTIONS.with(|c| c.borrow().clone());
    let assets = ASSETS.with(|a| a.borrow().clone());
    storage::stable_save((collections, assets)).expect("Failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (collections, assets): (HashMap<u64, CarCollection>, HashMap<String, Asset>) =
        storage::stable_restore().expect("Failed to restore stable state");
    COLLECTIONS.with(|c| *c.borrow_mut() = collections);
    ASSETS.with(|a| *a.borrow_mut() = assets);
}

#[update]
fn add_car_collection(car: CarCollection) -> Result<(), String> {
    COLLECTIONS.with(|collections| {
        if collections.borrow().contains_key(&car.id) {
            Err("Collection with this ID already exists".to_string())
        } else {
            collections.borrow_mut().insert(car.id, car);
            Ok(())
        }
    })
}

#[query]
fn get_car_collection(id: u64) -> Option<CarCollection> {
    COLLECTIONS.with(|collections| collections.borrow().get(&id).cloned())
}

#[update]
fn add_asset(mut asset: Asset) -> Result<(), String> {
    asset.owner = caller();
    ASSETS.with(|assets| {
        if assets.borrow().contains_key(&asset.id) {
            Err("Asset with this ID already exists".to_string())
        } else {
            assets.borrow_mut().insert(asset.id.clone(), asset);
            Ok(())
        }
    })
}

#[query]
fn get_asset(id: String) -> Option<Asset> {
    ASSETS.with(|assets| assets.borrow().get(&id).cloned())
}

// Export the candid interface
use ic_cdk_macros::query as export_query;

ic_cdk::export_candid!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
