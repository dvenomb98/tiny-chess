use crate::pieces;
use crate::player;
use crate::square;
use crate::err;

/// Represents the value of a square on the board
pub type BoardValue = Option<pieces::PieceType>;

/// Represents the board
/// 2D array of 8x8 squares
pub type Board = [[BoardValue; 8]; 8];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents the parsed FEN state without board
pub struct ParsedFenState {
    pub en_passant_square: Option<square::Square>,
    pub on_turn: player::Player,
    pub castle_white_short: bool,
    pub castle_white_long: bool,
    pub castle_black_short: bool,
    pub castle_black_long: bool,
    pub half_moves: u32,
    pub full_moves: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents the parsed FEN position
pub struct ParsedFen {
    pub board: Board,
    pub state: ParsedFenState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub from_col_idx: usize,
    pub from_row_idx: usize,
    pub to_col_idx: usize,
    pub to_row_idx: usize,
    pub is_passant: bool,
    pub is_castle: bool,
    pub piece: pieces::PieceType,
}

pub type ChessResult<T> = std::result::Result<T, err::ChessError>;
