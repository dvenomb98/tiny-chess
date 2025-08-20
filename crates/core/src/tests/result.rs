#[cfg(test)]
mod test_result {
    use crate::{Chess, result};

    #[test]
    pub fn test_start_position() {
        let game =
            Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    pub fn test_white_checkmate() {
        let game = Chess::parse_fen("6B1/8/7p/7P/8/7K/Q7/k7 b - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), Some(result::GameResult::WhiteCheckmate));
    }

    #[test]
    pub fn test_black_checkmate() {
        let game = Chess::parse_fen("6B1/8/7p/6qP/8/7K/8/k6r w - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), Some(result::GameResult::BlackCheckmate));
    }

    #[test]
    pub fn test_stalemate() {
        let game = Chess::parse_fen("6B1/8/7p/7P/8/8/8/k1K5 b - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), Some(result::GameResult::Stalemate));
    }

    #[test]
    pub fn test_insufficient_material_case_1() {
        let game = Chess::parse_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            Some(result::GameResult::InsufficientMaterial)
        );
    }

    #[test]
    pub fn test_insufficient_material_case_2() {
        let game = Chess::parse_fen("4k3/8/8/8/8/8/8/2B1K3 w - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            Some(result::GameResult::InsufficientMaterial)
        );
    }

    #[test]
    pub fn test_insufficient_material_case_3() {
        let game = Chess::parse_fen("4k1n1/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            Some(result::GameResult::InsufficientMaterial)
        );
    }

    #[test]
    pub fn test_insufficient_material_case_4() {
        let game = Chess::parse_fen("4k1b1/8/8/8/8/8/2B5/4K3 w - - 0 1").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            Some(result::GameResult::InsufficientMaterial)
        );
    }

    #[test]
    pub fn test_fifty_move_rule() {
        let game = Chess::parse_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 w - - 100 50").unwrap();
        let result = result::get_game_result(game);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), Some(result::GameResult::FiftyMoveRule));
    }
}
