use serde::{Serialize, Deserialize};
use crate::player;

const WHITE_KING: char = 'K';
const WHITE_QUEEN: char = 'Q';
const WHITE_ROOK: char = 'R';
const WHITE_BISHOP: char = 'B';
const WHITE_KNIGHT: char = 'N';
const WHITE_PAWN: char = 'P';

const BLACK_KING: char = 'k';
const BLACK_QUEEN: char = 'q';
const BLACK_ROOK: char = 'r';
const BLACK_BISHOP: char = 'b';
const BLACK_KNIGHT: char = 'n';
const BLACK_PAWN: char = 'p';

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    WhitePawn,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackQueen,
    BlackKing,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceKind {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::WhitePawn => WHITE_PAWN,
            PieceType::WhiteRook => WHITE_ROOK,
            PieceType::WhiteBishop => WHITE_BISHOP,
            PieceType::WhiteKnight => WHITE_KNIGHT,
            PieceType::WhiteQueen => WHITE_QUEEN,
            PieceType::WhiteKing => WHITE_KING,
            PieceType::BlackPawn => BLACK_PAWN,
            PieceType::BlackRook => BLACK_ROOK,
            PieceType::BlackBishop => BLACK_BISHOP,
            PieceType::BlackKnight => BLACK_KNIGHT,
            PieceType::BlackQueen => BLACK_QUEEN,
            PieceType::BlackKing => BLACK_KING,
        }
    }

    pub fn from_char(c: char) -> Option<PieceType> {
        match c {
            WHITE_PAWN => Some(PieceType::WhitePawn),
            WHITE_ROOK => Some(PieceType::WhiteRook),
            WHITE_BISHOP => Some(PieceType::WhiteBishop),
            WHITE_KNIGHT => Some(PieceType::WhiteKnight),
            WHITE_QUEEN => Some(PieceType::WhiteQueen),
            WHITE_KING => Some(PieceType::WhiteKing),
            BLACK_PAWN => Some(PieceType::BlackPawn),
            BLACK_ROOK => Some(PieceType::BlackRook),
            BLACK_BISHOP => Some(PieceType::BlackBishop),
            BLACK_KNIGHT => Some(PieceType::BlackKnight),
            BLACK_QUEEN => Some(PieceType::BlackQueen),
            BLACK_KING => Some(PieceType::BlackKing),
            _ => None,
        }
    }

    pub fn is_valid_piece_char(c: char) -> bool {
        Self::from_char(c).is_some()
    }
    
    pub fn color(&self) -> player::Player {
        match self {
            PieceType::WhitePawn
            | PieceType::WhiteBishop
            | PieceType::WhiteKnight
            | PieceType::WhiteKing
            | PieceType::WhiteQueen
            | PieceType::WhiteRook => player::Player::White,
            _ => player::Player::Black,
        }
    }

    pub fn piece_kind(&self) -> PieceKind {
        match self {
            PieceType::BlackPawn | PieceType::WhitePawn => PieceKind::Pawn,
            PieceType::BlackBishop | PieceType::WhiteBishop => PieceKind::Bishop,
            PieceType::BlackKing | PieceType::WhiteKing => PieceKind::King,
            PieceType::BlackKnight | PieceType::WhiteKnight => PieceKind::Knight,
            PieceType::BlackQueen | PieceType::WhiteQueen => PieceKind::Queen,
            PieceType::BlackRook | PieceType::WhiteRook => PieceKind::Rook,
        }
    }

    pub fn is_white(&self) -> bool {
        match self.color() {
            player::Player::White => true,
            player::Player::Black => false,
        }
    }
}