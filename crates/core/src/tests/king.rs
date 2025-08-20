#[cfg(test)]
mod test_king_moves {
    use crate::{Chess, moves, square};



    #[test]
    pub fn test_initial_position() {
        let game =
            Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(0, 4), game);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    pub fn test_directions() {
        let game = Chess::parse_fen("rnb1kbnr/pppqpppp/8/3p2K1/2PPP3/8/PP3PPP/RNBQ1BNR w kq - 7 9")
            .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(3, 6), game);

        assert_eq!(moves.len(), 8);
    }

    #[test]
    pub fn test_short_castle() {
        let game =
            Chess::parse_fen("rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(7, 4), game);

        assert_eq!(moves.len(), 3);
        assert_eq!(moves[2].is_castle, true);
        assert_eq!(moves[2].to_col_idx, 6);
    }

    #[test]
    pub fn test_short_castle_unavailable() {
        let game =
            Chess::parse_fen("rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w kq - 4 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(7, 4), game);

        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].is_castle, false);
        assert_eq!(moves[1].is_castle, false);
    }

    #[test]
    pub fn test_double_castle() {
        let game =
            Chess::parse_fen("r3k2r/p1pp1ppp/bpnq1n2/2b1p3/4P3/N1P2N2/PPQPBPPP/R1B1K2R b kq - 5 9")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(0, 4), game);

        assert_eq!(moves.len(), 5);
        assert_eq!(moves[3].is_castle, true);
        assert_eq!(moves[4].to_col_idx, 2);
        assert_eq!(moves[4].is_castle, true);
    }
}
