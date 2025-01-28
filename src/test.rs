#[cfg(test)]
mod test {
    use crate::mesh::mesh::Mesh;
    use crate::mesh::obj_parser::WaveFrontParsable;
    use crate::object_traits::{Rotation, SceneObject};

    macro_rules! chrono {
        ($func:expr,$($arg:tt)*) => {{
            let now = std::time::Instant::now();
            let result = $func($($arg)*);
            let elapsed_time = now.elapsed();
            println!("test OK ✅, took {}", elapsed_time.as_secs_f64());
            result}
        };
    }

    macro_rules! test_valid_wavefront {
        ($obj:tt) => {
            chrono!(
                Mesh::load_from_wavefront,
                format!("tests/obj/valid/{}.obj", $obj)
            )
            .expect(format!("{} failed", $obj).as_str())
        };
    }

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

    #[test]
    pub(super) fn transformation_test() {
        println!("->\ttest transformation");

        let axis = (core::f32::consts::FRAC_PI_2, (1., 0., 0.));
        let rotation = Rotation::from_axis(axis.0, axis.1);
        assert_eq!(axis, rotation.to_axis());
    }
}
