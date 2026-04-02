//! Vector math: cosine / dot (scalar + wasm SIMD128).

mod scalar;
mod simd_wasm;

pub use scalar::{cosine_distance, dot, norm_sq}; // dot/norm_sq: public metrics API
pub use simd_wasm::cosine_similarity;
