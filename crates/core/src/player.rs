use serde::{Serialize, Deserialize};

const BLACK_PLAYER: char = 'b';
const WHITE_PLAYER: char = 'w';

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// Represents the player on the board
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn to_char(&self) -> char {
        match &self {
            Player::White => WHITE_PLAYER,
            Player::Black => BLACK_PLAYER,
        }
    }

    pub fn from_char(c: char) -> Option<Player> {
        match c {
            BLACK_PLAYER => Some(Player::Black),
            WHITE_PLAYER => Some(Player::White),
            _ => None,
        }
    }
}