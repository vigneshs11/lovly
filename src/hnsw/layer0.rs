//! HNSW layer 0: flat graph + greedy local search (Day 3 baseline).

use crate::math;

/// Layer-0 graph: `dim`-dimensional vectors row-major in `vectors`, adjacency in `neighbors`.
#[derive(Debug, Clone)]
pub struct GraphLayer0 {
    dim: usize,
    vectors: Vec<f32>,
    neighbors: Vec<Vec<usize>>,
}

impl GraphLayer0 {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            vectors: Vec::new(),
            neighbors: Vec::new(),
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn len(&self) -> usize {
        self.neighbors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.neighbors.is_empty()
    }

    /// Append a vector (length `dim`) and its layer-0 neighbor ids (within current index range).
    pub fn push(&mut self, vector: &[f32], neighbors: Vec<usize>) -> Option<usize> {
        if vector.len() != self.dim {
            return None;
        }
        let id = self.len();
        for &n in &neighbors {
            if n >= id {
                return None;
            }
        }
        self.vectors.extend_from_slice(vector);
        self.neighbors.push(neighbors);
        Some(id)
    }

    /// Replace adjacency for `id`. Every neighbor must be a valid distinct index.
    pub fn set_neighbors(&mut self, id: usize, neighbors: Vec<usize>) -> bool {
        if id >= self.len() {
            return false;
        }
        for &n in &neighbors {
            if n >= self.len() || n == id {
                return false;
            }
        }
        self.neighbors[id] = neighbors;
        true
    }

    fn vector(&self, id: usize) -> Option<&[f32]> {
        if id >= self.len() {
            return None;
        }
        let d = self.dim;
        Some(&self.vectors[id * d..(id + 1) * d])
    }

    /// Greedy climb: from `entry`, repeatedly move to the neighbor most similar to `query` until local maximum.
    /// Returns the best node id visited.
    pub fn greedy_search(&self, query: &[f32], entry: usize) -> Option<usize> {
        if query.len() != self.dim || entry >= self.len() {
            return None;
        }
        let mut current = entry;
        loop {
            let cur_v = self.vector(current)?;
            let cur_sim = math::cosine_similarity(query, cur_v)?;

            let mut best = current;
            let mut best_sim = cur_sim;

            for &n in &self.neighbors[current] {
                if let Some(nv) = self.vector(n) {
                    if let Some(sim) = math::cosine_similarity(query, nv) {
                        if sim > best_sim {
                            best_sim = sim;
                            best = n;
                        }
                    }
                }
            }

            if best == current {
                return Some(current);
            }
            current = best;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greedy_finds_best_neighbor() {
        let mut g = GraphLayer0::new(2);
        g.push(&[1.0f32, 0.0], vec![]).unwrap();
        g.push(&[0.99, 0.01], vec![0]).unwrap();
        g.push(&[0.95, 0.05], vec![1]).unwrap();
        assert!(g.set_neighbors(0, vec![1]));
        assert!(g.set_neighbors(1, vec![0, 2]));

        let q = [0.9f32, 0.1];
        let best = g.greedy_search(&q, 0).unwrap();
        assert_eq!(best, 2);
    }
}
