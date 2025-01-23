//TODO obj parse !

use std::{fs::read_to_string, path::Path};

use crate::mesh::vertex::Vertex;

use super::mesh::Mesh;
use crate::utils::macro_util::debug_println;

pub trait WaveFrontParsable {
    fn load_from_wavefront<P: AsRef<Path>>(path: P) -> Result<Self, WavefrontError>
    where
        Self: Sized;
}
#[derive(Debug)]
pub enum WavefrontError {
    CouldNotReadFile,
    InvalidFaceData,
}

struct FaceDataTripelet {
    geo_vert_index: u32,
    _texture_vertex_index: Option<u32>,
    _vertex_normal_index: u32,
}

type FaceData = Vec<FaceDataTripelet>;

impl WaveFrontParsable for Mesh {
    /* Note
     *   -> Obj index start with 1.
     */
    fn load_from_wavefront<P: AsRef<Path>>(path: P) -> Result<Self, WavefrontError> {
        let mut name = None;
        let mut geo_vert_data = vec![];
        let mut texture_vert_data = vec![];
        let mut normal_vert_data = vec![];
        //let mut _parameter_space_vertex = vec![];
        let mut faces_data = vec![];

        for line in read_to_string(path)
            .map_err(|_| WavefrontError::CouldNotReadFile)?
            .lines()
        {
            match line_type(line) {
                WaveFrontLineType::GeoVertex => geo_vert_data.push(parse_tree_float(line, 0.)),
                WaveFrontLineType::TextureVertex => {
                    texture_vert_data.push(parse_tree_float(line, 0.))
                }
                WaveFrontLineType::VertexNormal => {
                    normal_vert_data.push(parse_tree_float(line, 0.))
                }

                WaveFrontLineType::Face => faces_data.push(parse_face(line)?),

                WaveFrontLineType::Name => name = Some(line[2..].to_string()),

                WaveFrontLineType::Comment => debug_println!("Fond comment :{}", line),
                WaveFrontLineType::Empty => (),
                WaveFrontLineType::Unknow => {
                    debug_println!("Unknow or not yet implemented line :{}", line)
                }
                other => debug_println!("{:#?} not implemented yet", other),
            }
        }

        let vertecies: Vec<Vertex> = geo_vert_data
            .into_iter()
            .enumerate()
            .map(|(i, pos)| Vertex {
                position: pos,
                normal: *normal_vert_data.get(i).unwrap_or(&[0.; 3]),
                texture: *texture_vert_data.get(i).unwrap_or(&[0.; 3]),
            })
            .collect();

        let mut indices = vec![];
        for face in faces_data {
            for tripelet in face {
                indices.push(tripelet.geo_vert_index - 1);
            }
        }

        let mut result = Mesh::from_verts_and_indices(vertecies, indices);

        result.name = name;
        Ok(result)
    }
}

// fn triangulate_face(indices :&[u32])->Vec<[u32;3]>{
//     let mut triangle = Vec::new();
//     for i in 1..(indices.len() -1){
//         triangle.push([indices[0],indices[i],indices[i+1]]);
//     }
//     triangle
// }

fn line_type(line: &str) -> WaveFrontLineType {
    if line.len() < 2 {
        WaveFrontLineType::Empty
    } else {
        match &line[0..2] {
            "v " => WaveFrontLineType::GeoVertex,
            "vt" => WaveFrontLineType::TextureVertex,
            "vn" => WaveFrontLineType::VertexNormal,
            "vp" => WaveFrontLineType::ParameterSpaceVert,

            "p " => WaveFrontLineType::Point,
            "l " => WaveFrontLineType::Line,
            "f " => WaveFrontLineType::Face,

            "o " => WaveFrontLineType::Name,
            "g " => WaveFrontLineType::GroupeName,
            "s " => WaveFrontLineType::SmothingGroup,
            "mg" => WaveFrontLineType::MergingGroupe,
            _ => match &line[0..1] {
                "#" => WaveFrontLineType::Comment,
                _ => WaveFrontLineType::Unknow,
            },
        }
    }
}

#[derive(Debug)]
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

    //grouping
    Name,          //o
    GroupeName,    //g
    SmothingGroup, //s
    MergingGroupe, //mg

    Comment,
    Empty,
    Unknow,
}

fn parse_tree_float(line: &str, default: f32) -> [f32; 3] {
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

use core::str::FromStr;

fn parse_face(line: &str) -> Result<FaceData, WavefrontError> {
    //remove all the comments on the line
    let line = match line.split_once('#') {
        Some((before_comment, _)) => before_comment.trim(),
        None => line.trim(),
    };

    //separate each element to be treat
    let split: Vec<&str> = line.split(' ').collect();

    //split(0) == 'f'
    if split.get(0).is_none_or(|c| *c != "f") {
        debug_println!("face line \"{}\" is invalide", line);
        return Err(WavefrontError::InvalidFaceData);
    }

    let mut face_data = Vec::new();

    //treating every triplet
    for str_triplet in &split[1..] {
        let mut indices = str_triplet.split('/');
        let geo_vert_index = indices
            .next()
            .and_then(|v| u32::from_str(v).ok())
            .ok_or(WavefrontError::InvalidFaceData)?;

        let texture_vertex_index = if let Some(str) = indices.next() {
            str.parse().ok()
        } else {
            None
        };

        let vertex_normal_index = indices
            .next()
            .and_then(|v| u32::from_str(v).ok())
            .ok_or(WavefrontError::InvalidFaceData)?;

        face_data.push(FaceDataTripelet {
            geo_vert_index,
            _texture_vertex_index: texture_vertex_index,
            _vertex_normal_index: vertex_normal_index,
        });
    }

    Ok(face_data)
}
