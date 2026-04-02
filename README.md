# LovlyDB

Browser-native vector search: Rust core compiled to **WebAssembly** (`wasm-bindgen`), with a TypeScript SDK planned later. The repo is **not** pure Rust—you need the WASM toolchain and (optionally) Node for scripts.

## Prerequisites

1. **Rust** — [rustup](https://rustup.rs/) (stable is fine).
2. **WASM target** — `rustup target add wasm32-unknown-unknown`
3. **wasm-pack** — `cargo install wasm-pack` (or [installer](https://rustwasm.github.io/wasm-pack/installer/))
4. **Optional: Node.js** — for `npm run build` convenience; you can call `wasm-pack` directly without Node.

## Build

```bash
cargo test
cargo check --target wasm32-unknown-unknown
wasm-pack build --target web --out-dir pkg
```

Or: `npm run build` (same as above).

### WASM-SIMD

`.cargo/config.toml` passes `target-feature=+simd128` for `wasm32-unknown-unknown`, so `wasm-pack` / `cargo check --target wasm32-unknown-unknown` use SIMD cosine automatically. Native `cargo test` uses the scalar path.

## Layout

| Path | Role |
|------|------|
| `src/lib.rs` | WASM exports: `version`, `ping`, `wasm_cosine_similarity`, `wasm_cosine_distance` |
| `src/math/` | Cosine/dot/norms — SIMD128 on wasm32, scalar on host tests |
| `src/hnsw/layer0.rs` | Layer-0 graph + greedy local search |
| `pkg/` | Generated JS + WASM (gitignored); import from your app |

## License

MIT OR Apache-2.0 (see `Cargo.toml`).
