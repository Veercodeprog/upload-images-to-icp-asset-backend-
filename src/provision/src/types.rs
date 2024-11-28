// src/types.rs
use candid::{CandidType, Deserialize as CandidDeserialize, Principal};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::rc_bytes::RcBytes;

/// Represents a collection request with all necessary details.
#[derive(CandidType, CandidDeserialize, Serialize, Clone)]
pub struct CarCollection {
    pub id: u64,
    pub name: String,
    pub model: String,
    pub logo: String,           // Asset reference (e.g., URL or asset ID)
    pub images: Vec<String>,    // List of asset references
    pub documents: Vec<String>, // List of asset references
    pub owner: Principal,
    pub approved: bool,
}

/// Represents an asset with various configurations.
#[derive(CandidType, CandidDeserialize, Serialize, Clone)]
pub struct Asset {
    pub id: String, // Unique identifier for the asset
    pub content_type: String,
    pub encodings: BTreeMap<String, AssetEncoding>,
    pub max_age: Option<u64>,
    pub headers: Option<BTreeMap<String, String>>,
    pub is_aliased: Option<bool>,
    pub allow_raw_access: Option<bool>,
    pub owner: Principal,
}

/// Represents the encoding details of an asset.
#[derive(CandidType, CandidDeserialize, Serialize, Clone)]
pub struct AssetEncoding {
    pub modified: u64,
    pub content_chunks: Vec<RcBytes>,
    pub total_length: usize,
    pub certified: bool,
    pub sha256: [u8; 32],
}

