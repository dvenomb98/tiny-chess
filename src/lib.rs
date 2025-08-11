pub mod err;
pub mod fen;
pub mod moves;
pub mod pieces;
pub mod player;
pub mod square;
pub mod types;

pub struct Chess {}

impl Chess {
    pub fn parse_fen(fen: &str) -> types::ChessResult<types::ParsedFen> {
        return fen::parse(fen);
    }

    pub fn stringify(parsed_fen: &types::ParsedFen) -> types::ChessResult<String> {
        return fen::stringify(parsed_fen);
    }

    pub fn get_moves(square: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
        moves::get_moves(square, game)
    }

    pub fn move_piece() {
        todo!("Move from A to B square. Should return new ParsedFen.")
    }

    pub fn validate_move() {
        todo!("Validate if Move From A to B square is valid. Return boolean")
    }


}
