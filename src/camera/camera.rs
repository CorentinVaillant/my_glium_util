use glium::uniforms::AsUniformValue;

use crate::{
    object_traits::{Rotation, Scale, SceneObject, Translation},
    utils::types_util::Mat4,
};

pub trait Camera {
    fn projection_matrix(&self) -> Mat4;

    fn view_matrix(&self) -> Mat4;

    fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

#[derive(Debug, Clone, Copy)]

pub struct OrthographicCam {
    position: Translation,
    width: f32,
    height: f32,
    near: f32,
    far: f32,

    scale: Scale,
    rotation: Rotation,
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
        }
    }
}

impl Camera for OrthographicCam {
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

        // Create a translation matrix based on the camera's position
        let translation = Mat4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-x, -y, -z, 1.0],
        ]);

        // Apply the camera's rotation
        self.rotation.to_mat4() * translation
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
        glium::uniforms::UniformValue::Mat4(self.view_projection_matrix().into())
    }
}
