use crate::types;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessError {
    InvalidFen(String),
    InvalidParsedFen(types::ParsedFen),
    InvalidSquare(String),
    OutOfBounds(usize, usize),
    InvalidMove(String),
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::InvalidFen(fen) => write!(f, "Invalid FEN: {}", fen),
            ChessError::InvalidParsedFen(parsed_fen) => {
                write!(f, "Invalid parsed FEN: {:?}", parsed_fen)
            }
            ChessError::InvalidSquare(msg) => write!(f, "Invalid square: {}", msg),
            ChessError::OutOfBounds(row, col) => {
                write!(f, "Out of bounds. Row: {}, Col: {}", row, col)
            }
            ChessError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
        }
    }
}

impl std::error::Error for ChessError {}
