pub mod err;
pub mod fen;
pub mod types;
pub mod utils;

pub struct Chess {}

impl Chess {
    pub fn parse_fen(fen: &str) -> err::ChessResult<types::ParsedFen> {
        return fen::parse(fen);
    }

    pub fn stringify(parsed_fen: &types::ParsedFen) -> err::ChessResult<String> {
        return fen::stringify(parsed_fen);
    }
}
