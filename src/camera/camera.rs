use my_rust_matrix_lib::my_matrix_lib::prelude::VectorSpace;

use crate::{object_traits::{Rotation, SceneObject}, utils::types_util::{Mat4, Vec3}};

pub trait Camera {
    fn projection_matrix(&self) -> Mat4;
}

pub struct OrthographicCam {
    right: f32,
    top: f32,
    far: f32,

    left: f32,
    bottom: f32,
    near: f32,

    position : Vec3,
    scale    : Vec3,
    rotation : Rotation,
}

impl OrthographicCam {
    pub fn new(right: f32, top: f32, far: f32, left: f32, bottom: f32, near: f32) -> Self {
        Self {
            right,
            top,
            far,
            left,
            bottom,
            near,

            position : Vec3::v_space_zero(),
            scale : [1.;3].into(),
            rotation : Rotation::zero(),
        }
    }
}

impl Camera for OrthographicCam{
    fn projection_matrix(&self) -> Mat4 {
        
        let c = ((self.near+self.left)/2.,(self.bottom+self.top)/2.,self.near);
        let s = (2./(self.right-self.left),2./(self.top-self.bottom),-2./(self.far-self.near));

        [
            [s.0, 0.0, 0.0,-c.0],
            [0.0, s.1, 0.0,-c.1],
            [0.0, 0.0, s.2,-c.2],
            [0.0, 0.0, 0.0, 1.0]
        ].into()
    }
}

impl SceneObject for OrthographicCam{
    fn translate(&mut self, trans: crate::utils::types_util::Vec3) {
        self.position += trans;
    }

    fn set_position(&mut self, pos: crate::utils::types_util::Vec3) {
        self.position = pos;

    }

    fn get_position(&self) -> crate::utils::types_util::Vec3 {
        self.position
    }

    fn apply_position(&mut self) {
        let c = ((self.near+self.left)/2.,(self.bottom+self.top)/2.,self.near);
        let [x,y,z] = self.position.into();

        self.right = x + c.0;
        self.left = x - c.0;

        self.far = y + c.0;
        self.near= y -c.0;

        self.top = z + c.0;
        self.bottom= z-c.0;
    }

    fn scale(&mut self, scale: crate::utils::types_util::Vec3) {
        todo!()
    }

    fn set_scale(&mut self, scale: crate::utils::types_util::Vec3) {
        todo!()
    }

    fn get_scale(&self) -> crate::utils::types_util::Vec3 {
        todo!()
    }

    fn apply_scale(&mut self) {
        todo!()
    }

    fn rotate(&mut self, rotation: crate::object_traits::Rotation) {
        todo!()
    }

    fn set_rotation(&mut self, rotation: crate::object_traits::Rotation) {
        todo!()
    }

    fn get_rotation(&self) -> crate::object_traits::Rotation {
        todo!()
    }

    fn apply_rotation(&mut self) {
        todo!()
    }
}