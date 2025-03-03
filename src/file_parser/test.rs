#[cfg(test)]
mod test_parser_utils {

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

    mod test_line_type {

        use crate::file_parser::wavefront_parser::{line_type, WaveFrontLineType};

        #[test]
        fn test_line_type_empty() {
            assert_eq!(line_type(""), WaveFrontLineType::Empty);
        }

        #[test]
        fn test_line_type_comment() {
            assert_eq!(line_type("# This is a comment"), WaveFrontLineType::Comment);
        }

        #[test]
        fn test_line_type_geo_vert() {
            assert_eq!(line_type("v 1.0 2.0 3.0"), WaveFrontLineType::GeoVert);
        }

        #[test]
        fn test_line_type_texture_vert() {
            assert_eq!(line_type("vt 0.1 0.2"), WaveFrontLineType::TextureVert);
        }

        #[test]
        fn test_line_type_vert_norm() {
            assert_eq!(line_type("vn 1.0 0.0 0.0"), WaveFrontLineType::VertNorm);
        }

        #[test]
        fn test_line_type_param_space_vert() {
            assert_eq!(line_type("vp 1.0 2.0"), WaveFrontLineType::ParamSpaceVert);
        }

        #[test]
        fn test_line_type_face() {
            assert_eq!(line_type("f 1 2 3"), WaveFrontLineType::Face);
        }

        #[test]
        fn test_line_type_group_name() {
            assert_eq!(line_type("g my_group"), WaveFrontLineType::GroupName);
        }

        #[test]
        fn test_line_type_object_name() {
            assert_eq!(line_type("o my_object"), WaveFrontLineType::ObjectName);
        }

        #[test]
        fn test_line_type_unknown() {
            assert_eq!(line_type("xyz 1 2 3"), WaveFrontLineType::Unknown);
        }
    }

    mod test_parse_linetype {
        use crate::file_parser::wavefront_parser::parse_linetype;

