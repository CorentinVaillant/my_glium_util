#[cfg(test)]


pub(super) mod parse_array_with_default_test{
    use crate::file_parser::wavefront_parser::parse_array_with_default;

    #[test]
    fn test_parse_array_with_valid_input_one(){
        let input = "v 0.005 0.1 1. 56.";
        let result = parse_array_with_default(input, 1.);
        assert_eq!(result,[0.005, 0.1, 1., 56.0]);  
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
