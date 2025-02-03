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
