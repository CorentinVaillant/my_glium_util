mod rotation;
pub use rotation::Rotation;

use crate::utils::types_util::Vec3;
pub trait SceneObject {
    fn translate(&mut self, trans: Vec3);
    fn set_position(&mut self, pos: Vec3);
    fn get_position(&self) -> Vec3;
    fn apply_position(&mut self);

    fn scale(&mut self, scale: Vec3);
    fn set_scale(&mut self, scale: Vec3);
    fn get_scale(&self) -> Vec3;
    fn apply_scale(&mut self);

    fn rotate(&mut self, rotation: Rotation);
    fn set_rotation(&mut self, rotation: Rotation);
    fn get_rotation(&self) -> Rotation;
    fn apply_rotation(&mut self);

    #[inline]
    fn apply_all_transforms(&mut self) {
        self.apply_position();
        self.apply_scale();
        self.apply_rotation();
    }
}

pub trait Renderable {
    type RenderError;

    fn render<F: glium::backend::Facade>(
        &self,
        facade: &F,
        program: &glium::Program,
        target: &mut glium::Frame,
        uniform: &glium::uniforms::UniformsStorage<
            impl glium::uniforms::AsUniformValue,
            impl glium::uniforms::Uniforms,
        >,
        draw_parameters: &glium::DrawParameters,
    ) -> Result<(), Self::RenderError>;
}
