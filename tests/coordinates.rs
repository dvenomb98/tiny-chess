use tiny_chess::utils;

mod utils_coordinates_tests {
    use super::*;

    #[test]
    fn test_rank_char_to_index() {
        for (expected_idx, rank_char) in "12345678".chars().enumerate() {
            assert_eq!(
                utils::coordinates::rank_char_to_index(rank_char),
                Some(expected_idx as u8),
                "Failed for rank '{}'", rank_char
            );
        }
        
        for invalid_char in ['0', '9', 'a', 'z', ' ', '\n'] {
            assert_eq!(
                utils::coordinates::rank_char_to_index(invalid_char),
                None,
                "Should return None for invalid char '{}'", invalid_char
            );
        }
    }

    #[test]
    fn test_file_char_to_index() {
        for (expected_idx, file_char) in "abcdefgh".chars().enumerate() {
            assert_eq!(
                utils::coordinates::file_char_to_index(file_char),
                Some(expected_idx as u8),
                "Failed for file '{}'", file_char
            );
        }
        
        for invalid_char in ['A', 'Z', '1', '8', ' ', 'i', 'z'] {
            assert_eq!(
                utils::coordinates::file_char_to_index(invalid_char),
                None,
                "Should return None for invalid char '{}'", invalid_char
            );
        }
    }

    #[test]
    fn test_index_to_file_char() {
        for (idx, expected_char) in "abcdefgh".chars().enumerate() {
            assert_eq!(
                utils::coordinates::index_to_file_char(idx as u8),
                Some(expected_char),
                "Failed for index {}", idx
            );
        }
        
        for invalid_idx in [8, 9, 255] {
            assert_eq!(
                utils::coordinates::index_to_file_char(invalid_idx),
                None,
                "Should return None for invalid index {}", invalid_idx
            );
        }
    }

    #[test]
    fn test_rank_index_to_char() {
        for (idx, expected_char) in "12345678".chars().enumerate() {
            assert_eq!(
                utils::coordinates::rank_index_to_char(idx as u8),
                Some(expected_char),
                "Failed for index {}", idx
            );
        }
        
        for invalid_idx in [8, 9, 255] {
            assert_eq!(
                utils::coordinates::rank_index_to_char(invalid_idx),
                None,
                "Should return None for invalid index {}", invalid_idx
            );
        }
    }

    #[test]
    fn test_coordinate_conversion_roundtrip() {
        for rank_char in "12345678".chars() {
            if let Some(idx) = utils::coordinates::rank_char_to_index(rank_char) {
                assert_eq!(
                    utils::coordinates::rank_index_to_char(idx),
                    Some(rank_char),
                    "Roundtrip failed for rank '{}'", rank_char
                );
            }
        }
        
        for file_char in "abcdefgh".chars() {
            if let Some(idx) = utils::coordinates::file_char_to_index(file_char) {
                assert_eq!(
                    utils::coordinates::index_to_file_char(idx),
                    Some(file_char),
                    "Roundtrip failed for file '{}'", file_char
                );
            }
        }
    }

    #[test]
    fn test_coordinate_boundaries() {
        assert_eq!(utils::coordinates::rank_char_to_index('1'), Some(0));
        assert_eq!(utils::coordinates::rank_char_to_index('8'), Some(7));
        assert_eq!(utils::coordinates::file_char_to_index('a'), Some(0));
        assert_eq!(utils::coordinates::file_char_to_index('h'), Some(7));
        
        assert_eq!(utils::coordinates::rank_index_to_char(0), Some('1'));
        assert_eq!(utils::coordinates::rank_index_to_char(7), Some('8'));
        assert_eq!(utils::coordinates::index_to_file_char(0), Some('a'));
        assert_eq!(utils::coordinates::index_to_file_char(7), Some('h'));
    }
}
