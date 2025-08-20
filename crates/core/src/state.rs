use crate::pieces;
use crate::player;
use crate::square;
use crate::types;

pub (super) fn get_next(
    req_move: types::Move,
    game: types::ParsedFen,
) -> types::ChessResult<types::ParsedFen> {
    let next_board = self::get_next_board(req_move, game)?;
    let next_state = self::get_next_state(req_move, game);

    Ok(types::ParsedFen {
        board: next_board,
        state: next_state,
    })
}

pub fn get_next_board(
    req_move: types::Move,
    game: types::ParsedFen,
) -> types::ChessResult<types::Board> {
    let mut next = game;

    // target, source swap
    next.assign_square(req_move.from_row_idx, req_move.from_col_idx, None)?;
    next.assign_square(
        req_move.to_row_idx,
        req_move.to_col_idx,
        Some(req_move.piece),
    )?;

    // passant
    if req_move.is_passant {
        next.assign_square(req_move.from_row_idx, req_move.to_col_idx, None)?;
    }
    //castle
    if req_move.is_castle {
        let (rook_source_col, rook_target_col) = match req_move.to_col_idx {
            6 => (7, 5),
            _ => (0, 3),
        };
        let rook_piece = match req_move.piece.color() {
            player::Player::White => pieces::PieceType::WhiteRook,
            player::Player::Black => pieces::PieceType::BlackRook,
        };

        next.assign_square(req_move.to_row_idx, rook_source_col, None)?;
        next.assign_square(req_move.to_row_idx, rook_target_col, Some(rook_piece))?;
    }

    // TODO replace promotion_piece with piece from player
    let (promotion_row, allowed_upg_piece, promotion_piece) = match req_move.piece.color() {
        player::Player::White => (
            0,
            pieces::PieceType::WhitePawn,
            pieces::PieceType::WhiteQueen,
        ),
        player::Player::Black => (
            7,
            pieces::PieceType::BlackPawn,
            pieces::PieceType::BlackQueen,
        ),
    };

    if req_move.to_row_idx == promotion_row && req_move.piece == allowed_upg_piece {
        next.assign_square(
            req_move.to_row_idx,
            req_move.to_col_idx,
            Some(promotion_piece),
        )?;
    };

    Ok(next.board)
}

pub fn get_next_state(req_move: types::Move, game: types::ParsedFen) -> types::ParsedFenState {
    let mut next_state = game.state;

    let piece_color = req_move.piece.color();
    let is_white = req_move.piece.is_white();
    let piece_kind = req_move.piece.piece_kind();

    let (can_castle_short, can_castle_long) = game.get_castle_ability(piece_color);

    // castle block
    if can_castle_long || can_castle_short {
        match req_move.piece {
            pieces::PieceType::BlackKing => {
                next_state.castle_black_long = false;
                next_state.castle_black_short = false;
            }
            pieces::PieceType::WhiteRook | pieces::PieceType::BlackRook => {
                if req_move.from_col_idx == 0 {
                    if is_white {
                        next_state.castle_white_long = false
                    } else {
                        next_state.castle_black_long = false
                    }
                }

                if req_move.from_col_idx == 7 {
                    if is_white {
                        next_state.castle_white_short = false
                    } else {
                        next_state.castle_black_short = false
                    }
                }
            }
            pieces::PieceType::WhiteKing => {
                next_state.castle_white_long = false;
                next_state.castle_white_short = false;
            }
            _ => {}
        }
    }
    // passant block
    if piece_kind == pieces::PieceKind::Pawn {
        let row_diff = req_move.to_row_idx.abs_diff(req_move.from_row_idx);

        if row_diff == 2 {
            if is_white {
                next_state.en_passant_square = Some(square::Square::new(
                    req_move.to_row_idx + 1,
                    req_move.to_col_idx,
                ));
            } else {
                next_state.en_passant_square = Some(square::Square::new(
                    req_move.to_row_idx - 1,
                    req_move.to_col_idx,
                ));
            }
        }
    }

    // half moves
    if piece_kind == pieces::PieceKind::Pawn
        || game.is_enemy_square(req_move.to_row_idx, req_move.to_col_idx, piece_color)
    {
        next_state.half_moves = 0
    } else {
        next_state.half_moves += 1
    }

    // full moves & onTurn
    if !is_white {
        next_state.full_moves += 1;
        next_state.on_turn = player::Player::White
    } else {
        next_state.on_turn = player::Player::Black
    }

    next_state
}
