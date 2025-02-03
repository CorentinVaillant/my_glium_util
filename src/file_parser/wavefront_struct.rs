#![allow(dead_code)]

pub type NotImpl = ();

pub(crate) use super::wavefront_parser::*;

/*------------------*\
|     Wavefront      |
\*------------------*/

#[derive(Debug, Clone)]
pub struct WavefrontObj {
    //vertex data
    pub(crate) geometric_vertices: Vec<[f32; 4]>,
    pub(crate) texture_vertices: Vec<[f32; 3]>,
    pub(crate) vertex_normals: Vec<[f32; 3]>,
    pub(crate) parameter_space_vertices: Vec<[f32; 3]>,

    //Free-form curve/surface attributes
    //not implemented

    //Elements
    pub(crate) points: Vec<Vec<i32>>,
    pub(crate) line: Vec<WavefrontLine>,
    pub(crate) face: Vec<WavefrontFace>,

    pub(crate) curv: NotImpl,    //not implemented
    pub(crate) curv2: NotImpl,   //not implemented
    pub(crate) surface: NotImpl, //not implemented

    //Free-form curve/surface body statements
    //not implemented

    //Grouping
    pub(crate) groups: Vec<WavefrontGroup>,
    pub(crate) smoothing_group: NotImpl,
    pub(crate) merging_group: NotImpl,
    pub(crate) object_name: Option<String>,

    //Display/render attributes
    //not implemented

    //Comments
    pub(crate) comments: Vec<String>,
}
#[derive(Debug, Clone)]
pub(crate) struct WavefrontLine {
    pub(crate) vertex_indices: Vec<i32>,
    pub(crate) texture_vertex_indices: Option<Vec<i32>>,
}

#[derive(Debug, Clone)]
pub(crate) struct WavefrontFace {
    pub(crate) vertex_indices: Vec<i32>,
    pub(crate) texture_vertex_indices: Option<Vec<i32>>,
    pub(crate) normal_vertex_indices: Option<Vec<i32>>,
}

#[derive(Debug, Clone)]
pub(crate) struct WavefrontGroup {
    pub(crate) name: String,
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
}

impl WavefrontObj {
    pub fn empty() -> Self {
        Self {
            geometric_vertices: vec![],
            texture_vertices: vec![],
            vertex_normals: vec![],
            parameter_space_vertices: vec![],
            points: vec![],
            line: vec![],
            face: vec![],
            curv: (),
            curv2: (),
            surface: (),
            groups: vec![],
            object_name: None,
            comments: vec![],
            smoothing_group: (),
            merging_group: (),
        }
    }
}
