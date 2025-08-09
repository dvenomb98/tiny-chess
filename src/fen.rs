use crate::err;
use crate::types;
use crate::utils;

pub const INITIAL_FEN_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const GRID_SIZE_INDEX: usize = 7;
const GRID_SIZE: usize = 8;
const EXACT_KINGS_COUNT: u8 = 1;
const MAX_DISTANCE_BETWEEN_KINGS: u8 = 2;

/// Parse fen.
///
/// When parsing the fen we are separating validation from parsing.
///
/// We follow just the a few simple rules to parse the board:
/// - Is fen structure correct (eg. correct number of slashes, format, correct characters etc..)
/// - Is grid size correct?
/// - Is there at most 1 king on each side?
/// - Are there no pawns on the first or eighth rank?
/// - Are kings at least 1 square apart?
///
pub fn parse(fen: &str) -> err::ChessResult<types::ParsedFen> {
    let splitted_fen = split_by_first_whitespace(fen)?;
    let board = fen_to_board(splitted_fen.0)?;
    let state = fen_to_state(splitted_fen.1)?;

    return Ok(types::ParsedFen { board, state });
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
                board[row][col] = Some(piece);
                col += 1;

                if col > GRID_SIZE {
                    return Err(get_fen_error(first_fen_part));
                }

                match piece {
                    types::WHITE_KING => {
                        white_kings_count += 1;
                        white_king_position = types::Square(col as u8, row as u8);
                    }
                    types::BLACK_KING => {
                        black_kings_count += 1;
                        black_king_position = types::Square(col as u8, row as u8);
                    }
                    types::WHITE_PAWN | types::BLACK_PAWN => {
                        if row == 0 || row == GRID_SIZE_INDEX {
                            return Err(get_fen_error(first_fen_part));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if row != GRID_SIZE_INDEX {
        return Err(get_fen_error(first_fen_part));
    }

    if white_kings_count != EXACT_KINGS_COUNT || black_kings_count != EXACT_KINGS_COUNT {
        return Err(get_fen_error(first_fen_part));
    }

    if white_king_position.0.abs_diff(black_king_position.0) < MAX_DISTANCE_BETWEEN_KINGS
        && white_king_position.1.abs_diff(black_king_position.1) < MAX_DISTANCE_BETWEEN_KINGS
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

        let col = utils::char_to_col::parse(file).ok_or_else(|| get_fen_error(second_fen_part))?;

        let row = utils::parse_rank::parse(rank).ok_or_else(|| get_fen_error(second_fen_part))?;

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
/// Split fen by first whitespace
/// First part is board, second part is state.
///
fn split_by_first_whitespace(fen: &str) -> err::ChessResult<(&str, &str)> {
    return fen.split_once(" ").ok_or(get_fen_error(fen));
}
