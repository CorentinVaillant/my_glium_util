use glium::Surface;

use crate::object_traits::{Renderable, Rotation, Scale, SceneObject, Translation};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub(crate) name: Option<String>,

    pub(crate) vertecies: Vec<Vertex>,
    pub(crate) indices: Option<Vec<u32>>,
    pub(crate) _texture: Option<glium::texture::Texture2d>, //? idk

    pub(crate) position: Translation,
    pub(crate) scale: Scale,
    pub(crate) rotation: Rotation,
}

impl SceneObject for Mesh {
    #[inline]
    fn translate(&mut self, trans: Translation) {
        self.position += trans;
    }

    #[inline]
    fn set_position(&mut self, pos: Translation) {
        self.position = pos;
    }

    #[inline]
    fn get_position(&self) -> Translation {
        self.position
    }

    fn apply_position(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.translate(self.position);
        }
        self.position = Translation::zero();
    }

    #[inline]
    fn scale(&mut self, scale: Scale) {
        self.scale += scale;
    }

    #[inline]
    fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    #[inline]
    fn get_scale(&self) -> Scale {
        self.scale
    }

    fn apply_scale(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.scale(self.scale);
        }
        self.scale = [1.; 3].into();
    }

    #[inline]
    fn rotate(&mut self, rotation: Rotation) {
        self.rotation += rotation;
    }

    #[inline]
    fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    #[inline]
    fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    fn apply_rotation(&mut self) {
        for vert in self.vertecies.iter_mut() {
            vert.rotate(self.rotation.into());
        }
        self.rotation = Rotation::zero();
    }
}

impl<A: Into<Vec<Vertex>>> From<A> for Mesh {
    fn from(value: A) -> Self {
        let vertecies = value.into();
        Self {
            name: None,
            vertecies,
            indices: None,
            _texture: None,

            position: Translation::zero(),
            scale: Scale::zero(),
            rotation: Rotation::zero(),
        }
    }
}

impl Mesh {
    pub fn empty() -> Self {
        vec![].into()
    }

    pub fn from_verts_and_indices(vertecies: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            name: None,
            vertecies,
            indices: Some(indices),
            _texture: None,
            position: Translation::zero(),
            scale: Scale::zero(),
            rotation: Rotation::zero(),
        }
    }
}

impl Mesh {
    pub fn vertecies_number(&self) -> usize {
        self.vertecies.len()
    }

    pub fn to_vertex_buffer<F: glium::backend::Facade>(
        &self,
        facade: &F,
    ) -> Result<glium::vertex::VertexBuffer<Vertex>, glium::vertex::BufferCreationError> {
        let vertecies: Vec<Vertex> = self
            .vertecies
            .iter()
            .map(|vert| vert.get_transform(self.position, self.scale, self.rotation.into()))
            .collect();

        glium::vertex::VertexBuffer::new(facade, &vertecies)
    }

    pub fn to_index_buffer<F: glium::backend::Facade>(
        &self,
        facade: &F,
    ) -> Result<glium::index::IndexBuffer<u32>, glium::index::BufferCreationError> {
        if let Some(ref indice) = self.indices {
            glium::index::IndexBuffer::new(
                facade,
                glium::index::PrimitiveType::TrianglesList,
                indice,
            )
        } else {
            glium::index::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &[])
        }
    }
}

impl Renderable for Mesh {
    type RenderError = glium::DrawError;

    fn render<F: glium::backend::Facade>(
        &self,
        facade: &F,
        program: &glium::Program,
        target: &mut glium::Frame,
        uniforms: &glium::uniforms::UniformsStorage<
            impl glium::uniforms::AsUniformValue,
            impl glium::uniforms::Uniforms,
        >,
        draw_parameters: &glium::DrawParameters,
    ) -> Result<(), Self::RenderError> {
        let vertex_buffer = self.to_vertex_buffer(facade).unwrap(); // !Scotch

        let index_buffer = self.to_index_buffer(facade).unwrap(); // !Scotch

        target.draw(
            &vertex_buffer,
            &index_buffer,
            program,
            uniforms,
            draw_parameters,
        )
    }
}
