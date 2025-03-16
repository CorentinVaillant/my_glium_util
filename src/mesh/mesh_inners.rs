use std::{cell::RefCell, rc::Rc};


use super::{vertex::Vertex, Mesh};

#[derive(Debug)]
pub(crate) struct InnerMesh{
    pub(crate) mesh : Rc<RefCell<Mesh>>,

    pub(crate) vertex_list: Vec<MeshVertex>,
    pub(crate) triangle_list: Vec<TriangleMesh>
}


impl InnerMesh {
    fn pop_vertex(&mut self, _vert:&MeshVertex){
        todo!()
    }
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
    pub(crate) origin: MeshVertexRef,

    pub(crate) mesh: Option<InnerMeshRef>,
    pub(crate) triangle: Option<TriangleMeshRef>,
    pub(crate) edge :Option<EdgeMeshRef>,
    pub(crate) next: Option<MeshHalfEdgeRef>,
    pub(crate) prev: Option<MeshHalfEdgeRef>,
    pub(crate) sibling: Option<MeshHalfEdgeRef>,
    pub(crate) oposite: Option<MeshHalfEdgeRef>,
}

pub(crate) type NonManifoldMesh = ();

impl MeshHalfEdge {
    pub(crate) fn new(mut mesh:Option<&mut Mesh>,origin:MeshVertexRef, target:MeshVertexRef, triangle:Option<TriangleMeshRef>) -> Result<MeshHalfEdgeRef,NonManifoldMesh> {
        let result_ref = Rc::new(RefCell::new(MeshHalfEdge {
            origin:origin.clone(),
            mesh : mesh.as_deref().map(|m|m.inner_mesh.clone()),
            triangle,
            edge : None,
            next: None,
            prev: None,
            sibling:None,
            oposite: None,
        }));

        let mut result = result_ref.borrow_mut();

        if let Some(m) = mesh.as_deref_mut() { m.push_halh_edge(result_ref.clone()) }

        let other = target.borrow().get_half_edge_to(origin.clone());
        if other.is_some(){
            if other.clone().is_some_and(|o|o.borrow().oposite.is_some()){
                return Err(());
            }
            result.oposite = other.clone();
            if let Some(other) = other.clone(){other.borrow_mut().oposite=Some(result_ref.clone())}

            result.edge = other.and_then(|o|o.borrow().edge.clone());
        }

        if result.edge.is_none(){
            result.edge = Some(EdgeMesh::new(mesh.as_deref(), result.origin.clone(), target));
        }

        m_vert_ref_link_sibling(origin, result_ref.clone());

        drop(result);
        Ok(result_ref)
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

pub(crate) type MeshVertexRef = Rc<RefCell<MeshVertex>>;

impl MeshVertex{
    pub(crate)fn new(mesh:Option<&Mesh>,vertex:Vertex)->Self{
        Self { vertex, half_edge: None, mesh :mesh.map(|m|m.inner_mesh.clone())}
    }

    pub(crate) fn get_vertex(&self)->&Vertex{
        &self.vertex
    }

    pub(crate) fn get_half_edge_to(&self, vertex:MeshVertexRef)->Option<MeshHalfEdgeRef>{
        let mut half_edge_opt = self.half_edge.clone();
        let vertex_ptr = vertex.as_ptr();
        while let Some(ref half_edge) = half_edge_opt.clone() {
            if half_edge.borrow().get_target().is_some_and(|v|v.as_ptr() == vertex_ptr){
                return Some(half_edge.clone());
            }
            half_edge_opt = half_edge.borrow().sibling.clone();
        }
        None
    }

}

impl Drop for MeshVertex{
    fn drop(&mut self) {

        //remove from the mesh vertex list
        if let Some(mesh) = &self.mesh{
            mesh.borrow_mut().pop_vertex(self);
        }
        self.mesh = None;

        let half_edge_opt = &self.half_edge;
        while let Some(half_edge) = half_edge_opt {
            let next = &half_edge.borrow().sibling;
            let a = half_edge.borrow().triangle.as_ref();

            drop();
        }

    }
}

fn m_vert_ref_link_sibling(vert:MeshVertexRef, halfedge:MeshHalfEdgeRef){
    halfedge.borrow_mut().sibling = vert.borrow().half_edge.clone();
    vert.borrow_mut().half_edge = Some(halfedge);

}

/**********************\
*                     *
*-----TriangleMesh----*
*                     *
\**********************/

#[derive(Debug)]
pub(crate) struct TriangleMesh{
    mesh : Option<InnerMeshRef>,
    vertices : [MeshVertexRef;3],
    half_edge: MeshHalfEdgeRef
}

pub(crate) type TriangleMeshRef = Rc<RefCell<TriangleMesh>>;

impl TriangleMesh {
    const ERR_MSH:&str = "something went wrong in TriangleMesh::new";

    pub(crate) fn new(mut mesh: Option<&mut Mesh>, vertices:[Vertex;3])-> Result<TriangleMeshRef,NonManifoldMesh>{


        let vertices = vertices.map(|v|Rc::new(RefCell::new(MeshVertex::new(mesh.as_deref(), v))));
        //creation of tree half edges 
        let mut half_edges = [const { None };3];
        for i in 0..3{
            half_edges[i] = Some(MeshHalfEdge::new(mesh.as_deref_mut(), vertices[i].clone(), vertices[(i+1)%3].clone(), None)?);
        }

        let [h1,h2,h3] = half_edges;

        let result = Rc::new(RefCell::new(
        Self{
            mesh: mesh.as_mut().map(|m|m.inner_mesh.clone()),
            vertices,
            half_edge: h1.clone().expect(Self::ERR_MSH)
        }));

        h1.expect(Self::ERR_MSH).borrow_mut().triangle = Some(result.clone());
        h2.expect(Self::ERR_MSH).borrow_mut().triangle = Some(result.clone());
        h3.expect(Self::ERR_MSH).borrow_mut().triangle = Some(result.clone());
        if let Some(mesh) = mesh.as_mut() { mesh.push_triangle(result.clone()) }

        Ok(result)



    }
}

impl Drop for TriangleMesh{
    fn drop(&mut self) {
        todo!()
    }
}


/**********************\
*                     *
*-------EdgeMesh------*
*                     *
\**********************/

pub(crate) type EdgeMeshRef = Rc<RefCell<EdgeMesh>>;
#[derive(Debug)]
pub(crate) struct EdgeMesh{
    mesh:Option<InnerMeshRef>,
    v1:MeshVertexRef,
    v2:MeshVertexRef
}

impl EdgeMesh {
    pub(crate)fn new(mesh:Option<&Mesh>,v1:MeshVertexRef,v2:MeshVertexRef)->EdgeMeshRef{
        Rc::new(RefCell::new(Self{
            mesh : mesh.map(|m|m.inner_mesh.clone()),
            v1,v2
        }))
    }
}