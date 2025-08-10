const FILE_TO_COL: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// Parse rank character ('1'-'8') to 0-based index (0-7)
pub fn rank_char_to_index(rank: char) -> Option<u8> {
    if rank >= '1' && rank <= '8' {
        Some((rank as u8) - ('1' as u8))
    } else {
        None
    }
}   

/// Parse file character ('a'-'h') to 0-based index (0-7)
pub fn file_char_to_index(file: char) -> Option<u8> {
    if file >= 'a' && file <= 'h' {
        let index = (file as usize) - ('a' as usize);
        Some(FILE_TO_COL[index])
    } else {
        None
    }
}

/// Convert 0-based index (0-7) to file character ('a'-'h')
pub fn index_to_file_char(col: u8) -> Option<char> {
    if col <= 7 {
        Some((col + (b'a')) as char)
    } else {
        None
    }
}

/// Parse index (0-7) to rank character ("1"-"8")
pub fn rank_index_to_char(rank: u8) -> Option<char> {
    if rank <= 7 {
        Some((rank + (b'1')) as char)
    } else {
        None
    }
}
