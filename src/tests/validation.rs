#[cfg(test)]
mod validation_tests {
    use crate::{Chess, pieces, types, validation};

    #[test]
    pub fn test_scholar_mate() {
        let game =
            Chess::parse_fen("r1bqk1nr/pppp1Qpp/2n5/2b1p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4")
                .unwrap();

        let king_move = validation::validate_move(
            types::Move {
                from_col_idx: 4,
                from_row_idx: 0,
                to_col_idx: 4,
                to_row_idx: 1,
                is_castle: false,
                is_passant: false,
                piece: pieces::PieceType::BlackKing,
            },
            game,
        );

        assert_eq!(king_move.is_err(), true);

        let queen_move = validation::validate_move(
            types::Move {
                from_col_idx: 3,
                from_row_idx: 7,
                to_col_idx: 7,
                to_row_idx: 4,
                is_castle: false,
                is_passant: false,
                piece: pieces::PieceType::BlackQueen,
            },
            game,
        );

        assert_eq!(queen_move.is_err(), true);
    }

    #[test]
    pub fn test_exposing_check() {
        let game =
            Chess::parse_fen("r1bqk1nr/pppp1ppp/2n5/2b1p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 5 4")
                .unwrap();

        let expose_check_move = validation::validate_move(
            types::Move {
                from_col_idx: 5,
                from_row_idx: 1,
                to_col_idx: 5,
                to_row_idx: 2,
                is_castle: false,
                is_passant: false,
                piece: pieces::PieceType::BlackPawn,
            },
            game,
        );

        assert_eq!(expose_check_move.is_err(), true);
    }

    #[test]
    pub fn test_blocking_castle() {
        let game =
            Chess::parse_fen("2b1kbnr/3npppp/r1p5/1p1pN3/pP1P3B/NQ1q3P/P3PPP1/R3KB1R w KQk - 4 13")
                .unwrap();

        let castle_move = validation::validate_move(
            types::Move {
                from_col_idx: 4,
                from_row_idx: 7,
                to_col_idx: 2,
                to_row_idx: 7,
                is_castle: true,
                is_passant: false,
                piece: pieces::PieceType::WhiteKing,
            },
            game,
        );

        assert_eq!(castle_move.is_err(), true);
    }
}
