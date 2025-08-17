#[cfg(test)]
mod test_pawn_moves {
    use crate::{Chess, moves, pieces, player, square, types};



    #[test]
    fn test_white_en_passant() {
        let game =
            Chess::parse_fen("rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(3, 3), game);

        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].to_col_idx, 3); // Forward move
        assert_eq!(moves[0].to_row_idx, 2); // Forward move
        assert_eq!(moves[0].is_passant, false);
        assert_eq!(moves[0].piece, pieces::PieceType::WhitePawn);
        assert_eq!(moves[1].to_col_idx, 4); // En passant capture
        assert_eq!(moves[1].to_row_idx, 2); // En passant capture
        assert_eq!(moves[1].is_passant, true);
        assert_eq!(moves[1].piece, pieces::PieceType::WhitePawn);
    }

    #[test]
    fn test_black_en_passant_with_block() {
        let game =
            Chess::parse_fen("rnbqkbnr/pp1p1ppp/2p5/8/3PpP2/4P3/PPP3PP/RNBQKBNR b KQkq f3 0 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(4, 4), game);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_col_idx, 5);
        assert_eq!(moves[0].to_row_idx, 5);
        assert_eq!(moves[0].piece, pieces::PieceType::BlackPawn);
    }

    #[test]
    fn test_black_starting_position() {
        let game =
            Chess::parse_fen("rnbqkbnr/pp1p1ppp/2p5/8/3PpP2/4P3/PPP3PP/RNBQKBNR b KQkq f3 0 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(square::Square::new(1, 0), game);

        assert_eq!(moves.len(), 2);
        assert_eq!(moves[1].to_col_idx, 0);
        assert_eq!(moves[1].to_row_idx, 3);
        assert_eq!(moves[1].is_passant, false);
        assert_eq!(moves[0].to_col_idx, 0);
        assert_eq!(moves[0].to_row_idx, 2);
        assert_eq!(moves[0].is_castle, false);
    
       
    }

    #[test]
    fn test_pawn_on_last_rank() {
        let game = types::ParsedFen {
            board: [
                [
                    None,
                    None,
                    None,
                    Some(pieces::PieceType::BlackKing),
                    None,
                    None,
                    None,
                    Some(pieces::PieceType::BlackPawn),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(pieces::PieceType::WhiteKing),
                    None,
                    None,
                    None,
                    Some(pieces::PieceType::BlackPawn),
                ],
            ],
            state: types::ParsedFenState {
                en_passant_square: None,
                on_turn: player::Player::White,
                castle_white_short: false,
                castle_white_long: false,
                castle_black_short: false,
                castle_black_long: false,
                half_moves: 0,
                full_moves: 1,
            },
        };

        // This just check if code is not panicking

        let moves = moves::get_pseudo_moves(square::Square::new(0, 7), game);
        let moves2 = moves::get_pseudo_moves(square::Square::new(7, 7), game);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves2.len(), 0);
    }
}
