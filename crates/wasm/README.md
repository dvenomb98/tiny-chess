# 🏁 wasm-chess

[![npm version](https://img.shields.io/npm/v/wasm-chess.svg)](https://www.npmjs.com/package/wasm-chess)
[![license](https://img.shields.io/npm/l/wasm-chess.svg)](https://github.com/dvenomb98/tiny-chess/blob/main/LICENSE.md)

A minimal, fast chess engine written in Rust and compiled to WebAssembly with **bundler target** for optimal integration with modern JavaScript bundlers and frameworks.

## 🛠️ Requirements

- **Bundler**: Webpack, Vite, Rollup, Parcel, or any modern JavaScript bundler
- **Node.js**: 16+ (for development)
- **TypeScript**: 5.0+ (recommended)

> ⚠️ **Important**: This package is built with `wasm-pack --target bundler` and requires a bundler to work properly. It will not work in vanilla browser environments or pure Node.js without bundling. Modern bundlers handle WASM loading automatically. No additional configuration needed in most cases.

## ✨ Features

- **🚀 Fast**: Rust performance compiled to WebAssembly
- **🛡️ Type Safe**: Full TypeScript definitions included
- **⚡ Universal**: Works in browser, Node.js, and server-side rendering (when bundled)
- **📦 Framework Ready**: Perfect for Next.js, React, Vue, Svelte, and more
- **📜 History Management**: Built-in undo/redo with position navigation

## 📥 Installation

```bash
npm install wasm-chess
```

## 🚀 Quick Start

```typescript
import { WasmChess } from "wasm-chess";

// Create a new game (starts with standard opening position)
const chess = new WasmChess();

// Or load from FEN string
const chess = new WasmChess(
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
);

// Get legal moves for a piece at position (row, col)
const moves = chess.get_moves(1, 4);

// Make a move
chess.move_piece(moves[0]);

// Get current position as FEN
const currentFen = chess.to_fen();

// Accessing board state
const is_enemy_square = chess.is_enemy_square(0, 0); // boolean
const is_own_square = chess.is_own_square(0, 0); // boolean
const is_square_empty = chess.is_square_empty(0, 0); // boolean
const board_value = chess.access_square(0, 0); // BoardValue

// Check game result
const result = chess.get_game_result();
if (result === "WhiteCheckmate") {
  console.log("White wins!");
}

// Get current game state
const state = chess.get_state();

// History navigation
chess.undo(); // Go back one move
chess.redo(); // Go forward one move
chess.goto_position(0); // Jump to start

// History queries
const canUndo = chess.can_undo(); // boolean
const canRedo = chess.can_redo(); // boolean
const currentPos = chess.get_current_position(); // number
const historyLen = chess.get_history_length(); // number
const history = chess.get_history(); // string[] (array of FEN strings)
```

```typescript
import {
  parse_fen,
  stringify_fen,
  square_to_chess_notation,
  square_from_chess_notation,
} from "wasm-chess";

const parsed_game = parse_fen(
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
);
const fen_string = stringify_fen(parsed_game);

const square_to_chess_notation = square_to_chess_notation(0, 0); // "a1"
const square_from_chess_notation = square_from_chess_notation("a1"); // { row: 0, col: 0 }
```

## Recommendation

To catch rust errors, we recommend to wrap methods in try/catch blocks.

```js
try {
  parse_fen("invalid_fen");
} catch (e) {
  console.log(e);
}
```

## 🎮 API Reference

### Constructor

```typescript
new WasmChess(fen?: string)
```

- `fen` (optional): FEN string for initial position. Defaults to standard starting position.

### Exports

#### `new WasmChess(fen?: string)`

Create a new game instance.

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

#### `load_new_fen(): void`

Load a new game from fen notation string

#### `get_state(): ParsedFen`

Get the current game state as a parsed FEN object.

#### `get_game_result(): GameResult | null`

Get the game result or `null` if game is ongoing.

#### `access_square(row: number, col: number): BoardValue`

Access a square on the board.

#### `is_enemy_square(row: number, col: number): boolean`

Check if a square is occupied by an enemy piece.

#### `is_own_square(row: number, col: number): boolean`

Check if a square is occupied by a piece of the current player.

#### `is_square_empty(row: number, col: number): boolean`

Check if a square is empty.

### History Management

#### `get_history(): string[]`

Get the complete move history as an array of FEN strings.

#### `get_current_position(): number`

Get the current position index in the history (0-based).

#### `get_history_length(): number`

Get the total number of positions in the history.

#### `can_undo(): boolean`

Check if undo is available (returns `true` if not at the start).

#### `can_redo(): boolean`

Check if redo is available (returns `true` if not at the latest position).

#### `undo(): void`

Go back one position in the history. Returns `true` if successful, `false` if already at the start.

#### `redo(): void`

Go forward one position in the history. Returns `true` if successful, `false` if already at the end.

#### `goto_position(index: number): void`

Jump to a specific position in the history. Throws an error if the index is out of bounds.

**Note:** When you make a move after undoing, all future positions are cleared (branching behavior).

### Additional methods

#### `parse_fen(fen: string): ParsedFen`

Parse a FEN string into a game object.

#### `stringify_fen(game: ParsedFen): string`

Convert a game object back to a FEN string.

#### `square_to_chess_notation(row: number, col: number): string | null`

Convert a square to chess notation (e.g., "e4", "a1").

#### `square_from_chess_notation(notation: string): Square | null`

Convert a chess notation to a square (e.g., "e4", "a1").

### Types

```typescript
export type BoardValue = PieceType | null;

export type Board = BoardValue[][];

export type ParsedFen = {
  board: Board;
  state: ParsedFenState;
};

export type ParsedFenState = {
  en_passant_square: Square | null;
  on_turn: Player;
  castle_white_short: boolean;
  castle_white_long: boolean;
  castle_black_short: boolean;
  castle_black_long: boolean;
  half_moves: number;
  full_moves: number;
};

export type Square = {
  row: number;
  col: number;
};

export type Move = {
  from_col_idx: number;
  from_row_idx: number;
  to_col_idx: number;
  to_row_idx: number;
  is_passant: boolean;
  is_castle: boolean;
  piece: PieceType;
};

export type Moves = Move[];

export type GameResult =
  | "WhiteCheckmate"
  | "BlackCheckmate"
  | "Stalemate"
  | "InsufficientMaterial"
  | "FiftyMoveRule"
  | "ThreefoldRepetition"
  | "WhiteResignation"
  | "BlackResignation"
  | null;

export type Player = "White" | "Black";

export type PieceType =
  | "WhitePawn"
  | "WhiteRook"
  | "WhiteBishop"
  | "WhiteKnight"
  | "WhiteQueen"
  | "WhiteKing"
  | "BlackPawn"
  | "BlackRook"
  | "BlackBishop"
  | "BlackKnight"
  | "BlackQueen"
  | "BlackKing";
```

## 🎨 Board Coordinate System

The engine uses a 0-indexed coordinate system:

- **Rows**: 0 (rank 1) to 7 (rank 8)
- **Columns**: 0 (file A) to 7 (file H)

Example: Square "e4" = `{ row: 3, col: 4 }`

## 📄 License

MIT © [Daniel Bilek](https://github.com/dvenomb98)

## 🔗 Links

- [GitHub Repository](https://github.com/dvenomb98/tiny-chess)
- [Issues & Bug Reports](https://github.com/dvenomb98/tiny-chess/issues)
- [npm Package](https://www.npmjs.com/package/wasm-chess)

## 🔧 Build Information

## 🔧 Build Information

This package is built using:

```bash
wasm-pack build --target bundler --release
```

The bundler target generates ES modules that are optimized for modern JavaScript bundlers and handle WASM loading automatically.
