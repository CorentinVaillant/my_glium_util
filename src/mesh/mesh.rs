use std::fs::read_to_string;

use glium::{implement_vertex, VertexBuffer};
use my_rust_matrix_lib::my_matrix_lib::prelude::{Matrix, SquaredMatrixTrait, VectorMath};

use crate::math::*;

#[derive(Clone, Copy,Debug)]
pub struct Vertex{
    pub position : [f32;4],
}

implement_vertex!(Vertex,position);


impl Vertex {
    #[allow(dead_code)]
    fn new(x:f32,y:f32,z:f32,w:f32)->Self{
        Vertex{
            position :[x,y,z,w]
        }
    }

    #[inline]
    pub fn apply_transform(&mut self, trans_mat:Matrix<f32,4,4>){
        self.position_as_mut_vec_math().dot_assign(trans_mat);
    }

    #[inline]
    pub fn transform(&self, trans_mat:&Matrix<f32,4,4>)->Self{
        (*self.position_as_vec_math()* *trans_mat).into()
    }
    
    #[inline]#[allow(dead_code)]
    fn position_as_vec_math(&self)->&VectorMath<f32,4>{
        (&self.position).into()
    }
    
    #[inline]
    fn position_as_mut_vec_math(&mut self)->&mut VectorMath<f32,4>{
        (&mut self.position).into()
    }
}

impl From<[f32;3]> for Vertex {
    fn from(value: [f32;3]) -> Self {
        Vertex{
            position :[value[0],value[1],value[2],1.]
        }
    }
}

impl From<[f32;4]> for Vertex{
    fn from(value: [f32;4]) -> Self {
        Vertex{
            position :value
        }
    }
}

impl From<VectorMath<f32,3>> for Vertex {
    fn from(vec: VectorMath<f32,3>) -> Self {
        Vertex { position: [vec[0],vec[1],vec[2],1.] }
    }
}

impl From<VectorMath<f32,4>> for Vertex {
    fn from(vec: VectorMath<f32,4>) -> Self {
        Vertex { position: [vec[0],vec[1],vec[2],vec[3]] }
    }
}


impl From<Vec<f32>> for Vertex{
    fn from(value: Vec<f32>) -> Self {
        let mut it = value.into_iter();
        let mut position = [0.;4];
        for i in 0..4{
            position[i] = it.next().unwrap_or(1.);   
        }

        Vertex{
            position
        }
    }
}

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct Mesh{

    trans_mat : Matrix<f32,4,4>,

    vertecies :Vec<Vertex>,
    vertex_indices :Vec<usize>


}

impl From<Vec<Vertex>> for Mesh{
    fn from(vertecies: Vec<Vertex>) -> Self {
        Mesh{
            trans_mat : Matrix::identity(),

            vertecies,
            vertex_indices:vec![]
        }
    }
}


impl From<Vec<[f32;4]>> for Mesh{
    fn from(vec: Vec<[f32;4]>) -> Self {
        let mut result_vec = vec![];

        for tab in vec.into_iter(){
            result_vec.push(Vertex::from(tab));
        }

        Mesh::from(result_vec)

    }
}



impl Mesh {
    #[inline]
    pub fn empty_mesh()->Mesh {
        let vec: Vec<Vertex> = vec![];
        Self::from(vec)
    }

    #[inline]
    pub fn as_vertex_slice(&self)->&[Vertex]{
        &self.vertecies.as_slice()
    }

    #[inline]
    pub fn as_mut_vertex_slice(&mut self)->&mut [Vertex]{
        self.vertecies.as_mut_slice()
    }

    #[inline]
    pub fn scale(&mut self, scalar:f32){
        self.trans_mat *= scalar;
    }

    #[inline]
    pub fn load_into_vertex_buffer(&self,buffer :& VertexBuffer<Vertex>){
        let mut vec: Vec<Vertex> = Vec::with_capacity(self.vertecies.len());
        for vert in &self.vertecies {
            vec.push(vert.transform(&self.trans_mat));
        }

        
        buffer.write(&vec);
    }

    #[inline]
    pub fn transform_with_matrix(&mut self, trans_mat :Matrix<f32,4,4>){
        self.trans_mat *= trans_mat;
    }

    #[inline]
    pub fn rotate_x(&mut self,theta:f32){
        self.trans_mat *= x_rotation_mat(theta);
    }

    #[inline]
    pub fn rotate_y(&mut self,theta:f32){
        self.trans_mat *= y_rotation_mat(theta);
    }

    #[inline]
    pub fn rotate_z(&mut self,theta:f32){
        self.trans_mat *= z_rotation_mat(theta);
    }

    pub fn apply_transform(&mut self){
        for vert in self.vertecies.iter_mut(){
            vert.apply_transform(self.trans_mat);
        }
        self.trans_mat = Matrix::identity();
    }


}


impl Mesh{
    pub fn old_load_from_obj(path:&str)->Result<Mesh,std::io::Error>{
        let mut vertex_vec: Vec<[f32;4]> = vec![];


        let file = read_to_string(path)?;
        for line in file.lines(){
            match obj_parse_line_type(line) {

                ObjLineType::Vertex => {
                    let mut vertex_coord = [0.;4];
                    obj_parse_vertex(line, &mut vertex_coord);
                    vertex_vec.push(vertex_coord);

                },

                
                // ObjLineType::Comment=>println!("OBJ comment :{}",line),
                _=>()//TODO,
            };
        }

        
        Ok(Mesh::from(vertex_vec))
    }

    pub fn load_from_obj(path:&str)->Result<Mesh,std::io::Error>{//TODO

        let mut vertex_vec: Vec<[f32;4]> = vec![];


        let file = read_to_string(path)?;
        for line in file.lines(){
            match obj_parse_line_type(line) {

                ObjLineType::Vertex => {
                    let mut vertex_coord = [0.;4];
                    obj_parse_vertex(line, &mut vertex_coord);
                    vertex_vec.push(vertex_coord);

                },

                
                ObjLineType::Comment=>println!("OBJ comment :{}",line),
                _=>()//TODO,
            };
        }

        
        Ok(Mesh::from(vertex_vec))
    }

}

enum ObjLineType {
    Vertex,
    VertexNormal,
    TextureCoordinate,
    ParameterSpaceVertices,
    Line,
    Face,
    Comment,

    Empty,
    Unknow,
}

fn obj_parse_line_type(line:&str)->ObjLineType{
    let mut line = line.chars();

    match line.next(){
        Some(c) => match c {
            'v'=>match  line.next(){
                Some(c)=> match c {
                    ' '=> ObjLineType::Vertex,
                    't'=>ObjLineType::TextureCoordinate,
                    'n'=>ObjLineType::VertexNormal,
                    'p'=>ObjLineType::ParameterSpaceVertices,

                    _=>ObjLineType::Unknow
                }
                None => ObjLineType::Unknow
            },
            'f'=>ObjLineType::Face,
            'l'=>ObjLineType::Line,
            '#'=>ObjLineType::Comment,

            _=>ObjLineType::Unknow,
        },
        None => ObjLineType::Empty,
    }

}

fn obj_parse_vertex(line : &str,tab :&mut [f32;4]){
    let mut vec_float = parse_float(line).into_iter();
    tab.iter_mut().for_each(|x|{*x = vec_float.next().unwrap_or(1.)});
}

fn parse_float(line :&str)->Vec<f32>{
    let mut values = vec![];
    for val in line.split_whitespace(){
        //comment case
        if val.starts_with("#"){break;}

        //parse case
        match val.parse(){
            Ok(v)=> values.push(v),
            _=>()
        }
    }

    values
}