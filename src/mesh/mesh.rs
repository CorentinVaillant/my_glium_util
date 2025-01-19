use my_rust_matrix_lib::my_matrix_lib::prelude::VectorSpace;

use crate::object_traits::SceneObject;
use crate::utils::types_util::{QuatF32, Vec3};

use super::vertex::Vertex;

#[derive(Debug,Clone)]
pub struct Mesh{
    pub(crate) vertecies : Vec<Vertex>,

    pub(crate) position : Vec3,
    pub(crate) scale : Vec3,
    pub(crate) rotation:QuatF32,
}

impl SceneObject for Mesh {

    #[inline]
    fn translate(&mut self, trans : Vec3) {
        self.position += trans;
    }

    #[inline]
    fn set_position(&mut self, pos : Vec3) {
        self.position = pos;
    }

    #[inline]
    fn get_position(&self)->Vec3 {
        self.position
    }

    fn apply_position(&mut self) {
        for vert in self.vertecies.iter_mut(){
            vert.translate(self.position);
        }
        self.position = Vec3::v_space_zero();
    }

    #[inline]
    fn scale(&mut self, scale : Vec3) {
        for i in 0..3{
            self.scale[i] *= scale[i];
        }
    }

    #[inline]
    fn set_scale(&mut self, scale : Vec3) {
        self.scale = scale;
    }

    #[inline]
    fn get_scale(&self)->Vec3 {
        self.scale
    }

    fn apply_scale(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.scale(self.scale);
        }
    }
    
    #[inline]
    fn rotate(&mut self, rotation : QuatF32) {
        self.rotation =rotation * self.rotation;
    }
    
    #[inline]
    fn set_rotation(&mut self, rotation : QuatF32) {
        self.rotation = rotation;
    }
    
    #[inline]
    fn get_rotation(&self)->QuatF32 {
        self.rotation
    }

    fn apply_rotation(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.rotate(self.rotation);
        }
    }
}

impl<A : Into<Vec<Vertex>>>  From<A> for Mesh{
    fn from(value: A) -> Self {
        let vertecies = value.into();
        Self { vertecies, position: Vec3::v_space_zero(), scale: [1.,1.,1.].into(), rotation: QuatF32::zero() }
    }
}

impl Mesh {
    pub fn empty()->Self{
        vec![].into()
    }
}
