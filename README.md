# Tiny Chess

A minimal chess engine library designed to be compiled to WASM and used in Node.js runtime.

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
cargo make build-wasm-dev
cargo make build-wasm-profiling
cargo make build-wasm-release
```

### Publish to npm

```bash
npm publish
```

## Structure

```
tiny-chess/
├── crates/
│   ├── core/ -> Pure Rust implementation of the chess engine
│   ├── wasm/ -> WASM bindings for the chess engine
```

## Goals

### Core Functionality

- [x] **FEN Parsing** - Parse FEN strings into board state
- [x] **FEN Generation** - Convert board state back to FEN string
- [x] **Move Generation** - Generate all legal moves for a position
- [x] **Move Validation** - Validate if a move is legal in current position
- [x] **Move Execution** - Apply moves to board state
- [ ] **Move promotion** - Be able to choose consumer promotion piece

### Game State Management

- [x] **Game Result** - Detect checkmate, stalemate, insufficient material, fifty move rule
- [ ] **Threefold Repetition** - Detect threefold repetition

### WASM Integration

- [ ] **WASM Bindings** - Expose public API for JavaScript
