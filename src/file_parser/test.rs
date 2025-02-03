#[cfg(test)]

#[test]
pub(super) fn wavefront_parser_test(){
    use crate::file_parser::wavefront_parser::parse_array_with_default;

    let input = "v 0.005 0.1 1. 56.";
    let result = parse_array_with_default(input, 1.);
    assert_eq!(result,[0.005, 0.1, 1., 56.0]);


} 