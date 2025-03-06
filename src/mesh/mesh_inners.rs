use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::file_parser::NotImpl;

use super::{vertex::Vertex, Mesh};

#[derive(Debug)]
pub(crate) struct InnerMesh{
    pub(crate) vertex_list: Vec<MeshVertex>
}

pub(crate) type InnerMeshRef = Rc<RefCell<InnerMesh>>;

/**********************\
*                     *
*-----HalfEdgeMesh----*
*                     *
\**********************/

type HalfEdgeRef = Rc<RefCell<HalfEdgeMesh>>;

#[derive(Debug)]
pub(crate) struct HalfEdgeMesh {
    pub(crate) origin: Rc<MeshVertex>,

    pub(crate) mesh: Option<Rc<Mesh>>,
    pub(crate) triangle: Rc<TriangleMesh>,
    pub(crate) next: Option<HalfEdgeRef>,
    pub(crate) prev: Option<HalfEdgeRef>,
    pub(crate) oposite: Option<HalfEdgeRef>,
    pub(crate) sibling: Option<HalfEdgeRef>
}

impl HalfEdgeMesh {
    pub(crate) fn new(mesh:Option<Rc<Mesh>>,origin:Rc<MeshVertex>, _target:NotImpl, triangle:Rc<TriangleMesh>) -> HalfEdgeRef {
        Rc::new(RefCell::new(HalfEdgeMesh {
            origin,
            mesh,
            triangle,
            next: None,
            prev: None,
            oposite: None,
            sibling:None,
        }))
    }
}

pub(crate) struct HalfEdgeFaceIterator {
    start_h_e: HalfEdgeRef,
    curr_h_e: Option<HalfEdgeRef>,

    started: bool,
}

impl HalfEdgeFaceIterator {
    pub(crate) fn new(start: HalfEdgeRef) -> Self {
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
    pub(super) origin: HalfEdgeRef,
}

impl HalfEdgeFace {
    pub(crate) fn iter(&self) -> HalfEdgeFaceIterator {
        HalfEdgeFaceIterator::new(self.origin.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FromVertSliceError {
    //Should have at least 3 vertecies !
    NotEnoughVertices,
}

// impl TryFrom<&[Rc<MeshVertex>]> for HalfEdgeFace {
//     type Error = FromVertSliceError;

//     fn try_from(vertices: &[Rc<MeshVertex>]) -> Result<Self, Self::Error> {
//         if vertices.len() < 3 {
//             Err(FromVertSliceError::NotEnoughVertices)
//         } else {
//             // Create half-edges for each vertex
//             let edges: Vec<HalfEdgeRef> = vertices
//                 .iter()
//                 .map(|v| HalfEdgeMesh::new(Rc::clone(v)))
//                 .collect();

//             let len = edges.len();
//             for i in 0..len {
//                 let next_id = (i + 1) % len;
//                 let prev_id = (i + len - 1) % len;

//                 // Link the half-edges in a circular fashion
//                 edges[i].borrow_mut().next = Some(Rc::clone(&edges[next_id]));
//                 edges[i].borrow_mut().prev = Some(Rc::clone(&edges[prev_id]));
//             }

//             Ok(Self {
//                 origin: edges[0].clone(),
//             })
//         }
//     }
// }

// impl<const N: usize> TryFrom<[Rc<Vertex>; N]> for HalfEdgeFace {
//     type Error = FromVertSliceError;

//     fn try_from(vertices: [Rc<Vertex>; N]) -> Result<Self, Self::Error> {
//         if N < 3 {
//             Err(FromVertSliceError::NotEnoughVertices)
//         } else {
//             let edges: Vec<HalfEdgeRef> = vertices
//                 .iter()
//                 .map(|v| HalfEdgeMesh::new(Rc::clone(v)))
//                 .collect();

//             for i in 0..N {
//                 let next_id = (i + 1) % N;
//                 let prev_id = (i + N - 1) % N;

//                 // Link the half-edges to form a closed loop
//                 edges[i].borrow_mut().next = Some(Rc::clone(&edges[next_id]));
//                 edges[i].borrow_mut().prev = Some(Rc::clone(&edges[prev_id]));
//             }

//             Ok(Self {
//                 origin: edges[0].clone(),
//             })
//         }
//     }
// }

// impl<const N: usize> From<Polygon<N>> for HalfEdgeFace {
//     fn from(polygon: Polygon<N>) -> Self {
//         polygon
//             .vertices
//             .map(Rc::new)
//             // A polygon have at least 3 vertices, the try_into always succeed
//             .try_into()
//             .expect("something went wrong inside From<Polygon<N>> for HalfEdgeFace")
//     }
// }

/********************\
*                   *
*-----MeshVertex----*
*                   *
\********************/

#[derive(Debug)]
pub(crate) struct MeshVertex{
    vertex : Vertex, 

    mesh : Option<InnerMeshRef>,
    half_edge : Option<Weak<HalfEdgeMesh>>,
    
}

impl MeshVertex{
    pub(crate)fn new(mesh:Option<InnerMeshRef>,vertex:Vertex)->Self{
        Self { vertex, half_edge: None, mesh }
    }

    pub(crate) fn get_vertex(&self)->&Vertex{
        &self.vertex
    }
}

/**********************\
*                     *
*-----TriangleMesh----*
*                     *
\**********************/

#[derive(Debug)]
pub(crate) struct TriangleMesh{
    vertices : [Rc<MeshVertex>;3]
}

