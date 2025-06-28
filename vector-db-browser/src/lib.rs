use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct VectorStore {
    store: HashMap<String, Vec<f32>>,
}

#[wasm_bindgen]
impl VectorStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VectorStore {
        VectorStore {
            store: HashMap::new(),
        }
    }

#[wasm_bindgen]
pub fn add_vector(&mut self, id: String, values: Vec<f32>) {
        self.store.insert(id, values);
    }

#[wasm_bindgen]
pub fn search(&self, query: Vec<f32>, top_k: usize) -> JsValue {
        let mut scored: Vec<ScoredResult> = self.store.iter()
            .filter_map(|(id, vec)| {
                if vec.len() != query.len() {
                    None
                } else {
                    Some(ScoredResult {
                        id: id.clone(),
                        score: cosine_similarity(&query, vec),
                    })
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        let top_k_results: Vec<_> = scored.into_iter().take(top_k).collect();

        JsValue::from_serde(&top_k_results).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct ScoredResult {
    id: String,
    score: f32,
}

fn cosine_similarity(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
