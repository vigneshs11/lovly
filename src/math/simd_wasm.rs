//! SIMD128 dot / norm for wasm32 — 4-wide `f32` in the hot loop.

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use core::arch::wasm32::*;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[inline]
fn horizontal_sum_f32x4(v: v128) -> f32 {
    f32x4_extract_lane::<0>(v)
        + f32x4_extract_lane::<1>(v)
        + f32x4_extract_lane::<2>(v)
        + f32x4_extract_lane::<3>(v)
}

/// `a · b` using SIMD where possible; falls back to scalar tails.
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn dot_simd_inner(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    let mut acc = f32x4_splat(0.0);
    let mut i = 0usize;
    while i + 4 <= len {
        let va = core::ptr::read_unaligned(a.as_ptr().add(i) as *const v128);
        let vb = core::ptr::read_unaligned(b.as_ptr().add(i) as *const v128);
        acc = f32x4_add(acc, f32x4_mul(va, vb));
        i += 4;
    }
    let mut sum = horizontal_sum_f32x4(acc);
    while i < len {
        sum += a[i] * b[i];
        i += 1;
    }
    sum
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn norm_sq_simd_inner(a: &[f32]) -> f32 {
    let len = a.len();
    let mut acc = f32x4_splat(0.0);
    let mut i = 0usize;
    while i + 4 <= len {
        let va = core::ptr::read_unaligned(a.as_ptr().add(i) as *const v128);
        acc = f32x4_add(acc, f32x4_mul(va, va));
        i += 4;
    }
    let mut sum = horizontal_sum_f32x4(acc);
    while i < len {
        let x = a[i];
        sum += x * x;
        i += 1;
    }
    sum
}

/// Cosine similarity with SIMD on wasm32+simd128; otherwise caller should use scalar.
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    unsafe {
        let dot_ab = dot_simd_inner(a, b);
        let na = norm_sq_simd_inner(a).sqrt();
        let nb = norm_sq_simd_inner(b).sqrt();
        let denom = na * nb;
        if denom == 0.0 {
            return None;
        }
        Some(dot_ab / denom)
    }
}

/// When SIMD is off (e.g. tests on host), use scalar.
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    super::scalar::cosine_similarity(a, b)
}
