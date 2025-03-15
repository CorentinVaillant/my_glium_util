use std::{cell::RefCell, ptr, rc::{Rc, Weak}};

use crate::file_parser::NotImpl;

use super::{vertex::Vertex, Mesh};

#[derive(Debug)]
pub(crate) struct InnerMesh{
    pub(crate) vertex_list: Vec<MeshVertex>,
    pub(crate) triangle_list: Vec<TriangleMesh>
}

pub(crate) type InnerMeshRef = Rc<RefCell<InnerMesh>>;

/**********************\
*                     *
*-----HalfEdgeMesh----*
*                     *
\**********************/

pub(crate)type MeshHalfEdgeRef = Rc<RefCell<MeshHalfEdge>>;

#[derive(Debug)]
pub(crate) struct MeshHalfEdge {
    pub(crate) origin: Rc<MeshVertex>,

    pub(crate) mesh: Option<InnerMeshRef>,
    pub(crate) triangle: Option<TriangleMeshRef>,
    pub(crate) next: Option<MeshHalfEdgeRef>,
    pub(crate) prev: Option<MeshHalfEdgeRef>,
    pub(crate) oposite: Option<MeshHalfEdgeRef>,
    pub(crate) sibling: Option<MeshHalfEdgeRef>
}

pub(crate) type NonManifoldMesh = ();

impl MeshHalfEdge {
    pub(crate) fn new(mesh:Option<&mut Mesh>,origin:MeshVertexRef, target:MeshVertexRef, triangle:Option<TriangleMeshRef>) -> Result<MeshHalfEdgeRef,NonManifoldMesh> {
        let mut result = MeshHalfEdge {
            origin:origin.clone(),
            mesh : mesh.as_deref().map(|m|m.inner_mesh.clone()),
            triangle,
            next: None,
            prev: None,
            oposite: None,
            sibling:None,
        };

        mesh.map(|m|m.push_halh_edge(result));

        let other = target.get_half_edge_to(origin);
        if other.is_some(){
            if other.is_some_and(|o|o.borrow().oposite.is_some()){
                return Err(());
            }
            //!here
            result.oposite = other;

        }



        todo!()
    }

    pub(crate) fn get_target(&self)->Option<MeshVertexRef>{
        self.next.clone().map(|he|he.borrow().origin.clone())
    }
}

pub(crate) struct HalfEdgeFaceIterator {
    start_h_e: MeshHalfEdgeRef,
    curr_h_e: Option<MeshHalfEdgeRef>,

    started: bool,
}

impl HalfEdgeFaceIterator {
    pub(crate) fn new(start: MeshHalfEdgeRef) -> Self {
        Self {
            start_h_e: start.clone(),
            curr_h_e: Some(start),
            started: false,
        }
    }
}

impl Iterator for HalfEdgeFaceIterator {
    type Item = Rc<MeshVertex>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop iteration if we have completed a full loop
        if self.started
            && self
                .curr_h_e
                .as_ref()
                .is_some_and(|curr| Rc::ptr_eq(curr, &self.start_h_e))
        {
            return None;
        }

        self.started = true;

        let curr = self.curr_h_e.clone()?;
        let vert = curr.borrow().origin.clone();

        self.curr_h_e = curr.borrow().next.clone();

        Some(vert)
    }
}

pub(crate) struct HalfEdgeFace {
    pub(super) origin: MeshHalfEdgeRef,
}

impl HalfEdgeFace {
    pub(crate) fn iter(&self) -> HalfEdgeFaceIterator {
        HalfEdgeFaceIterator::new(self.origin.clone())
    }
}



/********************\
*                   *
*-----MeshVertex----*
*                   *
\********************/

#[derive(Debug)]
pub(crate) struct MeshVertex{
    vertex : Vertex, 

    mesh : Option<InnerMeshRef>,
    half_edge : Option<MeshHalfEdgeRef>,
}

pub(crate) type MeshVertexRef = Rc<MeshVertex>;

impl MeshVertex{
    pub(crate)fn new(mesh:Option<&Mesh>,vertex:Vertex)->Self{
        Self { vertex, half_edge: None, mesh :mesh.map(|m|m.inner_mesh.clone())}
    }

    pub(crate) fn get_vertex(&self)->&Vertex{
        &self.vertex
    }

    pub(crate) fn get_half_edge_to(&self, vertex:MeshVertexRef)->Option<MeshHalfEdgeRef>{
        let mut opt_half_edge = self.half_edge.clone();
        let vertex = Some(vertex.as_ref() as *const MeshVertex);

        while let Some(ref half_edge) =  opt_half_edge{
            if half_edge.borrow().get_target().map(|v|v.as_ref() as *const MeshVertex) == vertex {
                return opt_half_edge;}
            opt_half_edge = half_edge.borrow().sibling.clone()
        }

        todo!()


    }
}

/**********************\
*                     *
*-----TriangleMesh----*
*                     *
\**********************/

#[derive(Debug)]
pub(crate) struct TriangleMesh{
    mesh : Option<InnerMeshRef>,
    vertices : [Rc<MeshVertex>;3],
    half_edge: MeshHalfEdgeRef
}

pub(crate) type TriangleMeshRef = Rc<RefCell<TriangleMesh>>;

impl TriangleMesh {
    pub(crate) fn new(mesh:&mut Option<Mesh>, vertices:[Vertex;3])-> TriangleMeshRef{


        let vertices = vertices.map(|v|Rc::new(MeshVertex::new(mesh.as_ref(), v)));
        let half_edges = vertices.clone().map(|v|MeshHalfEdge::new(mesh.as_mut(), v, (), None));
        for i in 0..3{
            half_edges[i].borrow_mut().next = Some(half_edges[(i+1)%3].clone());
        }

        let [h1,h2,h3] = half_edges;

        let result = Rc::new(RefCell::new(
        Self{
            mesh: mesh.as_mut().map(|m|m.inner_mesh.clone()),
            vertices,
            half_edge: h1.clone()
        }));

        h1.borrow_mut().triangle = Some(result.clone());
        h2.borrow_mut().triangle = Some(result.clone());
        h3.borrow_mut().triangle = Some(result.clone());
        mesh.as_mut().map(|m| m.push_triangle(result.clone()));

        result



    }
}