        #[test]
        fn test_parse_linetype_valid() {
            let input = "l 1/2 3/4 5/6";
            let result = parse_linetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 3, 5]);
            assert_eq!(result.texture_vertex_indices, Some(vec![2, 4, 6]));
        }

        #[test]
        fn test_parse_linetype_missing_texture_indices() {
            let input = "l 1 2 3";
            let result = parse_linetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 2, 3]);
            assert_eq!(result.texture_vertex_indices, None);
        }

        #[test]
        fn test_parse_linetype_mixed_texture_indices() {
            let input = "l 1/2 3 4/5";
            let result = parse_linetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 3, 4]);
            assert_eq!(result.texture_vertex_indices, None);
        }

        #[test]
        fn test_parse_linetype_invalid_input() {
            let input = "l 1/a 2/3";
            let result = parse_linetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 2]);
            assert_eq!(result.texture_vertex_indices, None);
        }

        #[test]
        fn test_parse_linetype_empty_input() {
            let input = "l";
            let result = parse_linetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![]);
            assert_eq!(result.texture_vertex_indices, Some(vec![]));
        }
    }

    mod test_parse_facetype {
        use crate::file_parser::wavefront_parser::parse_facetype;

        #[test]
        fn test_parse_facetype_valid() {
            let input = "f 1/2/3 4/5/6 7/8/9";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 4, 7]);
            assert_eq!(result.texture_vertex_indices, Some(vec![2, 5, 8]));
            assert_eq!(result.normal_vertex_indices, Some(vec![3, 6, 9]));
        }

        #[test]
        fn test_parse_facetype_missing_texture_indices() {
            let input = "f 1//3 4//6 7//9";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 4, 7]);
            assert_eq!(result.texture_vertex_indices, None);
            assert_eq!(result.normal_vertex_indices, Some(vec![3, 6, 9]));
        }

        #[test]
        fn test_parse_facetype_missing_normal_indices() {
            let input = "f 1/2 4/5 7/8";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 4, 7]);
            assert_eq!(result.texture_vertex_indices, Some(vec![2, 5, 8]));
            assert_eq!(result.normal_vertex_indices, None);
        }

        #[test]
        fn test_parse_facetype_mixed_indices() {
            let input = "f 1/2/3 4//6 7/8";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 4, 7]);
            assert_eq!(result.texture_vertex_indices, None);
            assert_eq!(result.normal_vertex_indices, None);
        }

        #[test]
        fn test_parse_facetype_invalid_input() {
            let input = "f 1/a/3 4/5/6";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![1, 4]);
            assert_eq!(result.texture_vertex_indices, None);
            assert_eq!(result.normal_vertex_indices, Some(vec![3, 6]));
        }

        #[test]
        fn test_parse_facetype_empty_input() {
            let input = "f";
            let result = parse_facetype(input).unwrap();
            assert_eq!(result.vertex_indices, vec![]);
            assert_eq!(result.texture_vertex_indices, Some(vec![]));
            assert_eq!(result.normal_vertex_indices, Some(vec![]));
        }
    }

    mod test_add_group_name {
        use crate::file_parser::wavefront_parser::add_group_name;
        use crate::file_parser::wavefront_struct::{WavefrontGroup, WavefrontObj};

        #[test]
        fn test_add_group_name_valid() {
            let mut obj = WavefrontObj::empty();
            obj.geometric_vertices.push([1., 2., 3., 1.]);
            obj.geometric_vertices.push([1., 2., 3., 1.]);
            obj.geometric_vertices.push([1., 2., 3., 1.]);
            obj.geometric_vertices.push([1., 2., 3., 1.]);
            let input = "g my_group";
            assert!(add_group_name(input, &mut obj).is_ok());
            assert_eq!(obj.groups.len(), 1);
            assert_eq!(obj.groups[0].name, "my_group");
            assert_eq!(obj.groups[0].start_index, 1);
            assert_eq!(obj.groups[0].end_index, 4);
        }

        #[test]
        fn test_add_group_name_with_existing_group() {
            let mut obj = WavefrontObj::empty();
            let group = WavefrontGroup {
                name: "old_group".to_string(),
                start_index: 1,
                end_index: 1,
            };
            let geometric_vertex1 = [1., 2., 3., 4.];
            let geometric_vertex2 = [10., 20., 30., 40.];

            obj.groups.push(group);
            obj.geometric_vertices.push(geometric_vertex1);
            obj.geometric_vertices.push(geometric_vertex2);

            let input = "g new_group";
            assert!(add_group_name(input, &mut obj).is_ok());
            assert_eq!(obj.groups.len(), 2);
            assert_eq!(obj.groups[1].name, "new_group");
            assert_eq!(obj.groups[1].start_index, 2);
            assert_eq!(obj.groups[1].end_index, 2);
        }

        #[test]
        fn test_add_group_name_missing_name() {
            let mut obj = WavefrontObj::empty();
            let geometric_vertex = [1., 2., 3., 1.];
            obj.geometric_vertices.push(geometric_vertex);

            let input = "g";
            assert!(add_group_name(input, &mut obj).is_err());
        }

        #[test]
        fn test_add_group_name_invalid_prefix() {
            let mut obj = WavefrontObj::empty();
            let geometric_vertex = [1., 2., 3., 1.];
            obj.geometric_vertices.push(geometric_vertex);

            let input = "x my_group";
            assert!(add_group_name(input, &mut obj).is_err());
        }

        #[test]
        fn test_add_group_name_empty_input() {
            let mut obj = WavefrontObj::empty();
            let geometric_vertex = [1., 2., 3., 1.];
            obj.geometric_vertices.push(geometric_vertex);

            let input = "";
            assert!(add_group_name(input, &mut obj).is_err());
        }
    }

    mod test_add_name {
        use crate::file_parser::wavefront_parser::add_name;
        use crate::file_parser::wavefront_struct::WavefrontObj;

        #[test]
        fn test_add_name_valid() {
            let mut obj = WavefrontObj::empty();
            let input = "o my_object";
            assert!(add_name(input, &mut obj).is_ok());
            assert_eq!(obj.object_name, Some("my_object".to_string()));
        }

        #[test]
        fn test_add_name_multiple_names() {
            let mut obj = WavefrontObj::empty();
            obj.object_name = Some("existing_object".to_string());
            let input = "o new_object";
            assert!(add_name(input, &mut obj).is_err());
        }

        #[test]
        fn test_add_name_missing_name() {
            let mut obj = WavefrontObj::empty();
            let input = "o";
            assert!(add_name(input, &mut obj).is_err());
        }

        #[test]
        fn test_add_name_invalid_prefix() {
            let mut obj = WavefrontObj::empty();
            let input = "x my_object";
            assert!(add_name(input, &mut obj).is_err());
        }

        #[test]
        fn test_add_name_empty_input() {
            let mut obj = WavefrontObj::empty();
            let input = "";
            assert!(add_name(input, &mut obj).is_err());
        }
    }
}

#[cfg(test)]
mod test_wavefront_parsable {

    use crate::file_parser::{WavefrontObj, WavefrontParsable};
    use std::fs::{remove_file, File};
    use std::io::Write;

    #[test]
    fn test_read_from_obj_valid() {
        let path = "tests/temps/test_valid.obj";
        let mut file = File::create(path).unwrap();
        writeln!(file, "v 1.0 2.0 3.0").unwrap();
        writeln!(file, "o my_object").unwrap();
        drop(file);

        let obj = WavefrontObj::read_from_obj(path).unwrap();
        assert!(obj.object_name.is_some());
        assert_eq!(obj.object_name.unwrap(), "my_object");
        let _ = remove_file(path);
    }

    #[test]
    fn test_read_from_obj_multiline_continuation() {
        let path = "tests/temps/test_multiline.obj";
        let mut file = File::create(path).unwrap();
        writeln!(file, "v 1.0 2.0 \\").unwrap();
        writeln!(file, "4.0 1.0").unwrap();
        drop(file);

        let obj = WavefrontObj::read_from_obj(path).unwrap();
        assert_eq!(obj.geometric_vertices.len(), 1);
        assert_eq!(obj.geometric_vertices.get(0), Some(&[1., 2., 4., 1.]));
        let _ = remove_file(path);
    }

    #[test]
    fn test_read_from_obj_empty_file() {
        let path = "tests/temps/test_empty.obj";
        File::create(path).unwrap();

        let obj = WavefrontObj::read_from_obj(path).unwrap();
        assert!(obj.object_name.is_none());
        assert!(obj.geometric_vertices.is_empty());
        let _ = remove_file(path);
    }

    #[test]
    fn test_read_from_obj_invalid_file() {
        let path = "non_existent.obj";
        let result = WavefrontObj::read_from_obj(path);
        assert!(result.is_err());
    }
}
