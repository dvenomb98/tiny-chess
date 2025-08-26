//!
//! # Validation Module
//!

use crate::err;
use crate::moves;
use crate::pieces;
use crate::square;
use crate::state;
use crate::types;

/// The `validate_moves` function validates a list of moves against chess rules.
///
/// 1. Validates each move against the chess rules.
///
/// Returns a list of validated moves.
///
pub fn validate_moves(req_moves: Vec<types::Move>, game: types::ParsedFen) -> Vec<types::Move> {
    let mut validated_moves = Vec::new();

    for req_move in req_moves {
        if validate_move(req_move, game).is_ok() {
            validated_moves.push(req_move);
        }
    }

    validated_moves
}

/// The `validate_move` function validation move agains chess rules.
///
///  1. Capturing own pieces
///  2. King would end up in check after the move
///  3. Current player king not found on board
///  4. Opponent has no pieces (corrupted game state)
///  5. King cannot castle (no castling rights)
///  6. Cannot castle short - path is under attack
///  7. Cannot castle long - path is under attack
///
/// If the move is valid, the function returns the next game state.
///
/// For validation against the game state, see `validate_move_against_state`.
///
pub fn validate_move(
    req_move: types::Move,
    game: types::ParsedFen,
) -> types::ChessResult<types::ParsedFen> {
    if game.is_own_square(
        req_move.to_row_idx,
        req_move.to_col_idx,
        req_move.piece.color(),
    ) {
        return Err(err::ChessError::InvalidMove(format!(
            "Cannot move {:?} to {:?}{:?} because it's own piece",
            req_move.piece, req_move.to_row_idx, req_move.to_col_idx
        )));
    }

    let next_game = state::get_next(req_move, game)?;

    let mut king_position: Option<square::Square> = None;
    let mut opponent_pieces: Vec<square::Square> = Vec::new();

    let is_white = req_move.piece.is_white();
    let target_king_piece = match is_white {
        true => pieces::PieceType::WhiteKing,
        false => pieces::PieceType::BlackKing,
    };

    for (row_idx, row) in next_game.board.iter().enumerate() {
        for (col_idx, piece) in row.iter().enumerate() {
            if let Some(found_piece) = piece {
                if *found_piece == target_king_piece {
                    king_position = Some(square::Square::new(row_idx, col_idx));
                }

                if found_piece.is_white() != is_white {
                    opponent_pieces.push(square::Square::new(row_idx, col_idx));
                }
            }
        }
    }

    if king_position.is_none() {
        return Err(err::ChessError::InvalidMove(
            "Current player king not found".to_string(),
        ));
    }

    if opponent_pieces.is_empty() {
        return Err(err::ChessError::InvalidMove(
            "Opponent has no pieces".to_string(),
        ));
    }

    let (can_castle_short, can_castle_long) = game.get_castle_ability(req_move.piece.color());

    for piece_sq in opponent_pieces {
        let pseudo_moves = moves::get_pseudo_moves(piece_sq, next_game);

        if pseudo_moves.is_empty() {
            continue;
        }

        if pseudo_moves
            .iter()
            .find(|m| {
                m.to_col_idx == king_position.unwrap().col
                    && m.to_row_idx == king_position.unwrap().row
            })
            .is_some()
        {
            return Err(err::ChessError::InvalidMove(
                "King would end up in check".to_string(),
            ));
        }

        if req_move.piece.piece_kind() == pieces::PieceKind::King && req_move.is_castle {
            if !can_castle_short && !can_castle_long {
                return Err(err::ChessError::InvalidMove(
                    "King cannot castle.".to_string(),
                ));
            }

            let king_row = match is_white {
                true => 7,
                false => 0,
            };

            if can_castle_short {
                let short_castle_path = vec![
                    square::Square::new(king_row, 4),
                    square::Square::new(king_row, 5),
                    square::Square::new(king_row, 6),
                ];

                if pseudo_moves.iter().any(|m| {
                    short_castle_path.iter().any(|path_square| {
                        m.to_row_idx == path_square.row && m.to_col_idx == path_square.col
                    })
                }) {
                    return Err(err::ChessError::InvalidMove(
                        "Cannot castle short: path is under attack".to_string(),
                    ));
                }
            }

            if can_castle_long {
                let long_castle_path = vec![
                    square::Square::new(king_row, 4),
                    square::Square::new(king_row, 3),
                    square::Square::new(king_row, 2),
                ];

                if pseudo_moves.iter().any(|m| {
                    long_castle_path.iter().any(|path_square| {
                        m.to_row_idx == path_square.row && m.to_col_idx == path_square.col
                    })
                }) {
                    return Err(err::ChessError::InvalidMove(
                        "Cannot castle long: path is under attack".to_string(),
                    ));
                }
            }
        }
    }

    Ok(next_game)
}

/// The `validate_move_against_state` function validates the move against the game state.
///
///  1. Wrong turn - piece color doesn't match current player
///
/// If the move is valid, the function returns `Ok(())`.
///
/// For validation against chess rules, see `validate_move`.
pub fn validate_move_against_state(
    req_move: types::Move,
    game: types::ParsedFen,
) -> types::ChessResult<()> {
    if game.state.on_turn != req_move.piece.color() {
        return Err(err::ChessError::InvalidMove(format!(
            "Cannot move {:?} when player on turn is {:?}",
            req_move.piece, game.state.on_turn
        )));
    }

    Ok(())
}
