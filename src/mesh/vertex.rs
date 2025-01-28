use glium::implement_vertex;
use my_rust_matrix_lib::my_matrix_lib::prelude::{Field, IntoVecMath};

use crate::{object_traits::{Scale, Translation}, utils::types_util::{Arr3F32, Arr4F32, QuatF32, Vec3}};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub(crate) position: Arr4F32,
    pub(crate) normal: Arr3F32,
    pub(crate) texture: Arr3F32,
}

implement_vertex!(Vertex, position, normal, texture);

impl From<Arr4F32> for Vertex {
    fn from(value: Arr4F32) -> Self {
        Vertex {
            position: value,
            normal: [0.; 3],
            texture: [0.; 3],
        }
    }
}

impl Vertex {
    pub fn get_translated(&self, trans: Translation) -> Self {
        Self {
            position: (self.position.into_vec_math() + trans.into()).into(),
            normal: self.normal,
            texture: self.texture,
        }
    }

    pub fn get_scaled(&self, scale: Scale) -> Self {
        let mut result = *self;
        result.position.iter_mut().zip(<Scale as Into<Vec3>>::into(scale)).for_each(|(a, b)| {
            *a = *a * b;
        });

        result
    }

    pub fn get_rotated(&self, rotation: QuatF32) -> Self {
        let mut position: QuatF32 = (0., self.position[1],self.position[2],self.position[3]).into();
        position = rotation * position * rotation.f_mult_inverse();
        let (_, position): (f32, Arr3F32) = position.into();
        Self {
            position:[position[0],position[1],position[2],self.position[3]],
            normal: self.normal,
            texture: self.texture,
        }
    }

    pub fn get_transform(&self, trans: Translation, scale: Scale, rotation: QuatF32) -> Self {
        let mut transformed = self.get_translated(trans);
        transformed.scale(scale);
        transformed.rotate(rotation);

        transformed
    }

    #[inline]
    pub fn translate(&mut self, trans: Translation) {
        self.position = (self.position.into_vec_math() + trans.into()).into();
    }

    #[inline]
    pub fn scale(&mut self, scale: Scale) {
        for i in 0..3 {
            self.position[i] *= <Scale as Into<Vec3>>::into(scale)[i];
        }
    }

    #[inline]
    pub fn rotate(&mut self, rotation: QuatF32) {

        let (_, vec): (f64, Vec3) = rotation.into();
        let rotation: QuatF32 = (1., vec).into();
        (_, self.position[0],self.position[3],self.position[2],) = <QuatF32 as Into<(f32,f32,f32,f32)>>::into(
            rotation
                * <(f32,f32,f32,f32) as Into<QuatF32>>::into((0., self.position[0],self.position[1],self.position[2]))
                * rotation.f_mult_inverse(),
        );
    }
}
