use glium::implement_vertex;
use my_rust_matrix_lib::my_matrix_lib::prelude::{Field, IntoVecMath};

use crate::utils::types_util::{Arr3F32, QuatF32, Vec3};


#[derive(Debug,Clone, Copy)]
pub struct Vertex{
    position: Arr3F32,
    normal : Arr3F32,
    texture : Arr3F32,
}

implement_vertex!(Vertex, position, normal, texture);

impl Vertex {

    pub fn get_translated(&self,trans:Vec3)->Self{
        Self{
            position : (self.position.into_vec_math() + trans).into(),
            normal : self.normal,
            texture: self.texture,
        }
    }

    pub fn get_scaled(&self, scale:Vec3)->Self{
        let mut result = *self;
        result.position.iter_mut().zip(scale).for_each(|(a,b)|{
            *a = *a * b;
        });

        result
    }

    pub fn get_rotated(&self,rotation:QuatF32)->Self{

        let mut position :QuatF32 = (0.,self.position).into();
        position = rotation * position * rotation.f_mult_inverse();
        let (_,position) : (f32,Arr3F32)= position.into();
        Self{
            position,
            normal : self.normal,
            texture: self.texture
        }

    }


    #[inline]
    pub fn translate(&mut self, trans:Vec3){
        self.position = (self.position.into_vec_math() + trans).into();
    }

    #[inline]
    pub fn scale(&mut self, scale:Vec3){
        for i in 0..3{
            self.position[i] *= scale[i]
        }
    }

    #[inline]
    pub fn rotate(&mut self, rotation:QuatF32){
        (_,self.position)= <QuatF32 as Into<(f32, [f32; 3])>>::into(
            rotation * 
            <(f32, [f32; 3]) as Into<QuatF32>>::into((0.,self.position)) 
            * rotation.f_mult_inverse()
        );
    }

}