//! # Moves Module
//!
//! This module is reponsible for generating pseudo-moves
//! Pseudo move is move, where it generate possible moves for each piece at current board, but without chess rules validation (eg. exposing check after moving piece);
//!

use crate::pieces;
use crate::player;
use crate::square;
use crate::types;

const DIAGONAL_DIRS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const STRAIGHT_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const KNIGHT_DIRS: [(isize, isize); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

/// Get pseudo moves for a given piece at a given square
pub fn get_pseudo_moves(target: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
    let piece = match game.safe_access_square(target.row, target.col) {
        Ok(Some(piece)) => piece,
        _ => return Vec::new(),
    };

    match piece.piece_kind() {
        pieces::PieceKind::Pawn => self::pawn_moves(target, piece, game),
        pieces::PieceKind::Bishop => self::bishop_moves(target, piece, game),
        pieces::PieceKind::Rook => self::rook_moves(target, piece, game),
        pieces::PieceKind::Queen => self::queen_moves(target, piece, game),
        pieces::PieceKind::Knight => self::knight_moves(target, piece, game),
        pieces::PieceKind::King => self::king_moves(target, piece, game),
    }
}

/// Pawn pseudo moves
fn pawn_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    let mut moves = Vec::new();
    let diagonals = [1, -1];
    let piece_color = piece.color();
    let (direction, start_row) = match piece_color {
        player::Player::White => (-1, 6),
        player::Player::Black => (1, 1),
    };

    if let Some(one_step) = target.row.checked_add_signed(direction) {
        // One step forward
        if game.is_square_empty(one_step, target.col) {
            moves.push(types::Move {
                from_col_idx: target.col,
                from_row_idx: target.row,
                to_col_idx: target.col,
                to_row_idx: one_step,
                is_castle: false,
                is_passant: false,
                piece
            });
        }
        // Second step forward
        if target.row == start_row {
            if let Some(second_step) = one_step.checked_add_signed(direction) {
                if game.is_square_empty(second_step, target.col) {
                    moves.push(types::Move {
                        from_col_idx: target.col,
                        from_row_idx: target.row,
                        to_col_idx: target.col,
                        to_row_idx: second_step,
                        is_castle: false,
                        is_passant: false,
                        piece
                    });
                }
            }
        }

        // Captures
        for diagonal in diagonals {
            if let (Some(new_row), Some(new_col)) = (
                target.row.checked_add_signed(direction),
                target.col.checked_add_signed(diagonal),
            ) {
                if game.is_enemy_square(new_row, new_col, piece_color) {
                    moves.push(types::Move {
                        from_col_idx: target.col,
                        from_row_idx: target.row,
                        to_col_idx: new_col,
                        to_row_idx: new_row,
                        is_castle: false,
                        is_passant: false,
                        piece,
                    });
                } else if let Some(en_passant_square) = game.state.en_passant_square {
                    if new_row == en_passant_square.row && new_col == en_passant_square.col {
                        moves.push(types::Move {
                            from_col_idx: target.col,
                            from_row_idx: target.row,
                            to_col_idx: new_col,
                            to_row_idx: new_row,
                            is_castle: false,
                            is_passant: true,
                            piece,
                        });
                    }
                }
            }
        }
    }

    moves
}

/// Bishop pseudo moves
fn bishop_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    self::get_direction_moves(target, piece, game, &DIAGONAL_DIRS)
}

/// Rook pseudo moves
fn rook_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    self::get_direction_moves(target, piece, game, &STRAIGHT_DIRS)
}

/// Knight pseudo moves
fn knight_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    let mut moves = Vec::new();
    let piece_color = piece.color();

    for &(row_dir, col_dir) in &KNIGHT_DIRS {
        if let (Some(new_row), Some(new_col)) = (
            target.row.checked_add_signed(row_dir),
            target.col.checked_add_signed(col_dir),
        ) {
            if game.is_square_empty(new_row, new_col)
                || game.is_enemy_square(new_row, new_col, piece_color)
            {
                moves.push(types::Move {
                    from_col_idx: target.col,
                    from_row_idx: target.row,
                    to_col_idx: new_col,
                    to_row_idx: new_row,
                    is_castle: false,
                    is_passant: false,
                    piece,
                });
            }
        }
    }

    moves
}

