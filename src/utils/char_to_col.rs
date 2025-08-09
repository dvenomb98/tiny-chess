
/// indices: a=0, b=1, ..., h=7
const FILE_TO_COL: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

pub fn parse(file: char) -> Option<u8> {
    if file >= 'a' && file <= 'h' {
        let index = (file as usize) - ('a' as usize);
        Some(FILE_TO_COL[index])
    } else {
        None
    }
}
