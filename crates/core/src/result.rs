//! # Game Result Module
//!
//! This module handles game result.
//!
//! Valid game results:
//! - Checkmate
//! - Stalemate
//! - Insufficient material
//! - Fifty move rule
//! - Threefold repetition (TODO)
//!
//! Insufficient material list:
//! 1. King vs. king
//! 2. King and bishop vs. king
//! 3. King and knight vs. king
//! 4. King and bishop vs. king and bishop (bishops have to be on the same color)
use serde::{Serialize, Deserialize};

use crate::Chess;
use crate::err;
use crate::pieces;
use crate::player;
use crate::square;
use crate::types;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GameResult {
    WhiteCheckmate,
    BlackCheckmate,
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
    ThreefoldRepetition
}

pub fn get_game_result(game: types::ParsedFen) -> types::ChessResult<Option<GameResult>> {
    if game.state.half_moves >= 100 {
        return Ok(Some(GameResult::FiftyMoveRule));
    }

    let target_king_piece = match game.state.on_turn {
        player::Player::White => pieces::PieceType::WhiteKing,
        player::Player::Black => pieces::PieceType::BlackKing,
    };

    let mut king_position: Option<square::Square> = None;
    let mut player_pieces: Vec<square::Square> = Vec::new();
    let mut opponent_pieces: Vec<square::Square> = Vec::new();
    let mut player_has_king = false;
    let mut player_has_bishop = false;
    let mut player_has_knight = false;
    let mut player_bishop_pos: Option<square::Square> = None;
    let mut opponent_has_king = false;
    let mut opponent_has_bishop = false;
    let mut opponent_has_knight = false;
    let mut opponent_bishop_pos: Option<square::Square> = None;

    for (row_idx, row) in game.board.iter().enumerate() {
        for (col_idx, piece) in row.iter().enumerate() {
            if let Some(piece) = piece {
                let square = square::Square::new(row_idx, col_idx);

                if *piece == target_king_piece {
                    king_position = Some(square);
                }

                if piece.color() == game.state.on_turn {
                    player_pieces.push(square);
                    match piece {
                        pieces::PieceType::WhiteKing | pieces::PieceType::BlackKing => {
                            player_has_king = true
                        }
                        pieces::PieceType::WhiteBishop | pieces::PieceType::BlackBishop => {
                            player_has_bishop = true;
                            player_bishop_pos = Some(square);
                        }
                        pieces::PieceType::WhiteKnight | pieces::PieceType::BlackKnight => {
                            player_has_knight = true
                        }
                        _ => {}
                    }
                } else {
                    opponent_pieces.push(square);
                    match piece {
                        pieces::PieceType::WhiteKing | pieces::PieceType::BlackKing => {
                            opponent_has_king = true
                        }
                        pieces::PieceType::WhiteBishop | pieces::PieceType::BlackBishop => {
                            opponent_has_bishop = true;
                            opponent_bishop_pos = Some(square);
                        }
                        pieces::PieceType::WhiteKnight | pieces::PieceType::BlackKnight => {
                            opponent_has_knight = true
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if king_position.is_none() {
        return Err(err::ChessError::InvalidParsedFen(game));
    }

    if player_pieces.is_empty() {
        return Err(err::ChessError::InvalidParsedFen(game));
    }

    if opponent_pieces.is_empty() {
        return Err(err::ChessError::InvalidParsedFen(game));
    }

    let is_insufficient = match (player_pieces.len(), opponent_pieces.len()) {
        // King vs King
        (1, 1) => player_has_king && opponent_has_king,
        // King vs King + Bishop || King vs King + Knight
        (1, 2) => {
            (player_has_king && opponent_has_king && (opponent_has_bishop || opponent_has_knight))
                || (opponent_has_king
                    && player_has_king
                    && (player_has_bishop || player_has_knight))
        }
        (2, 1) => {
            (player_has_king && opponent_has_king && (player_has_bishop || player_has_knight))
                || (opponent_has_king
                    && player_has_king
                    && (opponent_has_bishop || opponent_has_knight))
        }
        // King + Bishop || King + Bishop (same color bishops)
        (2, 2) => {
            player_has_king
                && opponent_has_king
                && player_has_bishop
                && opponent_has_bishop
                && if let (Some(player_pos), Some(opponent_pos)) =
                    (player_bishop_pos, opponent_bishop_pos)
                {
                    (player_pos.row + player_pos.col) % 2
                        == (opponent_pos.row + opponent_pos.col) % 2
                } else {
                    false
                }
        }
        _ => false,
    };

    if is_insufficient {
        return Ok(Some(GameResult::InsufficientMaterial));
    }

    for piece in player_pieces {
        let valid_moves = Chess::get_moves(piece, game);
        if !valid_moves.is_empty() {
            return Ok(None);
        }
    }

    for piece in opponent_pieces {
        let valid_moves = Chess::get_moves(piece, game);

        if valid_moves.is_empty() {
            continue;
        }

        if valid_moves
            .iter()
            .find(|m| {
                m.to_col_idx == king_position.unwrap().col
                    && m.to_row_idx == king_position.unwrap().row
            })
            .is_some()
        {
            let checkmate_by = match game.state.on_turn {
                player::Player::White => GameResult::BlackCheckmate,
                player::Player::Black => GameResult::WhiteCheckmate,
            };

            return Ok(Some(checkmate_by));
        }
    }

    return Ok(Some(GameResult::Stalemate));
}
