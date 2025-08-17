#[cfg(test)]
mod test_rook_moves {
    use crate::{Chess, moves, square};



    #[test]
    pub fn test_forward() {
        let game =
            Chess::parse_fen("rnbqkbnr/pppppppp/8/PP6/8/8/2PPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(7, 0), game);

        assert_eq!(moves.len(), 3);
    }

    #[test]
    pub fn test_blocked_by_enemy() {
        let game =
            Chess::parse_fen("1nbqkbnr/pppppppp/2P5/PPrP4/2P5/8/2PPPPPP/RNBQKBNR w KQk - 0 1")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(3, 2), game);

        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn test_directions() {
        let game =
            Chess::parse_fen("1nbqkbnr/pppppppp/8/8/3R4/8/2PPPPPP/1NBQKBNR w Kk - 0 1").unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(4, 3), game);

        assert_eq!(moves.len(), 11);
    }
}
