# 🏁 tiny-chess-wasm

[![npm version](https://img.shields.io/npm/v/tiny-chess-wasm.svg)](https://www.npmjs.com/package/tiny-chess-wasm)
[![license](https://img.shields.io/npm/l/tiny-chess-wasm.svg)](https://github.com/danielbilek/tiny-chess/blob/main/LICENSE.md)

A minimal, fast chess engine written in Rust and compiled to WebAssembly. Perfect for chess applications, games, and educational projects.

## 🛠️ Requirements

- **Node.js**: 16+ 
- **Browsers**: Modern browsers with WebAssembly support
- **TypeScript**: 4.0+ (optional, but recommended)


## ✨ Features

- **🚀 Fast**: Rust performance compiled to WebAssembly
- **📦 Lightweight**: Minimal bundle size with `wee_alloc`
- **🛡️ Type Safe**: Full TypeScript definitions included

## 📥 Installation

```bash
npm install tiny-chess-wasm
```

## 🚀 Quick Start

```typescript
import { WasmChess } from 'tiny-chess-wasm';

// Create a new game (starts with standard opening position)
const chess = new WasmChess();

// Or load from FEN string
const chess = new WasmChess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

// Get legal moves for a piece at position (row, col)
const moves = chess.getMoves(1, 4); // Get moves for white pawn at e2

// Make a move
// chess.movePiece(moves[0])
chess.movePiece({
  from_row_idx: 1,
  from_col_idx: 4,
  to_row_idx: 3,
  to_col_idx: 4,
  is_passant: false,
  is_castle: false,
  piece: "WhitePawn"
});

// Get current position as FEN
const currentFen = chess.toFen();

// Check game result
const result = chess.getGameResult();
if (result === "WhiteCheckmate") {
  console.log("White wins!");
}
```

## 🎮 API Reference

### Constructor

```typescript
new WasmChess(fen?: string)
```
- `fen` (optional): FEN string for initial position. Defaults to standard starting position.

### Methods

#### `get_moves(row: number, col: number): Move[]`
Get all legal moves for a piece at the specified position.

#### `get_pseudo_moves(row: number, col: number): Move[]`
Get all pseudo-legal moves (may leave king in check).

#### `move_piece(move: Move): void`
Execute a move and update the game state.

#### `validate_move(move: Move): boolean`
Check if a move is legal without executing it.

#### `to_fen(): string`
Get the current position as a FEN string.

#### `get_game_result(): GameResult | null`
Get the game result or `null` if game is ongoing.

#### `square_to_chess_notation(row: number, col: number): string | null`
Convert a square to chess notation (e.g., "e4", "a1").

#### `square_from_chess_notation(notation: string): Square | null`
Convert a chess notation to a square (e.g., "e4", "a1").

### Types

```typescript
export type Board = (PieceType | null)[][];

export type ParsedFen = {
  board: Board;
  state: ParsedFenState;
}

export type ParsedFenState = {
  en_passant_square: Square | null;
  on_turn: Player;
  castle_white_short: boolean;
  castle_white_long: boolean;
  castle_black_short: boolean;
  castle_black_long: boolean;
  half_moves: number;
  full_moves: number;
}

export type Square = {
  row: number;
  col: number;
}

export type Move = {
  from_col_idx: number;
  from_row_idx: number;
  to_col_idx: number;
  to_row_idx: number;
  is_passant: boolean;
  is_castle: boolean;
  piece: PieceType;
}

export type Moves = Move[];

export type GameResult = "WhiteCheckmate" | "BlackCheckmate" | "Stalemate" | "InsufficientMaterial" | "FiftyMoveRule" | "ThreefoldRepetition" | null;

export type Player = "White" | "Black";

export type PieceType = 
  | "WhitePawn" | "WhiteRook" | "WhiteBishop" | "WhiteKnight" | "WhiteQueen" | "WhiteKing"
  | "BlackPawn" | "BlackRook" | "BlackBishop" | "BlackKnight" | "BlackQueen" | "BlackKing";
```

## 🔧 Utility Functions

Static utility functions for working with FEN strings:

```typescript
import { parseFen, stringifyFen } from 'wasm-chess';

// Parse FEN to game object
const parsedGame = parseFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

// Convert game object back to FEN
const fenString = stringifyFen(parsedGame);
```

## 🎨 Board Coordinate System

The engine uses a 0-indexed coordinate system:
- **Rows**: 0 (rank 1) to 7 (rank 8)
- **Columns**: 0 (file A) to 7 (file H)

Example: Square "e4" = `{ row: 3, col: 4 }`

## 📄 License

MIT © [Daniel Bilek](https://github.com/danielbilek)

## 🔗 Links

- [GitHub Repository](https://github.com/danielbilek/tiny-chess)
- [Issues & Bug Reports](https://github.com/danielbilek/tiny-chess/issues)
- [npm Package](https://www.npmjs.com/package/tiny-chess-wasm)
