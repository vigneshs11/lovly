//! Scalar cosine / dot — baseline and host tests.

/// Dot product `a · b`. Lengths must match.
#[inline]
pub fn dot(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() {
        return None;
    }
    let mut s = 0.0f32;
    for i in 0..a.len() {
        s += a[i] * b[i];
    }
    Some(s)
}

/// Sum of squares `‖a‖²`.
#[inline]
pub fn norm_sq(a: &[f32]) -> f32 {
    a.iter().map(|x| x * x).sum()
}

/// Cosine similarity `a·b / (‖a‖ ‖b‖)`. `None` if lengths differ, empty, or a zero vector.
#[inline]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    let dot_ab = dot(a, b)?;
    let na = norm_sq(a).sqrt();
    let nb = norm_sq(b).sqrt();
    let denom = na * nb;
    if denom == 0.0 {
        return None;
    }
    Some(dot_ab / denom)
}

/// Cosine **distance** `1 - cos_sim` in `[0, 2]` for unit-scale semantics.
#[inline]
pub fn cosine_distance(a: &[f32], b: &[f32]) -> Option<f32> {
    cosine_similarity(a, b).map(|c| 1.0 - c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_orthogonal() {
        let a = [1.0f32, 0.0];
        let b = [0.0f32, 1.0];
        assert!((cosine_similarity(&a, &b).unwrap() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_same_direction() {
        let a = [1.0f32, 2.0, 3.0];
        let b = [2.0f32, 4.0, 6.0];
        assert!((cosine_similarity(&a, &b).unwrap() - 1.0).abs() < 1e-5);
    }
}
