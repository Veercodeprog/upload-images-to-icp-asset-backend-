// src/state.rs
use crate::certification::{create_asset_witness, verify_asset_integrity};
use crate::certification::{on_asset_change, AssetHashes};
use crate::types::{Asset, AssetEncoding, HttpResponse, StreamingCallbackToken, StreamingStrategy};

use crate::rc_bytes::RcBytes;
use crate::types::*;
use crate::utils::url_decode;
use base64::prelude::*;
use ic_cdk::api::trap;

use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct State {
    pub assets: HashMap<AssetKey, Asset>,
    pub asset_hashes: AssetHashes,
}

impl Default for State {
    fn default() -> Self {
        State {
            assets: HashMap::new(),
            asset_hashes: AssetHashes::default(),
        }
    }
}

impl State {
    pub fn store(&mut self, arg: StoreArg, time: u64) -> Result<(), String> {
        // Retrieve or create the asset
        let asset = self.assets.entry(arg.key.clone()).or_insert_with(|| Asset {
            content_type: arg.content_type.clone(),
            encodings: HashMap::new(),
            max_age: None,
            headers: None,
            is_aliased: arg.aliased,
            allow_raw_access: None,
        });

        // Update asset properties
        asset.content_type = arg.content_type.clone();
        asset.is_aliased = arg.aliased;

        // Compute SHA-256 hash of the content
        let hash = Sha256::digest(&arg.content).into();

        // Verify provided SHA-256 hash if present
        if let Some(provided_hash) = arg.sha256 {
            if hash != provided_hash.as_ref() {
                return Err("SHA-256 hash mismatch".to_string());
            }
        }

        // Update or create the encoding
        let encoding = asset
            .encodings
            .entry(arg.content_encoding.clone())
            .or_insert_with(|| AssetEncoding {
                modified: time,
                content_chunks: vec![],
                total_length: 0,
                certified: false,
                sha256: [0; 32],
            });

        encoding.total_length = arg.content.len();
        encoding.content_chunks = vec![RcBytes::from(arg.content)];
        encoding.modified = time;
        encoding.sha256 = hash;

        // Update asset certification
        let updated_asset_hashes =
            on_asset_change(std::mem::take(&mut self.asset_hashes), &arg.key, asset);
        self.asset_hashes = updated_asset_hashes;

        Ok(())
    }

    // Add this method to your State implementation
    fn update_asset_certification(&mut self, key: &str, asset: &Asset) {
        let updated_asset_hashes =
            on_asset_change(std::mem::take(&mut self.asset_hashes), key, asset);
        self.asset_hashes = updated_asset_hashes;
    }

    pub fn retrieve(&self, key: &AssetKey) -> Result<Vec<u8>, String> {
        let asset = self
            .assets
            .get(key)
            .ok_or_else(|| "Asset not found".to_string())?;
        let encoding = asset
            .encodings
            .get("identity")
            .ok_or_else(|| "No identity encoding".to_string())?;
        let content: Vec<u8> = encoding
            .content_chunks
            .iter()
            .flat_map(|chunk| chunk.to_vec())
            .collect();

        Ok(content)
    }

    pub fn list_assets(&self) -> Vec<AssetKey> {
        self.assets.keys().cloned().collect()
    }
    pub fn handle_http_request(&self, req: HttpRequest, certificate: &[u8]) -> HttpResponse {
        let path = match url_decode(&req.url) {
            Ok(decoded_path) => decoded_path,
            Err(err) => {
                return HttpResponse {
                    status_code: 400,
                    headers: vec![],
                    body: RcBytes::from(ByteBuf::from(format!("Failed to decode path: {}", err))),
                    upgrade: None,
                    streaming_strategy: None,
                }
            }
        };

        let mut response = self.build_http_response(&path);

        // Generate witness
        let witness = create_asset_witness(&path);

        // Create IC-Certificate header
        let ic_certificate_value = format!(
            "certificate=:{}:, tree=:{}:",
            BASE64_STANDARD.encode(certificate),
            BASE64_STANDARD.encode(&witness)
        );

        response
            .headers
            .push(("IC-Certificate".to_string(), ic_certificate_value));

        response
    }

    fn build_http_response(&self, path: &str) -> HttpResponse {
        if let Some(asset) = self.assets.get(path) {
            // Get the encoding
            if let Some(encoding) = asset.encodings.get("identity") {
                // Verify asset integrity
                if !verify_asset_integrity(path, encoding.content_chunks[0].as_ref()) {
                    return HttpResponse {
                        status_code: 500,
                        headers: vec![],
                        body: RcBytes::from(ByteBuf::from("Asset integrity check failed")),
                        upgrade: None,
                        streaming_strategy: None,
                    };
                }

                // Create asset witness
                let witness = create_asset_witness(path);

                // Build the response
                HttpResponse {
                    status_code: 200,
                    headers: vec![
                        ("Content-Type".to_string(), asset.content_type.clone()),
                        (
                            "Content-Length".to_string(),
                            encoding.total_length.to_string(),
                        ),
                        (
                            "IC-Certificate-Witness".to_string(),
                            BASE64_STANDARD.encode(&witness),
                        ),
                        (
                            "Strict-Transport-Security".to_string(),
                            "max-age=31536000; includeSubDomains".to_string(),
                        ),
                        ("X-Frame-Options".to_string(), "DENY".to_string()),
                        ("X-Content-Type-Options".to_string(), "nosniff".to_string()),
                    ],
                    body: encoding.content_chunks[0].clone(),
                    upgrade: None,
                    streaming_strategy: None,
                }
            } else {
                // No suitable encoding found
                HttpResponse {
                    status_code: 404,
                    headers: vec![],
                    body: RcBytes::from(ByteBuf::from("Asset encoding not found")),
                    upgrade: None,
                    streaming_strategy: None,
                }
            }
        } else {
            // Asset not found
            HttpResponse {
                status_code: 404,
                headers: vec![],
                body: RcBytes::from(ByteBuf::from("Asset not found")),
                upgrade: None,
                streaming_strategy: None,
            }
        }
    }
    pub fn handle_streaming_callback(
        &self,
        token: StreamingCallbackToken,
    ) -> Result<StreamingCallbackHttpResponse, String> {
        // Implement streaming logic if needed
        Err("Streaming not implemented".to_string())
    }
}
