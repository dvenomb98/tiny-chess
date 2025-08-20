use crate::pieces;
use crate::types;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Represents a square on the board
/// Uses 0-based indexing: ranks 1-8 = 7-0 (inverted), files a-h = 0-7
/// Row 0 = rank 8 (top), Row 7 = rank 1 (bottom)
pub struct Square {
    pub row: usize,
    pub col: usize,
}

impl Square {
    /// Create a new square at the specified row and column
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Create a square from chess notation (e.g., "e4", "a1")
    pub fn from_chess_notation(notation: &str) -> Option<Self> {
        let chars: Vec<char> = notation.chars().collect();
        if chars.len() != 2 {
            return None;
        }

        let file = chars[0];
        let rank = chars[1];

        let col = Self::file_char_to_index(file)?;
        let row = Self::rank_char_to_index(rank)?;

        Some(Self::new(row, col))
    }

    /// Convert square to chess notation (e.g., "e4", "a1")
    pub fn to_chess_notation(&self) -> Option<String> {
        let file = Self::index_to_file_char(self.col)?;
        let rank = Self::rank_index_to_char(self.row)?;
        Some(format!("{}{}", file, rank))
    }

    /// Get piece from this square on the given board
    pub fn get_piece(&self, board: &types::Board) -> Option<pieces::PieceType> {
        board[self.row][self.col]
    }

    /// Parse rank character ('1'-'8') to 0-based index (0-7)
    /// Note: rank '1' maps to row 7 (bottom), rank '8' maps to row 0 (top)
    fn rank_char_to_index(rank: char) -> Option<usize> {
        if rank >= '1' && rank <= '8' {
            Some(7 - ((rank as usize) - ('1' as usize)))
        } else {
            None
        }
    }

    /// Parse file character ('a'-'h') to 0-based index (0-7)
    fn file_char_to_index(file: char) -> Option<usize> {
        if file >= 'a' && file <= 'h' {
            let index = (file as usize) - ('a' as usize);
            Some(index)
        } else {
            None
        }
    }

    /// Convert 0-based index (0-7) to file character ('a'-'h')
    fn index_to_file_char(col: usize) -> Option<char> {
        if col <= 7 {
            Some((col as u8 + b'a') as char)
        } else {
            None
        }
    }

    /// Parse index (0-7) to rank character ("1"-"8")
    /// Note: row 0 (top) maps to rank '8', row 7 (bottom) maps to rank '1'
    fn rank_index_to_char(rank: usize) -> Option<char> {
        if rank <= 7 {
            Some(((7 - rank) as u8 + b'1') as char)
        } else {
            None
        }
    }
}
