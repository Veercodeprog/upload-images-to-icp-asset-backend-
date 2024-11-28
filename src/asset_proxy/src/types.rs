// src/types.rs
use crate::rc_bytes::RcBytes;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde_bytes::ByteBuf;
use std::collections::HashMap;
pub type AssetKey = String;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StoreArg {
    pub key: AssetKey,
    pub content_type: String,
    pub content_encoding: String,
    pub content: ByteBuf,
    pub sha256: Option<ByteBuf>,
    pub aliased: Option<bool>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Asset {
    pub content_type: String,
    pub encodings: HashMap<String, AssetEncoding>,
    pub max_age: Option<u64>,
    pub headers: Option<HashMap<String, String>>,
    pub is_aliased: Option<bool>,
    pub allow_raw_access: Option<bool>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AssetEncoding {
    pub modified: u64,
    pub content_chunks: Vec<RcBytes>,
    pub total_length: usize,
    pub certified: bool,
    pub sha256: [u8; 32],
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
    pub streaming_strategy: Option<StreamingStrategy>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackToken {
    pub key: String,
    pub content_encoding: String,
    pub index: Nat,
    pub sha256: Option<ByteBuf>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackHttpResponse {
    pub body: ByteBuf,
    pub token: Option<StreamingCallbackToken>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback {
        callback: candid::Func,
        token: StreamingCallbackToken,
    },
}
