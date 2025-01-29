use glium::uniforms::AsUniformValue;

use crate::{
    object_traits::{Rotation, Scale, SceneObject, Translation},
    utils::types_util::Mat4,
};

pub trait Camera {
    fn projection_matrix(&self) -> Mat4;
}

#[derive(Debug, Clone, Copy)]
pub struct OrthographicCam {
    right: f32,
    top: f32,
    far: f32,

    left: f32,
    bottom: f32,
    near: f32,

    position: Translation,
    scale: Scale,
    rotation: Rotation,
}

/* TODO
pub struct OrthographicCam {
    position :Translation,
    width : f32,
    height: f32,
    near : f32,
    far : f32,
} */

impl OrthographicCam {
    pub fn new(right: f32, top: f32, far: f32, left: f32, bottom: f32, near: f32) -> Self {
        Self {
            right,
            top,
            far,
            left,
            bottom,
            near,

            position: Translation::zero(),
            scale: Scale::zero(),
            rotation: Rotation::zero(),
        }
    }
}

impl Camera for OrthographicCam {
    fn projection_matrix(&self) -> Mat4 {
        let c = (
            -(self.right+self.left) / (self.right-self.left),
            -(self.top+self.bottom) / (self.top-self.bottom),
            -(self.far+self.near)   / (self.far-self.near),
        );
        let s = (
             2. / (self.right - self.left),
             2. / (self.top - self.bottom),
            -2. / (self.far - self.near),
        );
 
        self.rotation.to_mat4()
            * Mat4::from([
                [s.0, 0.0, 0.0, c.0],
                [0.0, s.1, 0.0, c.1],
                [0.0, 0.0, s.2, c.2],
                [0.0, 0.0, 0.0, 1.0],
            ])
    }
}

impl SceneObject for OrthographicCam {
    fn translate(&mut self, trans: Translation) {
        self.position += trans;
    }

    fn set_position(&mut self, pos: Translation) {
        self.position = pos;
    }



    fn scale(&mut self, scale: Scale) {
        self.scale += scale
    }

    fn set_scale(&mut self, scale: Scale) {
        self.scale = scale
    }



    fn rotate(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation += rotation
    }

    fn set_rotation(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation = rotation;
    }

}

impl AsUniformValue for OrthographicCam {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.projection_matrix().into())
    }
}
