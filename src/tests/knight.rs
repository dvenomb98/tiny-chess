#[cfg(test)]
mod test_knight_moves {
    use crate::{Chess, moves, square};



    #[test]
    pub fn test_directions() {
        let game =
            Chess::parse_fen("rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 1")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(7, 6), game);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    pub fn test_close_to_bounds() {
        let game =
            Chess::parse_fen("rn1qkb1r/ppp1pppp/7N/3p1b2/3P1B2/8/PPP1PPPP/RN1QKB1R w KQkq - 0 1")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(2, 7), game);

        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn test_all_directions() {
        let game =
            Chess::parse_fen("r2qkb1r/ppp1pppp/7N/3p1b2/3n1B2/8/PPP1PPPP/RN1QKB1R w KQkq - 0 1")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(4, 3), game);

        assert_eq!(moves.len(), 7)
    }
}
