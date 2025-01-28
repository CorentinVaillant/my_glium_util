
use glium::uniforms::AsUniformValue;

use crate::{object_traits::{Rotation,Scale,Translation, SceneObject}, utils::types_util::Mat4};

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

    position : Translation,
    scale    : Scale,
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

            position : Translation::zero(),
            scale : Scale::zero(),
            rotation : Rotation::zero(),
        }
    }
}

impl Camera for OrthographicCam{
    fn projection_matrix(&self) -> Mat4 {
        
        let c = ((self.near+self.left)/2.,(self.bottom+self.top)/2.,self.near);
        let s = (2./(self.right-self.left),2./(self.top-self.bottom),-2./(self.far-self.near));

        self.rotation.to_mat4() *  
        Mat4::from([
            [s.0, 0.0, 0.0,-c.0],
            [0.0, s.1, 0.0,-c.1],
            [0.0, 0.0, s.2,-c.2],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }
}

impl SceneObject for OrthographicCam{
    fn translate(&mut self, trans: Translation) {
        self.position += trans;
    }

    fn set_position(&mut self, pos: Translation) {
        self.position = pos;

    }

    fn get_position(&self) -> Translation {
        self.position
    }

    fn apply_position(&mut self) {
        let c = ((self.near+self.left)/2.,(self.bottom+self.top)/2.,self.near);
        let [x,y,z]:[f32;3] = self.position.into();

        self.right = x + c.0;
        self.left = x - c.0;

        self.far = y + c.0;
        self.near= y -c.0;

        self.top = z + c.0;
        self.bottom= z-c.0;
    }

    fn scale(&mut self, scale: Scale) {
        self.scale += scale
    }

    fn set_scale(&mut self, scale: Scale) {
        self.scale = scale
    }

    fn get_scale(&self) -> Scale {
        self.scale
    }

    fn apply_scale(&mut self) {
        self.right *= self.scale['x'];
        self.left  *= self.scale['x'];

        self.far *= self.scale['y'];
        self.near*= self.scale['y'];

        self.top    *= self.scale['z'];
        self.bottom *= self.scale['z'];

        self.scale = Scale::zero();
    }

    fn rotate(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation += rotation
    }

    fn set_rotation(&mut self, rotation: crate::object_traits::Rotation) {
        self.rotation = rotation;
    }

    fn get_rotation(&self) -> crate::object_traits::Rotation {
        self.rotation
    }

    fn apply_rotation(&mut self) {
        ()
    }
}

impl AsUniformValue for OrthographicCam{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.projection_matrix().into())
    }
}