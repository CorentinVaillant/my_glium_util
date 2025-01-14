use crate::mesh::mesh::*;


type VertexArr<const N:usize> = [Vertex;N];

#[allow(unused)]
pub const TRIANGLE :VertexArr<3> = [
    Vertex { position: [ 0.43*100.,-0.25*100.,0.,0.] },
    Vertex { position: [ 0.0 *100., 0.5 *100.,0.,0.] },
    Vertex { position: [-0.43*100.,-0.25*100.,0.,0.] },

];