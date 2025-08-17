# Tiny Chess

A minimal chess engine library designed to be compiled to WASM and used in Node.js runtime.

## Goals

### Core Functionality
- [x] **FEN Parsing** - Parse FEN strings into board state
- [x] **FEN Generation** - Convert board state back to FEN string
- [x] **Move Generation** - Generate all legal moves for a position
- [x] **Move Validation** - Validate if a move is legal in current position
- [x] **Move Execution** - Apply moves to board state

### Game State Management
- [ ] **Game State Validation** - Detect check, checkmate, stalemate, draw

### WASM Integration
- [ ] **WASM Bindings** - Expose public API for JavaScript
