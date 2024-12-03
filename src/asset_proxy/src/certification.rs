// src/certification.rs
use crate::state::State;
use crate::types::{Asset, AssetKey};
use crate::STATE;
use ic_cdk::api::set_certified_data;
use ic_certified_map::{AsHashTree, Hash, RbTree};
use serde_cbor;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct AssetHashes {
    pub hashes: RbTree<AssetKey, Hash>,
}

impl Default for AssetHashes {
    fn default() -> Self {
        AssetHashes {
            hashes: RbTree::new(),
        }
    }
}

pub fn on_asset_change(mut asset_hashes: AssetHashes, key: &str, asset: &Asset) -> AssetHashes {
    if let Some(encoding) = asset.encodings.get("identity") {
        let hash = Hash::from(encoding.sha256);
        asset_hashes.hashes.insert(key.to_string(), hash);
    }
    asset_hashes
}
pub fn get_root_hash(asset_hashes: &AssetHashes) -> Hash {
    asset_hashes.hashes.root_hash()
}

// pub fn get_root_hash() -> Hash {
//     STATE.with(|state| {
//         let asset_hashes = &state.borrow().asset_hashes;
//         reconstruct(&asset_hashes.hashes.as_hash_tree())
//     })
// }

pub fn update_certified_data(asset_hashes: &AssetHashes) {
    let root_hash = asset_hashes.hashes.root_hash();
    set_certified_data(&root_hash);
}
pub fn create_asset_witness(key: &str) -> Vec<u8> {
    STATE.with(|state| {
        let asset_hashes = &state.borrow().asset_hashes;
        let witness = asset_hashes.hashes.witness(key.as_bytes());
        // Convert the HashTree to Vec<u9> using CBOR encoding
        serde_cbor::to_vec(&witness).unwrap_or_default()
    })
}

pub fn verify_asset_integrity(key: &str, content: &[u8]) -> bool {
    STATE.with(|state| {
        let asset_hashes = &state.borrow().asset_hashes;
        if let Some(stored_hash) = asset_hashes.hashes.get(key.as_bytes()) {
            let mut hasher = Sha256::new();
            hasher.update(content);
            let hash_result = hasher.finalize();

            // Convert hash_result to [u8; 32]
            let computed_hash: [u8; 32] = hash_result.into(); // Explicit conversion

            // Assuming stored_hash is of type [u8; 32] or similar
            computed_hash == *stored_hash
        } else {
            false
        }
    })
}