/// King pseudo moves
fn king_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    let mut moves: Vec<types::Move> = Vec::new();

    let mut directions: Vec<(isize, isize)> = Vec::new();
    directions.extend(&STRAIGHT_DIRS);
    directions.extend(&DIAGONAL_DIRS);

    for (row, col) in directions {
        if let (Some(new_row), Some(new_col)) = (
            target.row.checked_add_signed(row),
            target.col.checked_add_signed(col),
        ) {
            if game.is_square_empty(new_row, new_col)
                || game.is_enemy_square(new_row, new_col, piece.color())
            {
                moves.push(types::Move {
                    from_col_idx: target.col,
                    from_row_idx: target.row,
                    to_col_idx: new_col,
                    to_row_idx: new_row,
                    piece,
                    is_castle: false,
                    is_passant: false,
                })
            }
        }
    }

    let (can_castle_short, can_castle_long) = game.get_castle_ability(piece.color());

    let king_initial_row = match piece.color() {
        player::Player::White => 7,
        player::Player::Black => 0,
    };
    let king_initial_col = 4;
    let rook_short_col = 7;
    let rook_long_col = 0;
    let rook_piece = match piece.color() {
        player::Player::White => pieces::PieceType::WhiteRook,
        player::Player::Black => pieces::PieceType::BlackRook,
    };

    // king is at initial position
    if target.col == king_initial_col && target.row == king_initial_row {
        // short castle
        if can_castle_short {
            let short_path_clear = game.is_square_empty(king_initial_row, king_initial_col + 1)
                && game.is_square_empty(king_initial_row, king_initial_col + 2);
            let is_rook_in_place = game
                .safe_access_square(king_initial_row, rook_short_col)
                .is_ok_and(|piece| piece.is_some() && piece.unwrap() == rook_piece);

            if short_path_clear && is_rook_in_place {
                moves.push(types::Move {
                    from_col_idx: target.col,
                    from_row_idx: target.row,
                    to_col_idx: king_initial_col + 2,
                    to_row_idx: king_initial_row,
                    is_castle: true,
                    is_passant: false,
                    piece,
                });
            }
        }

        // long castle
        if can_castle_long {
            let long_path_clear = game.is_square_empty(king_initial_row, king_initial_col - 1)
                && game.is_square_empty(king_initial_row, king_initial_col - 2)
                && game.is_square_empty(king_initial_row, king_initial_col - 3);
            let is_rook_in_place = game
                .safe_access_square(king_initial_row, rook_long_col)
                .is_ok_and(|piece| piece.is_some() && piece.unwrap() == rook_piece);

            if long_path_clear && is_rook_in_place {
                moves.push(types::Move {
                    from_col_idx: target.col,
                    from_row_idx: target.row,
                    to_col_idx: king_initial_col - 2,
                    to_row_idx: king_initial_row,
                    is_castle: true,
                    is_passant: false,
                    piece,
                });
            }
        }
    }

    moves
}

/// Queen pseudo moves
fn queen_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    let mut moves = Vec::new();
    moves.extend(self::get_direction_moves(target, piece, game, &STRAIGHT_DIRS));
    moves.extend(self::get_direction_moves(target, piece, game, &DIAGONAL_DIRS));
    moves
}

/// Get direction moves for a given piece at a given square
fn get_direction_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
    directions: &[(isize, isize)],
) -> Vec<types::Move> {
    let mut moves = Vec::new();
    let piece_color = piece.color();

    for &(row_dir, col_dir) in directions {
        let mut current_row = target.row;
        let mut current_col = target.col;

        loop {
            if let (Some(new_row), Some(new_col)) = (
                current_row.checked_add_signed(row_dir),
                current_col.checked_add_signed(col_dir),
            ) {
                if !game.is_in_bounds(new_row, new_col) {
                    break;
                }

                if game.is_square_empty(new_row, new_col) {
                    moves.push(types::Move {
                        from_col_idx: target.col,
                        from_row_idx: target.row,
                        to_col_idx: new_col,
                        to_row_idx: new_row,
                        is_castle: false,
                        is_passant: false,
                        piece,
                    });
                    current_row = new_row;
                    current_col = new_col;
                } else if game.is_enemy_square(new_row, new_col, piece_color) {
                    moves.push(types::Move {
                        from_col_idx: target.col,
                        from_row_idx: target.row,
                        to_col_idx: new_col,
                        to_row_idx: new_row,
                        is_castle: false,
                        is_passant: false,
                        piece,
                    });
                    // stop in this direction after capture
                    break;
                } else {
                    // friendly piece - stop in this direction (no move added)
                    break;
                }
            } else {
                // checked_add_signed returned None (overflow), out of bounds
                break;
            }
        }
    }

    moves
}
