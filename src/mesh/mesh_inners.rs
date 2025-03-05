use std::rc::{Rc, Weak};

use super::{half_edge::HalfEdge, vertex::Vertex, Mesh};

struct MeshVertex{
    vertex : Vertex, 

    half_edge : Option<Weak<HalfEdge>>,
    mesh : Rc<Mesh>
}

impl MeshVertex{
    pub(crate)fn new(mesh:Rc<Mesh>,vertex:Vertex)->Self{
        Self { vertex, half_edge: None, mesh }
    }
}