#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

mod rotation;
pub use rotation::Rotation;
pub use scale::Scale;
pub use translation::Translation;

mod scale;
mod translation;

pub trait SceneObject {
    fn translate(&mut self, trans: Translation);
    fn set_position(&mut self, pos: Translation);

    fn scale(&mut self, scale: Scale);
    fn set_scale(&mut self, scale: Scale);

    fn rotate(&mut self, rotation: Rotation);
    fn set_rotation(&mut self, rotation: Rotation);
}

pub trait GetableSceneObject
where Self:SceneObject
{
    fn get_position(&self) -> Translation;
    fn get_scale(&self) -> Scale;
    fn get_rotation(&self) -> Rotation;
}

pub trait ApplicableSceneObject
where Self : SceneObject
{
    fn apply_position(&mut self);
    fn apply_scale(&mut self);
    fn apply_rotation(&mut self);
    #[inline]
    fn apply_all_transforms(&mut self) {
        self.apply_rotation();
        self.apply_scale();
        self.apply_position();
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
