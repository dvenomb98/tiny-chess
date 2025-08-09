# Tiny Chess

A minimal chess engine library designed to be compiled to WASM and used in Node.js runtime.

## Goals

### Core Functionality
- [x] **FEN Parsing** - Parse FEN strings into board state
- [ ] **FEN Generation** - Convert board state back to FEN string
- [ ] **Move Representation** - Define move types (normal, castle, en passant, promotion)
- [ ] **Move Generation** - Generate all legal moves for a position
- [ ] **Move Validation** - Validate if a move is legal in current position
- [ ] **Move Execution** - Apply moves to board state
- [ ] **Move Undo** - Revert moves (with state restoration)

### Game State Management
- [ ] **Game State Validation** - Detect check, checkmate, stalemate, draw
- [ ] **Position History** - Track positions for threefold repetition
- [ ] **50-Move Rule** - Track halfmove clock for draw detection
- [ ] **Insufficient Material** - Detect draw by insufficient material

### WASM Integration
- [ ] **WASM Bindings** - Expose public API for JavaScript
- [ ] **Memory Management** - Efficient memory usage for WASM
- [ ] **Error Handling** - Proper error propagation to JavaScript
- [ ] **Serialization** - JSON serialization for complex types

### Essential Chess Rules
- [ ] **Castling Logic** - Validate and execute castling moves
- [ ] **En Passant** - Handle en passant captures
- [ ] **Pawn Promotion** - Handle pawn promotion to any piece
- [ ] **Check Detection** - Detect when king is in check
- [ ] **Pin Detection** - Handle pinned pieces correctly
