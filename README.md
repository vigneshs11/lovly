# LovlyDB

Browser-native vector search: Rust core compiled to **WebAssembly** (`wasm-bindgen`), with a TypeScript SDK planned later. The repo is **not** pure Rust—you need the WASM toolchain and (optionally) Node for scripts.

## Prerequisites

1. **Rust** — [rustup](https://rustup.rs/) (stable is fine).
2. **WASM target** — `rustup target add wasm32-unknown-unknown`
3. **wasm-pack** — `cargo install wasm-pack` (or [installer](https://rustwasm.github.io/wasm-pack/installer/))
4. **Optional: Node.js** — for `npm run build` convenience; you can call `wasm-pack` directly without Node.

## Build

```bash
cargo check
wasm-pack build --target web --out-dir pkg
```

Or: `npm run build` (same as above).

### WASM-SIMD (Day 2+)

Enable SIMD128 for release builds (PowerShell):

```powershell
$env:RUSTFLAGS = "-C target-feature=+simd128"
wasm-pack build --target web --out-dir pkg --release
```

Bash:

```bash
RUSTFLAGS='-C target-feature=+simd128' wasm-pack build --target web --out-dir pkg --release
```

## Layout

| Path | Role |
|------|------|
| `src/lib.rs` | WASM exports (`wasm-bindgen`) |
| `pkg/` | Generated JS + WASM (gitignored); import from your app |

## License

MIT OR Apache-2.0 (see `Cargo.toml`).
