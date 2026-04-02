# Project LovlyDB: High-Performance, Security-First Browser Vector DB

## 1. Vision & Identity
**LovlyDB** (named for Lakshmi) is a browser-native vector database designed to outperform existing JavaScript engines like Orama and LanceDB. 

- **Core Goal**: Provide enterprise-grade, zero-knowledge vector search entirely within the user's browser sandbox.
- **The "Unfair Advantage"**: Using Rust-based **ANN HNSW** with **WASM-SIMD** to handle 100k+ vectors without the UI "jank" found in JS-based competitors.

## 2. Technical Differentiators: How we do it BETTER


| Feature | LovlyDB (Our Edge) | Competitors (Orama/Others) |
| :--- | :--- | :--- |
| **Search Algo** | **SIMD-Accelerated HNSW (ANN)** | Primarily Flat (KNN) or JS-based Graphs |
| **Engine** | **Rust + WASM** (Off-heap memory) | TypeScript (Main-thread garbage collection) |
| **Hardware** | **WASM-SIMD128** (Parallel math) | Standard JavaScript scalar loops |
| **Scalability** | **Sub-10ms search** on 100k+ vectors | Performance degrades after 10k vectors |
| **Security** | **Encrypted HNSW Traversal** | Unencrypted data in IndexedDB |

## 3. Patent Strategy (Section 3k - Technical Effect)
To secure a patent in India, LovlyDB focuses on these "Technical Advancements" in **ANN HNSW**:
1. **Encrypted HNSW Traversal**: A novel method where the graph "links" (pointers) and vector nodes are stored as encrypted blobs. The search algorithm navigates the graph by decrypting only the immediate "neighbor" nodes in WASM memory, preventing a full data leak.
2. **SIMD-Parallel Distance Logic**: A specific process that uses WASM-SIMD to calculate distances for 4-8 dimensions simultaneously during the HNSW "greedy search" phase.
3. **Hardware-Linked Graph Persistence**: Binding the HNSW index to the browser's hardware-backed `SubtleCrypto` keys, ensuring the index cannot be stolen or run on another machine.

## 4. 14-Day "Sprint to Prototype" Journey Map

### Phase 1: The ANN HNSW Engine (Days 1-5)
- **Day 1: Setup**: Rust + `wasm-pack` env with `simd128` flags enabled.
- **Day 2: SIMD Math Core**: Implement Cosine Similarity using Rust's `std::arch::wasm32`.
- **Day 3: HNSW Level 0**: Build the base graph layer and the "Greedy Search" algorithm in Rust.
- **Day 4: Multi-Layer HNSW**: Implement the hierarchical "express lanes" that allow $O(\log N)$ search speeds.
- **Day 5: Persistence**: Map the HNSW graph structure to **IndexedDB** using serialized ByteBuffers.

### Phase 2: Security & Innovation (Days 6-10)
- **Day 6: The Lovly-Vault**: Wrap the HNSW node ingestion in `crypto.subtle` AES-GCM encryption.
- **Day 7: Encrypted Traversal**: Optimize the WASM loop to handle decryption-on-the-fly during search.
- **Day 8: Scalar Quantization (SQ8)**: Compress 32-bit vectors to 8-bit to fit massive HNSW graphs in browser RAM.
- **Day 9: Graph Blinding**: Obfuscate the "links" between HNSW nodes to hide data relationships (Patent Target).
- **Day 10: Stress Testing**: Prove sub-10ms latency on a 100,000 vector dataset in-browser.

### Phase 3: Developer Experience (Days 11-14)
- **Day 11: Auto-Embedding**: Integrate `Transformers.js` for on-device vector generation.
- **Day 12: Hybrid Search**: Merge HNSW results with BM25 text scores using RRF logic.
- **Day 13: SDK Wrapper**: Finalize the TypeScript wrapper for the WASM core.
- **Day 14: IP Protection**: File **Provisional Patent** for "Secure ANN HNSW Traversal."

## 5. Legal & Compliance
- **Clean Room Design**: Zero reference to Orama/LanceDB source code. Logic built from HNSW whitepapers.
- **Copyright Header**: `Copyright © 2026 LovlyDB. All Rights Reserved.`
