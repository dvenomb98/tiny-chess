//! # FEN Module
//!
//! This module handles parsing and stringifying FEN (Forsyth-Edwards Notation) strings.
//!
//! ## Validation Rules
//! We are not validating the 100% correct position, but we are validating the most common mistakes.
//! This approach keeps flexibility constructing the board from principal incorrect positions
//!
//! - Is FEN structure correct (eg. correct number of slashes, format, correct characters etc..)
//! - Is grid size correct?
//! - Is there at most 1 king on each side?
//! - Are there no pawns on the first or eighth rank?
//! - Are kings at least 1 square apart?
//!
use crate::err;
use crate::types;
use crate::utils;

const GRID_SIZE_INDEX: usize = 7;
const GRID_SIZE: usize = 8;
const EXACT_KINGS_COUNT: u8 = 1;
const MAX_DISTANCE_BETWEEN_KINGS: u8 = 2;

/// Parse FEN string into structured data.
pub fn parse(fen: &str) -> err::ChessResult<types::ParsedFen> {
    let splitted_fen = split_by_first_whitespace(fen)?;
    let board = fen_to_board(splitted_fen.0)?;
    let state = fen_to_state(splitted_fen.1)?;

    return Ok(types::ParsedFen { board, state });
}

/// Convert structured FEN data back to FEN notation string.
pub fn stringify(parsed_fen: &types::ParsedFen) -> err::ChessResult<String> {
    let board = stringify_to_board(parsed_fen)?;
    let state = stringify_to_state(parsed_fen)?;

    return Ok(format!("{} {}", board, state));
}

/// Stringify the parsed_fen.board into fen string
fn stringify_to_board(parsed_fen: &types::ParsedFen) -> err::ChessResult<String> {
    let mut fen = String::new();

    let mut white_kings_count = 0;
    let mut black_kings_count = 0;

    let mut white_king_position = types::Square(0, 0);
    let mut black_king_position = types::Square(0, 0);

    for (row_idx, row) in parsed_fen.board.iter().enumerate() {
        let mut count = 0;

        if !fen.is_empty() {
            fen.push('/');
        }

        for (col, piece) in row.iter().enumerate() {
            if let Some(piece_char) = piece {
                if types::VALID_PIECES.contains(*piece_char) {
                    if count != 0 {
                        fen.push_str(&count.to_string());
                        count = 0;
                    }

                    fen.push(*piece_char);

                    if validate_piece_placement(
                        *piece_char,
                        col,
                        row_idx,
                        &mut white_kings_count,
                        &mut black_kings_count,
                        &mut white_king_position,
                        &mut black_king_position,
                    )
                    .is_err()
                    {
                        return Err(get_parsed_fen_error(parsed_fen));
                    }
                } else {
                    return Err(get_parsed_fen_error(parsed_fen));
                }
            } else {
                count += 1;
                if col >= GRID_SIZE_INDEX {
                    fen.push_str(&count.to_string());
                    count = 0;
                }
            }
        }
    }

    if validate_king_placement(
        &white_king_position,
        &black_king_position,
        &mut black_kings_count,
        &mut white_kings_count,
    )
    .is_err()
    {
        return Err(get_parsed_fen_error(parsed_fen));
    }

    return Ok(fen);
}

// Stringify the parsed_fen.state into fen string
fn stringify_to_state(parsed_fen: &types::ParsedFen) -> err::ChessResult<String> {
    let types::ParsedFenState {
        full_moves,
        half_moves,
        en_passant_square,
        on_turn,
        castle_black_long,
        castle_black_short,
        castle_white_long,
        castle_white_short,
    } = parsed_fen.state;

    let mut fen = String::new();

    match on_turn {
        types::Player::BLACK => fen.push(types::BLACK_PLAYER),
        types::Player::WHITE => fen.push(types::WHITE_PLAYER),
    }

    fen.push(' ');

    if !castle_black_long && !castle_black_short && !castle_white_long && !castle_white_short {
        fen.push('-')
    } else {
        if castle_white_short {
            fen.push(types::WHITE_KING)
        }
        if castle_white_long {
            fen.push(types::WHITE_QUEEN)
        }
        if castle_black_short {
            fen.push(types::BLACK_KING)
        }
        if castle_black_long {
            fen.push(types::BLACK_QUEEN)
        }
    }

    fen.push(' ');

    if let Some(enp) = en_passant_square {
        let file = utils::coordinates::index_to_file_char(enp.0)
            .ok_or_else(|| get_parsed_fen_error(parsed_fen))?;
        let rank = utils::coordinates::rank_index_to_char(enp.1)
            .ok_or_else(|| get_parsed_fen_error(parsed_fen))?;

        fen.push(file);
        fen.push(rank);
    } else {
        fen.push('-')
    }

    fen.push(' ');

    fen.push_str(&half_moves.to_string());

    fen.push(' ');

    fen.push_str(&full_moves.to_string());

    return Ok(fen);
}

