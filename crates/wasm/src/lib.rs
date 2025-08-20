use tiny_chess_core::*;
use wasm_bindgen::prelude::*;

// Custom TypeScript definitions
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface ParsedFen {
  board: (PieceType | null)[][];
  state: ParsedFenState;
}

export interface ParsedFenState {
  en_passant_square: Square | null;
  on_turn: Player;
  castle_white_short: boolean;
  castle_white_long: boolean;
  castle_black_short: boolean;
  castle_black_long: boolean;
  half_moves: number;
  full_moves: number;
}

export interface Square {
  row: number;
  col: number;
}

export type Player = "White" | "Black";

export type PieceType = 
  | "WhitePawn" | "WhiteRook" | "WhiteBishop" | "WhiteKnight" | "WhiteQueen" | "WhiteKing"
  | "BlackPawn" | "BlackRook" | "BlackBishop" | "BlackKnight" | "BlackQueen" | "BlackKing";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParsedFen")]
    pub type ParsedFenJs;
}

#[wasm_bindgen]
pub fn parse_fen(fen: &str) -> Result<ParsedFenJs, JsValue> {
    let result = Chess::parse_fen(fen)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    
    Ok(serde_wasm_bindgen::to_value(&result)?.into())
}

#[wasm_bindgen]
pub fn stringify_fen(game: ParsedFenJs) -> Result<String, JsValue> {
    let parsed_game: ParsedFen = serde_wasm_bindgen::from_value(game.into())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Chess::stringify(&parsed_game)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}