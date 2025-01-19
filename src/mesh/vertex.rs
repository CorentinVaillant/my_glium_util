use glium::implement_vertex;
use my_rust_matrix_lib::my_matrix_lib::prelude::{Field, IntoVecMath};

use crate::math::type_util::{Arr3F32, QuatF32, Vec3};



#[derive(Debug,Clone, Copy)]
pub struct Vertex{
    position: Arr3F32,
    normal : Arr3F32,
    texture : Arr3F32,
}

implement_vertex!(Vertex, position, normal, texture);

impl Vertex {
    pub fn rotate(&self,rotation:QuatF32)->Self{

        let mut position :QuatF32 = (0.,self.position).into();
        position = rotation * position * rotation.f_mult_inverse();
        let (_,position) : (f32,Arr3F32)= position.into();
        Self{
            position,
            normal : self.normal,
            texture: self.texture
        }

    }

    pub fn translate(&self,trans:Vec3)->Self{
        Self{
            position : (self.position.into_vec_math() + trans).into(),
            normal : self.normal,
            texture: self.texture,
        }
    }

    pub fn scale(&self, scale:Vec3)->Self{
        let mut result = *self;
        result.position.iter_mut().zip(scale).for_each(|(a,b)|{
            *a = *a * b;
        });

        result
    }
}