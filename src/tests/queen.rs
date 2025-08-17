#[cfg(test)]
mod test_queen_moves {
    use crate::{Chess, moves, square};



    #[test]
    pub fn test_directions() {
        let game = Chess::parse_fen("rn1qkb1r/ppp1pppp/8/2QpN3/3PbBn1/8/PPP1PPPP/RN2KB1R w KQkq - 0 1")
            .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(3, 2), game);

        assert_eq!(moves.len(), 13);
    }
}