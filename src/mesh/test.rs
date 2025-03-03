/*
#![cfg(test)]

    macro_rules! test_valid_wavefront {
        ($obj:tt) => {
            chrono!(
                Mesh::load_from_wavefront,
                format!("tests/obj/valid/{}.obj", $obj)
            )
            .expect(format!("{} failed", $obj).as_str())
        };
}
*/

/*
#[test]
pub(super) fn mesh_test() {
    println!("->\ttest mesh");
    let triangle = test_valid_wavefront!("triangle");
    let mut cube = test_valid_wavefront!("cube");
    let mut sphere = test_valid_wavefront!("sphere");
    let _suzanne = test_valid_wavefront!("suzanne");
    let _teapot = test_valid_wavefront!("teapot");
    let _bunny = test_valid_wavefront!("bunny");

    assert_eq!(triangle.vertecies_number(), 3);

    chrono!(Mesh::translate, &mut cube, [1., 1., 1.].into());
    chrono!(Mesh::apply_position, &mut cube);
    let rotation = Rotation::from_axis(3.1415, (0., 1., 0.));
    chrono!(Mesh::rotate, &mut sphere, rotation);
}
*/

#[cfg(test)]
mod tests_half_edges {
    use std::rc::Rc;

    use crate::mesh::{
        half_edge::{HalfEdge, HalfEdgeFace, HalfEdgeFaceIterator},
        vertex::Vertex,
    };

    /// Helper function to create a triangle face
    fn create_test_face() -> HalfEdgeFace {
        let v1 = Rc::new([0.0, 0.0, 0.0].into());
        let v2 = Rc::new([1.0, 0.0, 0.0].into());
        let v3 = Rc::new([0.0, 1.0, 0.0].into());

        HalfEdgeFace::try_from([v1, v2, v3]).unwrap()
    }

    /// Test: Face creation correctly links half-edges
    #[test]
    fn test_face_creation() {
        let face = create_test_face();
        let edge = face.origin.clone();

        let e1 = edge.clone();
        let e2 = e1.borrow().next.as_ref().unwrap().clone();
        let e3 = e2.borrow().next.as_ref().unwrap().clone();

        // Ensure the cycle is correctly formed
        assert_eq!(e3.borrow().next.as_ref().unwrap().as_ptr(), e1.as_ptr());

        // Ensure previous links are correctly formed
        assert_eq!(e1.borrow().prev.as_ref().unwrap().as_ptr(), e3.as_ptr());
        assert_eq!(e2.borrow().prev.as_ref().unwrap().as_ptr(), e1.as_ptr());
        assert_eq!(e3.borrow().prev.as_ref().unwrap().as_ptr(), e2.as_ptr());
    }

    /// Test: Iterating over vertices in a face returns them in the correct order
    #[test]
    fn test_vertex_iteration() {
        let face = create_test_face();
        let vertices: Vec<[f32; 4]> = face.iter().map(|v| v.position).collect();

        println!("{:?}", vertices);

        assert_eq!(vertices.len(), 3);
        assert_eq!(vertices[0], [0.0, 0.0, 0.0, 1.]);
        assert_eq!(vertices[1], [1.0, 0.0, 0.0, 1.]);
        assert_eq!(vertices[2], [0.0, 1.0, 0.0, 1.]);
    }

    /// Test: Half-edges correctly reference their origin vertices
    #[test]
    fn test_half_edge_origin() {
        let face = create_test_face();
        let edge = face.origin.clone();

        let v1 = edge.borrow().origin.clone();
        let v2 = edge.borrow().next.as_ref().unwrap().borrow().origin.clone();
        let v3 = edge
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .origin
            .clone();

        assert_eq!(v1.position, [0.0, 0.0, 0.0, 1.]);
        assert_eq!(v2.position, [1.0, 0.0, 0.0, 1.]);
        assert_eq!(v3.position, [0.0, 1.0, 0.0, 1.]);
    }

    #[test]
    fn test_iteration_on_disconnected_edge() {
        let v1: Rc<Vertex> = Rc::new([0.0, 0.0, 0.0].into());

        let edge = HalfEdge::new(v1.clone()); // Only one half-edge, no links!

        let mut iter = HalfEdgeFaceIterator::new(edge);

        let _first_vertex = iter.next();
        let second_vertex = iter.next();

        assert!(
            second_vertex.is_none(),
            "Iterator should stop after a single vertex!"
        );
    }
}
