#[cfg(test)]
mod test_square {
    use crate::square;



    #[test]
    fn test_from_chess_notation() {
        // Test valid squares
        assert_eq!(square::Square::from_chess_notation("a1"), Some(square::Square::new(7, 0))); // bottom-left
        assert_eq!(square::Square::from_chess_notation("h8"), Some(square::Square::new(0, 7))); // top-right
        assert_eq!(square::Square::from_chess_notation("e4"), Some(square::Square::new(4, 4))); // middle
        assert_eq!(square::Square::from_chess_notation("d2"), Some(square::Square::new(6, 3))); // near bottom

        // Test invalid notation
        assert_eq!(square::Square::from_chess_notation(""), None);
        assert_eq!(square::Square::from_chess_notation("a"), None);
        assert_eq!(square::Square::from_chess_notation("abc"), None);
        assert_eq!(square::Square::from_chess_notation("i1"), None); // Invalid file
        assert_eq!(square::Square::from_chess_notation("a9"), None); // Invalid rank
        assert_eq!(square::Square::from_chess_notation("z0"), None); // Both invalid
    }

    #[test]
    fn test_to_chess_notation() {
        // Test valid squares
        assert_eq!(square::Square::new(7, 0).to_chess_notation(), Some("a1".to_string())); // bottom-left
        assert_eq!(square::Square::new(0, 7).to_chess_notation(), Some("h8".to_string())); // top-right
        assert_eq!(square::Square::new(4, 4).to_chess_notation(), Some("e4".to_string())); // middle
        assert_eq!(square::Square::new(6, 3).to_chess_notation(), Some("d2".to_string())); // near bottom

        // Test invalid squares (out of bounds)
        assert_eq!(square::Square::new(8, 0).to_chess_notation(), None);
        assert_eq!(square::Square::new(0, 8).to_chess_notation(), None);
        assert_eq!(square::Square::new(9, 9).to_chess_notation(), None);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let test_cases = [
            "a1", "a8", "h1", "h8", "e4", "d2", "f7", "g3"
        ];

        for notation in test_cases {
            let square = square::Square::from_chess_notation(notation).unwrap();
            let back_to_notation = square.to_chess_notation().unwrap();
            assert_eq!(notation, back_to_notation);
        }
    }
}