///
/// Parse board from first part of fen.
///
fn fen_to_board(first_fen_part: &str) -> err::ChessResult<types::Board> {
    let mut board: types::Board = [[None; 8]; 8];
    let mut row = 0;
    let mut col = 0;

    let mut white_kings_count = 0;
    let mut black_kings_count = 0;

    let mut white_king_position = types::Square(0, 0);
    let mut black_king_position = types::Square(0, 0);

    for ch in first_fen_part.chars() {
        match ch {
            '/' => {
                if col < GRID_SIZE {
                    return Err(get_fen_error(first_fen_part));
                }

                row += 1;
                col = 0;

                if row > GRID_SIZE_INDEX {
                    return Err(get_fen_error(first_fen_part));
                }
            }
            '1'..='8' => {
                col += ch.to_digit(10).unwrap() as usize;
                if col > GRID_SIZE {
                    return Err(get_fen_error(first_fen_part));
                }
            }
            piece => {
                if !types::VALID_PIECES.contains(piece) {
                    return Err(get_fen_error(first_fen_part));
                }

                if validate_piece_placement(
                    piece,
                    col,
                    row,
                    &mut white_kings_count,
                    &mut black_kings_count,
                    &mut white_king_position,
                    &mut black_king_position,
                )
                .is_err()
                {
                    return Err(get_fen_error(first_fen_part));
                }

                board[row][col] = Some(piece);
                col += 1;

                if col > GRID_SIZE {
                    return Err(get_fen_error(first_fen_part));
                }
            }
        }
    }

    if row != GRID_SIZE_INDEX {
        return Err(get_fen_error(first_fen_part));
    }

    if validate_king_placement(
        &white_king_position,
        &black_king_position,
        &mut black_kings_count,
        &mut white_kings_count,
    )
    .is_err()
    {
        return Err(get_fen_error(first_fen_part));
    }

    Ok(board)
}

///
/// Parse state from second part of fen.
///
fn fen_to_state(second_fen_part: &str) -> err::ChessResult<types::ParsedFenState> {
    let array_from_values: Vec<&str> = second_fen_part.split_whitespace().collect();

    if array_from_values.len() < 5 {
        return Err(get_fen_error(second_fen_part));
    }

    let mut initial_state = types::ParsedFenState {
        en_passant_square: None,
        on_turn: types::Player::WHITE,
        castle_white_short: false,
        castle_white_long: false,
        castle_black_short: false,
        castle_black_long: false,
        half_moves: 0,
        full_moves: 0,
    };

    // Parse current turn
    match array_from_values[0] {
        "w" => initial_state.on_turn = types::Player::WHITE,
        "b" => initial_state.on_turn = types::Player::BLACK,
        _ => return Err(get_fen_error(second_fen_part)),
    }

    // Parse castling availability
    let castling = array_from_values[1];
    if castling != "-" {
        for ch in castling.chars() {
            match ch {
                types::WHITE_KING => initial_state.castle_white_short = true,
                types::WHITE_QUEEN => initial_state.castle_white_long = true,
                types::BLACK_KING => initial_state.castle_black_short = true,
                types::BLACK_QUEEN => initial_state.castle_black_long = true,
                _ => return Err(get_fen_error(second_fen_part)),
            }
        }
    }

    // Parse en passant target square
    let en_passant = array_from_values[2];
    if en_passant != "-" {
        let chars: Vec<char> = en_passant.chars().collect();

        if chars.len() != 2 {
            return Err(get_fen_error(second_fen_part));
        }

        let file = chars[0];
        let rank = chars[1];

        let col = utils::coordinates::file_char_to_index(file)
            .ok_or_else(|| get_fen_error(second_fen_part))?;

        let row = utils::coordinates::rank_char_to_index(rank)
            .ok_or_else(|| get_fen_error(second_fen_part))?;

        initial_state.en_passant_square = Some(types::Square(col, row));
    }

    // Parse halfmove clock
    initial_state.half_moves = array_from_values[3]
        .parse::<u32>()
        .map_err(|_| get_fen_error(second_fen_part))?;

    // Parse fullmove number
    initial_state.full_moves = array_from_values[4]
        .parse::<u32>()
        .map_err(|_| get_fen_error(second_fen_part))?;

    Ok(initial_state)
}

///
/// Get error for invalid fen.
///
fn get_fen_error(fen: &str) -> err::ChessError {
    return err::ChessError::InvalidFen(fen.to_string());
}

///
/// Get error for invalid parsed fen.
///
fn get_parsed_fen_error(parsed_fen: &types::ParsedFen) -> err::ChessError {
    return err::ChessError::InvalidParsedFen(parsed_fen.clone());
}

///
/// Split fen by first whitespace
/// First part is board, second part is state.
///
fn split_by_first_whitespace(fen: &str) -> err::ChessResult<(&str, &str)> {
    return fen.split_once(" ").ok_or(get_fen_error(fen));
}

/// Validates a piece and updates king counts/positions
/// Returns an error if the piece placement is invalid (e.g., pawns on first/last rank)
fn validate_piece_placement(
    piece_char: char,
    col: usize,
    row_idx: usize,
    white_kings_count: &mut u8,
    black_kings_count: &mut u8,
    white_king_position: &mut types::Square,
    black_king_position: &mut types::Square,
) -> Result<(), ()> {
    match piece_char {
        types::WHITE_KING => {
            *white_kings_count += 1;
            *white_king_position = types::Square(col as u8, row_idx as u8);
        }
        types::BLACK_KING => {
            *black_kings_count += 1;
            *black_king_position = types::Square(col as u8, row_idx as u8);
        }
        types::WHITE_PAWN | types::BLACK_PAWN => {
            if row_idx == 0 || row_idx == GRID_SIZE_INDEX {
                return Err(());
            }
        }
        _ => {}
    }
    Ok(())
}

/// Validate that kings are at least 1 square apart
/// Validate if there is at least 1 king for each color
fn validate_king_placement(
    white_king_position: &types::Square,
    black_king_position: &types::Square,
    black_king_count: &mut u8,
    white_king_count: &mut u8,
) -> Result<(), ()> {
    if *white_king_count != EXACT_KINGS_COUNT || *black_king_count != EXACT_KINGS_COUNT {
        return Err(());
    }

    if white_king_position.0.abs_diff(black_king_position.0) < MAX_DISTANCE_BETWEEN_KINGS
        && white_king_position.1.abs_diff(black_king_position.1) < MAX_DISTANCE_BETWEEN_KINGS
    {
        return Err(());
    }
    Ok(())
}
