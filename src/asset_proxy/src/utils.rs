// src/utils.rs
use percent_encoding::percent_decode_str;

pub fn url_decode(url: &str) -> Result<String, String> {
    percent_decode_str(url)
        .decode_utf8()
        .map(|s| s.to_string())
        .map_err(|e| e.to_string())
}
