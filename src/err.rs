use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessError {
    InvalidFen(String)
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::InvalidFen(fen) => write!(f, "Invalid FEN: {}", fen),
        }
    }
}

impl std::error::Error for ChessError {}

pub type ChessResult<T> = std::result::Result<T, ChessError>;