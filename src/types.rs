pub const VALID_PIECES: &'static str = "pnbrqkPNBRQK";

pub const WHITE_KING: char = 'K';
pub const WHITE_QUEEN: char = 'Q';
pub const WHITE_ROOK: char = 'R';
pub const WHITE_BISHOP: char = 'B';
pub const WHITE_KNIGHT: char = 'N';
pub const WHITE_PAWN: char = 'P';

pub const BLACK_KING: char = 'k';
pub const BLACK_QUEEN: char = 'q';
pub const BLACK_ROOK: char = 'r';
pub const BLACK_BISHOP: char = 'b';
pub const BLACK_KNIGHT: char = 'n';
pub const BLACK_PAWN: char = 'p';

pub const WHITE_PLAYER: char = 'w';
pub const BLACK_PLAYER: char = 'b';


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents the kind of piece on the board
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents the player on the board
pub enum Player {
    BLACK,
    WHITE,
}


/// Represents the kind of piece on the board
pub type PieceChar = char;

/// Represents the value of a square on the board
pub type BoardValue = Option<PieceChar>;

/// Represents the board
/// 2D array of 8x8 squares
pub type Board = [[BoardValue; 8]; 8];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents a square on the board (file, rank)
/// Uses 0-based indexing: files a-h = 0-7, ranks 1-8 = 0-7
/// {col, row}
pub struct Square(pub u8, pub u8);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents the parsed FEN state without board
pub struct ParsedFenState {
    pub en_passant_square: Option<Square>,
    pub on_turn: Player,
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
