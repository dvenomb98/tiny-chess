pub mod types;
pub mod fen;
pub mod err;
pub mod utils;

pub struct Chess {}

impl Chess {

    pub fn parse_fen(fen: &str) -> err::ChessResult<types::ParsedFen> {
        return fen::parse(fen)
    }
}
