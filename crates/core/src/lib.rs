mod err;
mod fen;
mod moves;
mod pieces;
mod player;
mod result;
mod square;
mod state;
mod types;
mod validation;

#[cfg(test)]
mod tests;

// Public API
pub use err::ChessError;
pub use fen::INITIAL_FEN;
pub use pieces::{PieceKind, PieceType};
pub use player::Player;
pub use result::GameResult;
pub use square::Square;
pub use types::{ChessResult, Move, ParsedFen, ParsedFenState};

pub struct Chess {}

impl Chess {
    /// Parses a FEN (Forsyth-Edwards Notation) string into a chess game state.
    /// Returns a parsed representation that can be used for game operations.
    pub fn parse_fen(fen: &str) -> types::ChessResult<types::ParsedFen> {
        fen::parse(fen)
    }

    /// Converts a parsed chess game state back into a FEN string.
    /// Useful for serializing game positions for storage or transmission.
    pub fn stringify(parsed_fen: &types::ParsedFen) -> types::ChessResult<String> {
        fen::stringify(parsed_fen)
    }

    /// Gets all legal moves for a piece at the given square.
    pub fn get_moves(square: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
        let pseudo_moves = moves::get_pseudo_moves(square, game);

        validation::validate_moves(pseudo_moves, game)
    }

    /// Gets all pseudo-legal moves for a piece at the given square.
    /// These moves may leave the king in check and require validation.
    pub fn get_pseudo_moves(square: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
        moves::get_pseudo_moves(square, game)
    }

    /// Executes a move and returns the resulting game state.
    ///
    /// 1. Validates the move against the game state.
    /// 2. Validates the move is legal before applying it to the board.
    ///
    pub fn move_piece(
        req_move: types::Move,
        game: types::ParsedFen,
    ) -> types::ChessResult<types::ParsedFen> {
        validation::validate_move_against_state(req_move, game)?;
        Ok(validation::validate_move(req_move, game)?)
    }

    /// Validates the move against the game state and chess rules.
    ///
    /// 1. Validates the move against the game state.
    /// 2. Validates the move is legal before applying it to the board.
    ///
    /// Returns the resulting game state if the move is valid, or an error if it's not.
    ///
    pub fn validate_move(
        req_move: types::Move,
        game: types::ParsedFen,
    ) -> types::ChessResult<bool> {
        validation::validate_move_against_state(req_move, game)?;
        validation::validate_move(req_move, game)?;

        Ok(true)
    }

    /// Gets the game result for a given game state.
    /// Returns the game result if the game is over, or None if the game is not over.
    pub fn get_game_result(game: types::ParsedFen) -> types::ChessResult<Option<GameResult>> {
        result::get_game_result(game)
    }
}
