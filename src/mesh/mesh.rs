use glium::Surface;
use my_rust_matrix_lib::my_matrix_lib::prelude::VectorSpace;

use crate::object_traits::{Renderable, SceneObject};
use crate::utils::types_util::{QuatF32, Vec3};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) name : Option<String>,

    pub(crate) vertecies: Vec<Vertex>,
    pub(crate) indices : Option<Vec<u32>>, 
    pub(crate) _texture : Option<glium::texture::Texture2d>, //? idk

    pub(crate) position: Vec3,
    pub(crate) scale: Vec3,
    pub(crate) rotation: QuatF32,
}

impl SceneObject for Mesh {
    #[inline]
    fn translate(&mut self, trans: Vec3) {
        self.position += trans;
    }

    #[inline]
    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    #[inline]
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn apply_position(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.translate(self.position);
        }
        self.position = Vec3::v_space_zero();
    }

    #[inline]
    fn scale(&mut self, scale: Vec3) {
        for i in 0..3 {
            self.scale[i] *= scale[i];
        }
    }

    #[inline]
    fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    #[inline]
    fn get_scale(&self) -> Vec3 {
        self.scale
    }

    fn apply_scale(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.scale(self.scale);
        }
        self.scale = [1.;3].into();
    }

    #[inline]
    fn rotate(&mut self, rotation: QuatF32) {
        self.rotation = rotation * self.rotation;
    }

    #[inline]
    fn set_rotation(&mut self, rotation: QuatF32) {
        self.rotation = rotation;
    }

    #[inline]
    fn get_rotation(&self) -> QuatF32 {
        self.rotation
    }

    fn apply_rotation(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.rotate(self.rotation);
        }
        self.rotation = QuatF32::one();
    }
}

impl<A: Into<Vec<Vertex>>> From<A> for Mesh {
    fn from(value: A) -> Self {
        let vertecies = value.into();
        Self {
            name : None,
            vertecies,
            indices :None,
            _texture :None,

            position: Vec3::v_space_zero(),
            scale: [1., 1., 1.].into(),
            rotation: QuatF32::zero(),
        }
    }
}

impl Mesh {
    pub fn empty() -> Self {
        vec![].into()
    }

    pub fn from_verts_and_indices(vertecies : Vec<Vertex>, indices : Vec<u32>)->Self{
        Self { name:None,vertecies, indices: Some(indices) ,_texture: None, position: Vec3::v_space_zero(), scale: [1.;3].into(), rotation: QuatF32::zero() }
    }
}

impl Mesh {
    pub fn vertecies_number(&self)->usize{
        self.vertecies.len()
    }

    pub fn to_vertex_buffer<F : glium::backend::Facade>(&self,facade :&F)->Result<glium::vertex::VertexBuffer<Vertex>,glium::vertex::BufferCreationError>{
        
        let vertecies: Vec<Vertex> = self.vertecies.iter().map(|vert|{
            vert.get_transform(self.position, self.scale, self.rotation)
        }).collect();
        glium::vertex::VertexBuffer::new(facade, &vertecies)
    }

    pub fn to_index_buffer<F:glium::backend::Facade>(&self,facade :&F)->Result<glium::index::IndexBuffer<u32>, glium::index::BufferCreationError> {
        if let Some(ref indice) = self.indices{
            glium::index::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, indice)
        }else {
            glium::index::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &[])
        }
    }
}

impl Renderable for Mesh{
    type RenderError = glium::DrawError;

    fn render<F:glium::backend::Facade>(
        &self,facade :&F, 
        program:&glium::Program, 
        target :&mut glium::Frame,  
        uniforms:&glium::uniforms::UniformsStorage<impl glium::uniforms::AsUniformValue, impl glium::uniforms::Uniforms>,
        draw_parameters: &glium::DrawParameters,
    )->Result<(),Self::RenderError> {
        let vertex_buffer = self.to_vertex_buffer(facade).unwrap(); // !Scotch

        let index_buffer = self.to_index_buffer(facade).unwrap(); // !Scotch

        target.draw(&vertex_buffer, &index_buffer, program, uniforms, draw_parameters)
    }
}