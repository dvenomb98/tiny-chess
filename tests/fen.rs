use tiny_chess::{Chess, err, pieces, player, square};

mod fen_parsing_tests {
    use super::*;

    #[test]
    fn test_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        assert_eq!(parsed.board[0][0], Some(pieces::PieceType::BlackRook)); // Black rook on a8
        assert_eq!(parsed.board[0][4], Some(pieces::PieceType::BlackKing)); // Black king on e8
        assert_eq!(parsed.board[7][0], Some(pieces::PieceType::WhiteRook)); // White rook on a1
        assert_eq!(parsed.board[7][4], Some(pieces::PieceType::WhiteKing)); // White king on e1

        assert!(matches!(parsed.state.on_turn, player::Player::White));
        assert!(parsed.state.castle_white_short);
        assert!(parsed.state.castle_white_long);
        assert!(parsed.state.castle_black_short);
        assert!(parsed.state.castle_black_long);
        assert_eq!(parsed.state.half_moves, 0);
        assert_eq!(parsed.state.full_moves, 1);
        assert!(parsed.state.en_passant_square.is_none());
    }

    #[test]
    fn test_midgame_position() {
        let fen = "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 1 2";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        assert_eq!(parsed.board[4][4], Some(pieces::PieceType::WhitePawn)); // White pawn on e4
        assert_eq!(parsed.board[2][5], Some(pieces::PieceType::BlackKnight)); // Black knight on f6

        assert!(matches!(parsed.state.on_turn, player::Player::Black));
        assert_eq!(parsed.state.half_moves, 1);
        assert_eq!(parsed.state.full_moves, 2);

        if let Some(square) = parsed.state.en_passant_square {
            assert_eq!(square.col, 4); // e file
            assert_eq!(square.row, 5); // 3rd rank (corrected: 7-3+1 = 5)
        } else {
            panic!("Expected en passant square");
        }
    }

    #[test]
    fn test_no_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        assert!(!parsed.state.castle_white_short);
        assert!(!parsed.state.castle_white_long);
        assert!(!parsed.state.castle_black_short);
        assert!(!parsed.state.castle_black_long);
    }

    #[test]
    fn test_partial_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        assert!(parsed.state.castle_white_short); // K
        assert!(!parsed.state.castle_white_long); // no Q
        assert!(!parsed.state.castle_black_short); // no k
        assert!(parsed.state.castle_black_long); // q
    }

    #[test]
    fn test_promoted_pieces() {
        let fen = "rnbqkbnQ/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.board[0][7], Some(pieces::PieceType::WhiteQueen)); // White queen on h8 (promoted)
    }

    #[test]
    fn test_high_move_numbers() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 50 100";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.state.half_moves, 50);
        assert_eq!(parsed.state.full_moves, 100);
    }

    #[test]
    fn test_en_passant() {
        let fen = "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        assert_eq!(
            parsed.state.en_passant_square,
            Some(square::Square::new(2, 4))
        );
    }
}

mod invalid_fen_tests {
    use super::*;

    #[test]
    fn test_empty_fen() {
        let result = Chess::parse_fen("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            err::ChessError::InvalidFen(_)
        ));
    }

    #[test]
    fn test_fen_without_spaces() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_pieces() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQXBNR w KQkq - 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_turn() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_castling() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqX - 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_en_passant() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq z9 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_move_numbers() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - abc 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_too_few_fields() {
        let result = Chess::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_characters() {
        let result = Chess::parse_fen("rnbqklnr/ppppp#pp/8/8/8/8/PPPPPMMP/RNB~KBNR w KQkq - 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_king_count() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKKBNR w Kq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_row() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_col() {
        let fen = "rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }

    #[test]
    fn test_king_too_close() {
        let fen = "rnbqkKnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }
    #[test]
    fn test_king_too_close2() {
        let fen = "rnbq1bnr/ppp1pppp/8/1k1p4/1K1P4/8/PPP1PPPP/RNBQ1BNR w - - 6 5";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }

    #[test]
    fn test_pawn_on_first_or_last_rank() {
        let fen = "Pnbqkpnr/1ppppppp/8/8/8/8/1PPPPPPP/RNBQKBNp w KQkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_castle_pieces() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KPkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_err());
    }
}

mod fen_stringifying_tests {
    use super::*;

    #[test]
    fn test_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = Chess::parse_fen(fen);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        let stringified = Chess::stringify(&parsed);

        assert!(stringified.is_ok());
        assert_eq!(stringified.unwrap(), fen);
    }
}
