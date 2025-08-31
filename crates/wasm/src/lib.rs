use serde::Serialize;
use tiny_chess_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type BoardValue = PieceType | null;

export type Board = BoardValue[][];

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

export type GameResult = "WhiteCheckmate" | "BlackCheckmate" | "Stalemate" | "InsufficientMaterial" | "FiftyMoveRule" | "ThreefoldRepetition" | "WhiteResignation" | "BlackResignation" | null;

export type Player = "White" | "Black";

export type PieceType = 
  | "WhitePawn" | "WhiteRook" | "WhiteBishop" | "WhiteKnight" | "WhiteQueen" | "WhiteKing"
  | "BlackPawn" | "BlackRook" | "BlackBishop" | "BlackKnight" | "BlackQueen" | "BlackKing";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParsedFen")]
    pub type ParsedFenJs;

    #[wasm_bindgen(typescript_type = "Moves")]
    pub type MovesJs;

    #[wasm_bindgen(typescript_type = "Move")]
    pub type MoveJs;

    #[wasm_bindgen(typescript_type = "GameResult")]
    pub type GameResultJs;

    #[wasm_bindgen(typescript_type = "Square")]
    pub type SquareJs;

    #[wasm_bindgen(typescript_type = "BoardValue")]
    pub type BoardValueJs;
}

//
//
// # Stateless functions
//
//

#[wasm_bindgen]
pub fn parse_fen(fen: &str) -> Result<ParsedFenJs, JsValue> {
    let result = Chess::parse_fen(fen).map_err(|e| format_error(e))?;
    Ok(result.serialize(&CHESS_SERIALIZER)?.into())
}

#[wasm_bindgen]
pub fn stringify_fen(game: ParsedFenJs) -> Result<String, JsValue> {
    let parsed_game = parse_game_js(game)?;
    Chess::stringify(&parsed_game).map_err(|e| format_error(e))
}

#[wasm_bindgen]
pub fn square_to_chess_notation(row: usize, col: usize) -> Option<String> {
    Square::new(row, col).to_chess_notation()
}

#[wasm_bindgen]
pub fn square_from_chess_notation(notation: &str) -> Option<SquareJs> {
    Square::new_from_chess_notation(notation)
        .map(|square| serde_wasm_bindgen::to_value(&square).unwrap().into())
}

//
//
// # Main struct
//
//

#[wasm_bindgen]
pub struct WasmChess {
    game: ParsedFen,
}

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: Option<String>) -> Result<WasmChess, JsValue> {
        let game = if let Some(fen_str) = fen {
            Chess::parse_fen(&fen_str).map_err(|e| format_error(e))?
        } else {
            Chess::parse_fen(INITIAL_FEN).map_err(|e| format_error(e))?
        };

        Ok(WasmChess { game })
    }

    #[wasm_bindgen]
    pub fn load_new_fen(&mut self, fen: String) -> Result<(), JsValue> {
        let result = Chess::parse_fen(&fen).map_err(|e| format_error(e))?;
        self.game = result;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn to_fen(&self) -> Result<String, JsValue> {
        Chess::stringify(&self.game).map_err(|e| format_error(e))
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> Result<ParsedFenJs, JsValue> {
        Ok(self.game.serialize(&CHESS_SERIALIZER)?.into())
    }

    #[wasm_bindgen]
    pub fn get_moves(&self, row: usize, col: usize) -> Result<MovesJs, JsValue> {
        let moves = Chess::get_moves(Square::new(row, col), self.game);
        Ok(serde_wasm_bindgen::to_value(&moves)?.into())
    }

    #[wasm_bindgen]
    pub fn get_pseudo_moves(&self, row: usize, col: usize) -> Result<MovesJs, JsValue> {
        let moves = Chess::get_pseudo_moves(Square::new(row, col), self.game);
        Ok(serde_wasm_bindgen::to_value(&moves)?.into())
    }

    #[wasm_bindgen]
    pub fn move_piece(&mut self, req_move: MoveJs) -> Result<(), JsValue> {
        let parsed_move = parse_move_js(req_move)?;
        let result = Chess::move_piece(parsed_move, self.game).map_err(|e| format_error(e))?;
        self.game = result;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn validate_move(&self, req_move: MoveJs) -> Result<bool, JsValue> {
        let parsed_move = parse_move_js(req_move)?;
        let result = Chess::validate_move(parsed_move, self.game).map_err(|e| format_error(e))?;
        Ok(result)
    }

    #[wasm_bindgen]
    pub fn get_game_result(&self) -> Result<GameResultJs, JsValue> {
        let result = Chess::get_game_result(self.game).map_err(|e| format_error(e))?;

        Ok(result.serialize(&CHESS_SERIALIZER)?.into())
    }

    #[wasm_bindgen]
    pub fn access_square(&self, row: usize, col: usize) -> Result<BoardValueJs, JsValue> {
        let result = self
            .game
            .safe_access_square(row, col)
            .map_err(|e| format_error(e))?;
        Ok(result.serialize(&CHESS_SERIALIZER)?.into())
    }

    #[wasm_bindgen]
    pub fn is_enemy_square(&self, row: usize, col: usize) -> bool {
        self.game.is_enemy_square(row, col, self.game.state.on_turn)
    }

    #[wasm_bindgen]
    pub fn is_own_square(&self, row: usize, col: usize) -> bool {
        self.game.is_own_square(row, col, self.game.state.on_turn)
    }

    #[wasm_bindgen]
    pub fn is_square_empty(&self, row: usize, col: usize) -> bool {
        self.game.is_square_empty(row, col)
    }
}

//
//
// #  Private functions
//
//

static CHESS_SERIALIZER: serde_wasm_bindgen::Serializer =
    serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);

fn parse_game_js(game: ParsedFenJs) -> Result<ParsedFen, JsValue> {
    serde_wasm_bindgen::from_value::<ParsedFen>(game.into()).map_err(|e| format_error(e))
}

fn parse_move_js(req_move: MoveJs) -> Result<Move, JsValue> {
    serde_wasm_bindgen::from_value::<Move>(req_move.into()).map_err(|e| format_error(e))
}

fn format_error<T: std::fmt::Debug>(error: T) -> JsValue {
    JsValue::from_str(&format!("{:#?}", error))
}
