/// Parse rank character ('1'-'8') to 0-based index (0-7)
pub fn parse(rank: char) -> Option<u8> {
    if rank >= '1' && rank <= '8' {
        Some((rank as u8) - ('1' as u8))
    } else {
        None
    }
}