use std::fmt;
use crate::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessError {
    InvalidFen(String),
    InvalidParsedFen(types::ParsedFen)
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::InvalidFen(fen) => write!(f, "Invalid FEN: {}", fen),
            ChessError::InvalidParsedFen(parsed_fen) => write!(f, "Invalid parsed FEN: {:?}", parsed_fen)
        }
    }
}

impl std::error::Error for ChessError {}

