use crate::err;
use crate::pieces;
use crate::player;
use crate::square;
use serde::{Deserialize, Serialize};

pub const BOARD_SIZE: usize = 8;
pub const MAX_SIZE_INDEX: usize = 7;

/// Represents the value of a square on the board
pub type BoardValue = Option<pieces::PieceType>;

/// Represents the board
/// 2D array of 8x8 squares
pub type Board = [[BoardValue; 8]; 8];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// Represents the parsed FEN position
pub struct ParsedFen {
    pub board: Board,
    pub state: ParsedFenState,
}

impl ParsedFen {
    pub fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        return row <= MAX_SIZE_INDEX && col <= MAX_SIZE_INDEX;
    }

    pub fn safe_access_square(&self, row: usize, col: usize) -> ChessResult<BoardValue> {
        if self.is_in_bounds(row, col) {
            Ok(self.board[row][col])
        } else {
            Err(err::ChessError::OutOfBounds(row, col))
        }
    }

    pub fn is_enemy_square(&self, row: usize, col: usize, cur_color: player::Player) -> bool {
        return self
            .safe_access_square(row, col)
            .is_ok_and(|piece| piece.is_some() && piece.unwrap().color() != cur_color);
    }

    pub fn is_own_square(&self, row: usize, col: usize, cur_color: player::Player) -> bool {
        return self
            .safe_access_square(row, col)
            .is_ok_and(|piece| piece.is_some() && piece.unwrap().color() == cur_color);
    }

    pub fn is_square_empty(&self, row: usize, col: usize) -> bool {
        return self
            .safe_access_square(row, col)
            .is_ok_and(|piece| piece.is_none());
    }

    /// Assigns a value to a square on the board
    /// Returns an error if the square is out of bounds
    pub fn assign_square(&mut self, row: usize, col: usize, value: BoardValue) -> ChessResult<()> {
        if self.is_in_bounds(row, col) {
            self.board[row][col] = value;
            Ok(())
        } else {
            Err(err::ChessError::OutOfBounds(row, col))
        }
    }

    /// Returns (can_castle_short, can_castle_long)
    pub fn get_castle_ability(&self, cur_color: player::Player) -> (bool, bool) {
        match cur_color {
            player::Player::White => (self.state.castle_white_short, self.state.castle_white_long),
            player::Player::Black => (self.state.castle_black_short, self.state.castle_black_long),
        }
    }
}
