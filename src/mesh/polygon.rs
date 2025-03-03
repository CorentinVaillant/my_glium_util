use my_rust_matrix_lib::my_matrix_lib::prelude::{EuclidianSpace, VectorSpace};

use crate::utils::types_util::Vec3;

use super::vertex::Vertex;

/******************\
*                 *
*-----Polygons----*
*                 *
\******************/

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon<const N: usize> {
    pub(crate) vertices: [Vertex; N],
}

impl<const N: usize> Polygon<N> {
    pub fn points(&self) -> &[Vertex; N] {
        &self.vertices
    }

    pub fn edges(&self) -> [[&Vertex; 2]; N] {
        let mut result = Vec::with_capacity(N);

        for i in 0..N {
            result.push([&self.vertices[i], &self.vertices[(i + 1) % N]]);
        }

        result
            .try_into()
            .expect("something went wrong in Polygon::edges, could not convert vec to array")
    }
}

/******************\
*                 *
*-----Geometry----*
*                 *
\******************/

impl<const N: usize> Polygon<N> {
    pub fn perimeter(&self) -> f32 {
        let mut result = 0.;
        for i in 0..N {
            result += self.vertices[i].distance(&self.vertices[(i + 1) % N]);
        }
        result
    }

    pub fn area(&self) -> f32 {
        (0..N)
            .map(|i| {
                let [x1, y1, z1, _] = self.vertices[i].position;
                let [x2, y2, z2, _] = self.vertices[(i + 1) % N].position;

                let v1: Vec3 = [x1, y1, z1].into();
                let v2: Vec3 = [x2, y2, z2].into();
                v1.cross_product(v2)
            })
            .fold(Vec3::v_space_zero(), |i, v| i + v) //sum (maybe I should add the sum trait to vector math)
            .length()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PolygonCreationError {
    NotEnoughVertices,
}

impl<const N: usize> TryFrom<[Vertex; N]> for Polygon<N> {
    type Error = PolygonCreationError;

    fn try_from(vertices: [Vertex; N]) -> Result<Self, Self::Error> {
        if N < 3 {
            Err(PolygonCreationError::NotEnoughVertices)
        } else {
            Ok(Polygon { vertices })
        }
    }
}

/******************\
*                 *
*-----Triangle----*
*                 *
\******************/
pub type Triangle = Polygon<3>;

//Todo triangulation
