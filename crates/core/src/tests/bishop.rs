
#[cfg(test)]
mod test_bishop_moves {
    use crate::{Chess, moves, square};

    #[test]
    pub fn test_forward() {
        let game = Chess::parse_fen("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(7, 2), game);
        let moves_black = moves::get_pseudo_moves(square::Square::new(0, 2), game);

        assert_eq!(moves.len(), 5);
        assert_eq!(moves_black.len(), 5);
        // white last
        assert_eq!(moves[4].to_col_idx, 7);
        assert_eq!(moves[4].to_row_idx, 2);
        // black last
        assert_eq!(moves_black[4].to_col_idx, 7);
        assert_eq!(moves_black[4].to_row_idx, 5);
    }

    #[test]
    pub fn test_directions() {
        let game =
            Chess::parse_fen("rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 1")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(4, 5), game);

        assert_eq!(moves.len(), 9);
    }
}
