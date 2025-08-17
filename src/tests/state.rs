#[cfg(test)]
mod test_state {
    use crate::{Chess, moves, pieces, Square, state};


    #[test]
    pub fn test_random_sequence() {
        let game =
            Chess::parse_fen("rnbqkb1r/pp3ppp/2p1pn2/3p4/3P1B2/2N1P3/PPP2PPP/R2QKBNR w KQkq - 2 5")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(5, 2), game);

        let excepted_move = moves
            .iter()
            .find(|m| m.to_col_idx == 1 && m.to_row_idx == 3);

        assert_eq!(excepted_move.is_some(), true);

        let req_move = excepted_move.unwrap();

        let new_board = state::get_next_board(*req_move, game).unwrap();

        assert_eq!(new_board[5][2], None);
        assert_eq!(new_board[3][1], Some(pieces::PieceType::WhiteKnight));
    }

    #[test]
    pub fn test_castling() {
        let game =
            Chess::parse_fen("rnbqkb1r/pp3ppp/4pn2/3p4/1p1P1B2/3QP3/PPP2PPP/R3KBNR w KQkq - 0 7")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(7, 4), game);

        let excepted_move = moves
            .iter()
            .find(|m| m.to_col_idx == 2 && m.to_row_idx == 7);

        assert_eq!(excepted_move.is_some(), true);

        let new_board = state::get_next_board(*excepted_move.unwrap(), game).unwrap();

        assert_eq!(new_board[7][4], None);
        assert_eq!(new_board[7][2], Some(pieces::PieceType::WhiteKing));
        assert_eq!(new_board[7][3], Some(pieces::PieceType::WhiteRook));
    }

    #[test]
    pub fn test_en_passant() {
        let game =
            Chess::parse_fen("rnbqkb1r/1p3ppp/p3pn2/3p4/Pp1P1B2/3QP3/1PP2PPP/2KR1BNR b kq a3 0 8")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(4, 1), game);

        let excepted_move = moves
            .iter()
            .find(|m| m.to_col_idx == 0 && m.to_row_idx == 5);

        assert_eq!(excepted_move.is_some(), true);
        assert_eq!(excepted_move.unwrap().is_passant, true);

        let new_board = state::get_next_board(*excepted_move.unwrap(), game).unwrap();

        assert_eq!(new_board[4][1], None);
        assert_eq!(new_board[5][0], Some(pieces::PieceType::BlackPawn));
        assert_eq!(new_board[4][0], None);
    }

    #[test]
    pub fn test_promotion() {
        let game =
            Chess::parse_fen("rnbqkb1r/1p3ppp/p3pn2/P2p4/3P1B2/3QP3/1PpK1PPP/3R1BNR b kq - 1 10")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(6, 2), game);

        let excepted_move = moves
            .iter()
            .find(|m| m.to_col_idx == 2 && m.to_row_idx == 7);

        assert_eq!(excepted_move.is_some(), true);

        let new_board = state::get_next_board(*excepted_move.unwrap(), game).unwrap();

        // TODO: Need to be reworked after state.rs change promotion
        assert_eq!(new_board[7][2], Some(pieces::PieceType::BlackQueen));
    }
}


#[cfg(test)]
mod test_get_next_state {
    use crate::{Chess, moves, state, player, Square};

    #[test]
    pub fn test_king_break_castle() {
        let game =
            Chess::parse_fen("r1bqk1nr/pppp1ppp/n7/4p3/1bB1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(7, 4), game);
        let next_state = state::get_next_state(moves[0], game);

        assert_eq!(next_state.castle_white_long, false);
        assert_eq!(next_state.castle_white_short, false);
        assert_eq!(next_state.castle_black_long, true);
        assert_eq!(next_state.castle_black_short, true);
        assert_eq!(next_state.half_moves, 5);
        assert_eq!(next_state.full_moves, 4);
        assert_eq!(next_state.on_turn, player::Player::Black);
    }

    #[test]
    pub fn test_pawn_jump() {
        let game =
            Chess::parse_fen("r1bqk1nr/pppp1ppp/n7/4p3/1bB1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(6, 7), game);
        let next_state = state::get_next_state(moves[1], game);

        assert_eq!(
            next_state.en_passant_square,
            Some(Square { row: 5, col: 7 })
        );

        assert_eq!(next_state.half_moves, 0);
        assert_eq!(next_state.full_moves, 4);
        assert_eq!(next_state.castle_white_long, true);
        assert_eq!(next_state.castle_white_short, true);
        assert_eq!(next_state.castle_black_long, true);
        assert_eq!(next_state.castle_black_short, true);
        assert_eq!(next_state.on_turn, player::Player::Black);
    }

    #[test]
    pub fn test_rook_break_castle() {
        let game =
            Chess::parse_fen("r1bqk1nr/pppp1ppp/n7/4p3/1bB1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(7, 7), game);
        let next_state = state::get_next_state(moves[1], game);

        assert_eq!(next_state.castle_white_long, true);
        assert_eq!(next_state.castle_white_short, false);
        assert_eq!(next_state.castle_black_long, true);
        assert_eq!(next_state.castle_black_short, true);
        assert_eq!(next_state.half_moves, 5);
        assert_eq!(next_state.full_moves, 4);
        assert_eq!(next_state.on_turn, player::Player::Black);
    }

    #[test]
    pub fn test_king_castle() {
        let game =
            Chess::parse_fen("r3k1nr/ppp2ppp/n2p4/4p1N1/1bB1P1bq/2N5/PPPPQPPP/R1B2RK1 b kq - 5 7")
                .unwrap();
        let moves = moves::get_pseudo_moves(Square::new(0, 4), game);
        let next_state = state::get_next_state(moves[4], game);

        assert_eq!(next_state.castle_black_short, false);
        assert_eq!(next_state.castle_black_long, false);
        assert_eq!(next_state.castle_white_long, false);
        assert_eq!(next_state.castle_white_short, false);
        assert_eq!(next_state.half_moves, 6);
        assert_eq!(next_state.full_moves, 8);
        assert_eq!(next_state.on_turn, player::Player::White);
    }
}
