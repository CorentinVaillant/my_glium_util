use my_rust_matrix_lib::my_matrix_lib::prelude::{EuclidianSpace, Matrix, MatrixTrait, VectorSpace};

use crate::utils::types_util::Vec3;

use super::vertex::Vertex;

/******************\
*                 *
*-----Polygons----*
*                 *
\******************/

#[derive(Debug,Clone,PartialEq)]
pub struct Polygon<const N:usize>{
    vertices :[Vertex;N]
}

impl<const N:usize> Polygon<N>{
    pub fn points(&self)->&[Vertex;N]{
        &self.vertices
    }

    pub fn edges(&self)->[[&Vertex;2];N]{
        let mut result = Vec::with_capacity(N);
        
        for i in 0..N{
            result.push([&self.vertices[i],&self.vertices[(i+1)%N]]);
        }

        result.try_into()
            .expect("something went wrong in Polygon::edges, could not convert vec to array")
    }
}

/******************\
*                 *
*-----Geometry----*
*                 *
\******************/

impl<const N:usize> Polygon<N>{
    pub fn perimeter(&self)->f32{
        let mut result = 0.;
        for i in 0..N{
            result += self.vertices[i].distance(&self.vertices[(i+1)%N]);
        }
        result
    }

    pub fn area(&self)->f32{
        (0..N).map(|i|{
          let [x1,y1,z1,_] = self.vertices[i].position;
          let [x2,y2,z2,_] = self.vertices[(i+1)%N].position;

          let v1:Vec3 = [x1,y1,z1].into();
          let v2:Vec3 = [x2,y2,z2].into();
          v1.cross_product(v2)
        })
          .fold(Vec3::v_space_zero(), |i,v|i+v) //sum (maybe I should add the sum trait to vector math)
          .length()
    }
}

impl<I:Into<[Vertex;N]>,const N:usize> From<I> for Polygon<N>{
    fn from(value: I) -> Self {
        Self{
            vertices : value.into()
        }
    }
}

/******************\
*                 *
*-----Triangle----*
*                 *
\******************/
pub type Triangle = Polygon<3>;

impl Triangle {
    pub fn circumsphere_contains(&self, point: Vertex) -> bool {
        let [ax, ay, az,_] = self.vertices[0].position;
        let [bx, by, bz,_] = self.vertices[1].position;
        let [cx, cy, cz,_] = self.vertices[2].position;
        let [dx, dy, dz,_] = self.vertices[3].position;
        let [ex, ey, ez,_] = point.position;
    
        let mat = Matrix::from([
            [ax - ex, ay - ey, az - ez, (ax - ex).powi(2) + (ay - ey).powi(2) + (az - ez).powi(2)],
            [bx - ex, by - ey, bz - ez, (bx - ex).powi(2) + (by - ey).powi(2) + (bz - ez).powi(2)],
            [cx - ex, cy - ey, cz - ez, (cx - ex).powi(2) + (cy - ey).powi(2) + (cz - ez).powi(2)],
            [dx - ex, dy - ey, dz - ez, (dx - ex).powi(2) + (dy - ey).powi(2) + (dz - ez).powi(2)]
        ]);
    
        mat.det() > 0.
    }
}

// Polygon Triangulation 
impl<const N:usize> Polygon<N>{
    pub fn triangulate(self)->Vec<Triangle>{
        let super_triangle = self.super_triangle();
        let mut triangulation = vec![super_triangle.clone()];

        for point in self.vertices{
            let bad_triangles: Vec<_> = triangulation
              .iter()
              .filter(|t|t.circumsphere_contains(point))
              .cloned()
              .collect();

            let mut polygons= vec![];
    
            for triangle in &bad_triangles{
                for edge in triangle.edges(){
                    if bad_triangles.iter().any(|other|*other != *triangle && other.edges().contains(&edge)){
                        continue;
                    }
                    polygons.push(edge);
                }
            }
            triangulation.retain(|t|!bad_triangles.contains(&t));

            for edge in polygons{
                triangulation.push(Triangle::from([*edge[0],*edge[1],point]));
            }
        }

        triangulation.retain(|t|
          !t.vertices.iter().any(|v| super_triangle.vertices.contains(v)));
        triangulation
    }

    fn super_triangle(&self) -> Polygon<3> {
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        // Find bounding box
        for v in &self.vertices {

            let &[x, y, _,_] = &v.position;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        // Compute a large triangle that covers everything
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let dmax = dx.max(dy) * 2.0; // Scale it up

        let v1 = [min_x - dmax, min_y - dmax, 0.0].into();
        let v2 = [max_x + dmax, min_y - dmax, 0.0].into();
        let v3 = [(min_x + max_x) / 2.0, max_y + dmax, 0.0].into();

        Polygon { vertices: [v1, v2, v3] }
    }
 
}