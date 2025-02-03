#![cfg(test)]

mod test_parse_array_with_default {
    use crate::file_parser::wavefront_parser::parse_array_with_default;

    #[test]
    fn test_parse_array_with_valid_input_one() {
        let input = "v 0.005 0.1 1. 56.";
        let result = parse_array_with_default(input, 1.);
        assert_eq!(result, [0.005, 0.1, 1., 56.0]);
    }

    #[test]
    fn test_parse_array_with_valid_input() {
        let input = "1 2 3 4 5";
        let result: [i32; 5] = parse_array_with_default(input, 0);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_array_with_partial_input() {
        let input = "vn 10 20";
        let result: [i32; 5] = parse_array_with_default(input, -1);
        assert_eq!(result, [10, 20, -1, -1, -1]);
    }

    #[test]
    fn test_parse_array_with_empty_input() {
        let input = "v";
        let result: [i32; 3] = parse_array_with_default(input, 42);
        assert_eq!(result, [42, 42, 42]);
    }

    #[test]
    fn test_parse_array_with_non_numeric_values() {
        let input = "3 a 5";
        let result: [i32; 3] = parse_array_with_default(input, 0);
        assert_eq!(result, [3, 5, 0]);
    }

    #[test]
    fn test_parse_array_with_extra_values() {
        let input = "vt 1 2 3 4 5 6 7 8 9";
        let result: [i32; 5] = parse_array_with_default(input, 0);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }
}
mod test_parse_vec {
    use crate::file_parser::wavefront_parser::parse_vec;

    #[test]
    fn test_parse_vec_with_valid_input() {
        let input = "v 1 2 3 4 5";
        let result: Vec<i32> = parse_vec(input);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_vec_with_empty_input() {
        let input = "vt ";
        let result: Vec<i32> = parse_vec(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_vec_with_non_numeric_values() {
        let input = "t 10 a 20";
        let result: Vec<i32> = parse_vec(input);
        assert_eq!(result, vec![10, 20]);
    }

    #[test]
    fn test_parse_vec_with_mixed_spacing() {
        let input = "vt  3   4 5 ";
        let result: Vec<i32> = parse_vec(input);
        assert_eq!(result, vec![3, 4, 5]);
    }
}
