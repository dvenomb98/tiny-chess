mod err;
mod fen;
mod moves;
mod pieces;
mod player;
mod square;
mod state;
mod types;
mod validation;

#[cfg(test)]
mod tests;

// Public API
pub use err::ChessError;
pub use pieces::{PieceKind, PieceType};
pub use player::Player;
pub use square::Square;
pub use types::{ChessResult, Move, ParsedFen, ParsedFenState};

pub struct Chess {}

impl Chess {
    pub fn parse_fen(fen: &str) -> types::ChessResult<types::ParsedFen> {
        fen::parse(fen)
    }

    pub fn stringify(parsed_fen: &types::ParsedFen) -> types::ChessResult<String> {
        fen::stringify(parsed_fen)
    }

    pub fn get_moves(square: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
        let pseudo_moves = moves::get_pseudo_moves(square, game);

        validation::validate_moves(pseudo_moves, game)
    }

    pub fn get_pseudo_moves(square: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
        moves::get_pseudo_moves(square, game)
    }

    pub fn move_piece(
        req_move: types::Move,
        game: types::ParsedFen,
    ) -> types::ChessResult<types::ParsedFen> {
        validation::validate_move(req_move, game)?;
        state::get_next(req_move, game)
    }

    pub fn validate_move(
        req_move: types::Move,
        game: types::ParsedFen,
    ) -> types::ChessResult<bool> {
        validation::validate_move(req_move, game)?;

        Ok(true)
    }
}
