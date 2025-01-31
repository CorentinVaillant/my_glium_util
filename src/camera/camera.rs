use std::cell::RefCell;
use glium::uniforms::AsUniformValue;

use crate::{
    object_traits::{Rotation, Scale, SceneObject, Translation},
    utils::types_util::Mat4,
};

pub trait Camera {
    fn zoom(&mut self,factor:f32);

    fn projection_matrix(&self) -> Mat4;

    fn view_matrix(&self) -> Mat4;

    fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

#[derive(Debug, Clone)]
pub struct OrthographicCam {
    position: Translation,
    width: f32,
    height: f32,
    near: f32,
    far: f32,
    scale: Scale,
    rotation: Rotation,
    view_proj_mat: RefCell<Option<Mat4>>,
}

impl OrthographicCam {
    pub fn new(position: Translation, width: f32, height: f32, near: f32, far: f32) -> Self {
        Self {
            position,
            width,
            height,
            near,
            far,
            scale: Scale::zero(),
            rotation: Rotation::zero(),
            view_proj_mat: RefCell::new(None),
        }
    }

    fn invalidate_cache(&self) {
        self.view_proj_mat.replace(None);
    }
}

impl Camera for OrthographicCam {
    fn zoom(&mut self,factor:f32) {
        self.scale *= factor;
        self.invalidate_cache();
    }

    fn projection_matrix(&self) -> Mat4 {
        let right = self.scale.x() * self.width / 2.0;
        let left = -right;
        let top = self.scale.z() * self.height / 2.0;
        let bottom = -top;

        let c = (
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            -(self.far + self.near) / (self.far - self.near),
        );
        let s = (
            2. / (right - left),
            2. / (top - bottom),
            -2. / (self.far - self.near),
        );

        Mat4::from([
            [s.0, 0.0, 0.0, 0.0],
            [0.0, s.1, 0.0, 0.0],
            [0.0, 0.0, s.2, 0.0],
            [c.0, c.1, c.2, 1.0],
        ])
    }

    fn view_matrix(&self) -> Mat4 {
        let [x, y, z] = self.position.into();

        let translation = Mat4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-x, -y, -z, 1.0],
        ]);

        translation * self.rotation.to_mat4()
    }

    fn view_projection_matrix(&self) -> Mat4 {
        if let Some(mat) = *self.view_proj_mat.borrow() {
            return mat;
        }
        let mat = self.projection_matrix() * self.view_matrix();
        self.view_proj_mat.replace(Some(mat));
        mat
    }
}

impl SceneObject for OrthographicCam {
    fn translate(&mut self, trans: Translation) {
        self.position += trans;
        self.invalidate_cache();
    }

    fn set_position(&mut self, pos: Translation) {
        self.position = pos;
        self.invalidate_cache();
    }

    fn scale(&mut self, scale: Scale) {
        self.scale += scale;
        self.invalidate_cache();
    }

    fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
        self.invalidate_cache();
    }

    fn rotate(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation += rotation;
        self.invalidate_cache();
    }

    fn set_rotation(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation = rotation;
        self.invalidate_cache();
    }
}

impl AsUniformValue for OrthographicCam {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.view_projection_matrix().into())
    }
}

impl AsUniformValue for &OrthographicCam {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.view_projection_matrix().into())
    }
}