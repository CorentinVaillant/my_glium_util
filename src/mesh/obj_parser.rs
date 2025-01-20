//TODO obj parse !

use std::{fs::read_to_string, path::Path};


use crate::utils::macro_util::debug_println;

use super::mesh::Mesh;

impl Mesh {
    pub fn load_from_wavefront<P: AsRef<Path>>(path: P) -> Result<Self, WavefrontError> {
        

        let mut geo_vert_data = vec![];
        let mut texture_vert_data = vec![];
        let mut vertex_normal_data = vec![];
        let mut parameter_space_vertex = vec![];

        for line in read_to_string(path)
            .map_err(|_| WavefrontError::CouldNotReadFile)?
            .lines()
        {
            match line_type(line) {
                WaveFrontLineType::GeoVertex            => geo_vert_data.push(parse_tree_float(line,0.,)),
                WaveFrontLineType::TextureVertex        => texture_vert_data.push(parse_tree_float(line,0.)),
                WaveFrontLineType::VertexNormal         => vertex_normal_data.push(parse_tree_float(line,0.)),
                WaveFrontLineType::ParameterSpaceVert   => parameter_space_vertex.push(parse_tree_float(line,1.)),
                WaveFrontLineType::Point                => debug_println!("Point not implemented yet"),
                WaveFrontLineType::Line                 => debug_println!("Line not implemented yet"),
                WaveFrontLineType::Face => todo!(),
                WaveFrontLineType::Comment              => debug_println!("Fond comment :{}",line),
                WaveFrontLineType::Empty                => (),    
                WaveFrontLineType::Unknow               => debug_println!("Unknow or not yet implemented line :{}",line),
            }
        }

        let vertecies = vec![];
        Ok(Mesh::from(vertecies))
    }
}

#[derive(Debug)]
pub enum WavefrontError {
    CouldNotReadFile,
}

fn line_type(line: &str) -> WaveFrontLineType {
    let mut split = line.split(" ");
    match split.next() {
        None => WaveFrontLineType::Empty,
        Some(s) => {
            let mut chars = s.chars();
            match chars.next() {
                Some('v') => match chars.next() {
                    Some('t') => WaveFrontLineType::TextureVertex,
                    Some('n') => WaveFrontLineType::VertexNormal,
                    Some('p') => WaveFrontLineType::ParameterSpaceVert,
                    Some(_) => WaveFrontLineType::GeoVertex,
                    None => WaveFrontLineType::Unknow,
                },
                Some('p') => WaveFrontLineType::Point,
                Some('l') => WaveFrontLineType::Line,
                Some('f') => WaveFrontLineType::Face,
                Some('#') => WaveFrontLineType::Comment,

                _ => WaveFrontLineType::Unknow,
            }
        }
    }
}

enum WaveFrontLineType {
    //Vertex data
    GeoVertex,          //v
    TextureVertex,      //vt
    VertexNormal,       //vn
    ParameterSpaceVert, //vp

    //Element
    Point, //p
    Line,  //l
    Face,  //f

    Comment,
    Empty,
    Unknow,
}

fn parse_tree_float(line: &str,default:f32) -> [f32; 3] {
    let mut result = [default; 3];
    let mut indice = 0;
    for split in line.split(" ") {
        if indice > 3 {
            return result;
        }
        match split.parse() {
            Ok(val) => {
                result[indice] = val;
                indice += 1;
            }
            _ => (),
        };
    }

    result
}