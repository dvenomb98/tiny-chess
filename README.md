# Tiny Chess

A minimal chess engine library designed to be compiled to WASM.

## Prerequisites

Before building this project, ensure you have the following tools installed:

- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **wasm-pack** - WebAssembly build tool
  ```bash
  cargo install wasm-pack
  ```
- **cargo-make** - Task runner for Rust projects
  ```bash
  cargo install cargo-make
  ```

### Build WASM bindings

```bash
cargo make build-wasm
```

## Structure

```
tiny-chess/
├── crates/
│   ├── core/ -> Pure Rust implementation of the chess engine
│   ├── wasm/ -> WASM bindings for the chess engine
```
