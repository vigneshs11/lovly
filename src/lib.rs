//! LovlyDB — WASM core. SIMD HNSW and encrypted traversal will live here.

use wasm_bindgen::prelude::*;

/// Build metadata so the host can confirm the WASM module loaded.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn ping() -> String {
    "pong".to_string()
}
