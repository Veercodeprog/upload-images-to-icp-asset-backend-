// src/certification.rs
use crate::state::State;
use crate::types::{Asset, AssetKey};
use crate::STATE; // Import STATE
use sha2::Digest; // Import the Digest trait
use std::collections::HashMap;

pub struct AssetHashes {
    pub hashes: HashMap<AssetKey, Vec<u8>>,
}

impl Default for AssetHashes {
    fn default() -> Self {
        AssetHashes {
            hashes: HashMap::new(),
        }
    }
}

pub fn on_asset_change(asset_hashes: &mut AssetHashes, key: &str, asset: &Asset) {
    // Update the certification hashes for the asset
    if let Some(encoding) = asset.encodings.get("identity") {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&encoding.sha256);
        let hash = hasher.finalize().to_vec();
        asset_hashes.hashes.insert(key.to_string(), hash);
    }
}

pub fn get_root_hash() -> Vec<u8> {
    // Compute the root hash of the certification tree
    let mut hasher = sha2::Sha256::new();
    // Collect all hashes from the asset_hashes
    STATE.with(|state| {
        let asset_hashes = &state.borrow().asset_hashes;
        for hash in asset_hashes.hashes.values() {
            hasher.update(hash);
        }
    });
    hasher.finalize().to_vec()
}

