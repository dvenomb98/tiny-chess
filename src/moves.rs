use crate::pieces;
use crate::player;
use crate::square;
use crate::types;

pub fn get_moves(target: square::Square, game: types::ParsedFen) -> Vec<types::Move> {
    let piece = match target.get_piece(&game.board) {
        Some(piece) => piece,
        None => return Vec::new(),
    };

    match piece.piece_kind() {
        pieces::PieceKind::Pawn => pawn_moves(target, piece, game),
        pieces::PieceKind::Bishop => bishop_moves(),
        pieces::PieceKind::Rook => rook_moves(),
        pieces::PieceKind::Queen => queen_moves(),
        pieces::PieceKind::Knight => knight_moves(),
        pieces::PieceKind::King => king_moves(),
    }
}

fn pawn_moves(
    target: square::Square,
    piece: pieces::PieceType,
    game: types::ParsedFen,
) -> Vec<types::Move> {
    let moves: &mut Vec<types::Move> = &mut Vec::new();
    let diagonals = [1, -1];
    let piece_color = piece.color();
    let (direction, start_row) = match piece_color {
        player::Player::White => (-1, 6),
        player::Player::Black => (1, 1),
    };

    if let Some(one_step) = target.row.checked_add_signed(direction) {
        // One step forward
        if game.board[one_step][target.col].is_none() {
            moves.push(types::Move {
                from_col_idx: target.col,
                from_row_idx: target.row,
                to_col_idx: target.col,
                to_row_idx: one_step,
                is_castle: false,
                is_passant: false,
                piece,
            });
        }
        // Second step forward
        if target.row == start_row {
            if let Some(second_step) = one_step.checked_add_signed(direction) {
                if game.board[second_step][target.col].is_none() {
                    moves.push(types::Move {
                        from_col_idx: target.col,
                        from_row_idx: target.row,
                        to_col_idx: target.col,
                        to_row_idx: second_step,
                        is_castle: false,
                        is_passant: false,
                        piece,
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
                if let Some(captured_piece) = game.board[new_row][new_col] {
                    if captured_piece.color() != piece_color {
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

    return moves.to_vec();
}

fn bishop_moves() -> Vec<types::Move> {
    todo!();
}

fn rook_moves() -> Vec<types::Move> {
    todo!();
}

fn knight_moves() -> Vec<types::Move> {
    todo!();
}

fn king_moves() -> Vec<types::Move> {
    todo!();
}

fn queen_moves() -> Vec<types::Move> {
    todo!();
}
