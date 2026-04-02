//! LovlyDB — WASM core. SIMD HNSW and encrypted traversal will live here.

mod hnsw;
mod math;

pub use hnsw::GraphLayer0;
pub use math::{cosine_distance, cosine_similarity, dot, norm_sq};

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

/// Cosine similarity in `[-1, 1]`. Errors if lengths differ, empty, or zero vector.
#[wasm_bindgen]
pub fn wasm_cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, JsValue> {
    cosine_similarity(a, b).ok_or_else(|| JsValue::from_str("invalid vectors"))
}

/// Cosine distance `1 - similarity`.
#[wasm_bindgen]
pub fn wasm_cosine_distance(a: &[f32], b: &[f32]) -> Result<f32, JsValue> {
    cosine_distance(a, b).ok_or_else(|| JsValue::from_str("invalid vectors"))
}
