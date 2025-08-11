# Tiny Chess

A minimal chess engine library designed to be compiled to WASM and used in Node.js runtime.

## Goals

### Core Functionality
- [x] **FEN Parsing** - Parse FEN strings into board state
- [x] **FEN Generation** - Convert board state back to FEN string
- [ ] **Move Generation** - Generate all legal moves for a position
- [ ] **Move Validation** - Validate if a move is legal in current position
- [ ] **Move Execution** - Apply moves to board state

### Game State Management
- [ ] **Game State Validation** - Detect check, checkmate, stalemate, draw
- [ ] **50-Move Rule** - Track halfmove clock for draw detection
- [ ] **Insufficient Material** - Detect draw by insufficient material

### WASM Integration
- [ ] **WASM Bindings** - Expose public API for JavaScript
- [ ] **Memory Management** - Efficient memory usage for WASM
- [ ] **Error Handling** - Proper error propagation to JavaScript
- [ ] **Serialization** - JSON serialization for complex types
