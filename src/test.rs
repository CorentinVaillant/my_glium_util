#[cfg(test)]
mod test{
    use crate::mesh::mesh::Mesh;
    use crate::mesh::obj_parser::WaveFrontParsable;

    macro_rules! chrono {
        ($func:expr,$($arg:tt)*) => {{
            let now = std::time::Instant::now();
            let result = $func($($arg)*);
            let elapsed_time = now.elapsed();
            print!("{} ",$($arg)*);
            println!("test OK ✅, took {}", elapsed_time.as_secs_f64());
            result}
        };
    }

    macro_rules! test_valid_wavefront {
        ($obj:tt) => {
            chrono!(Mesh::load_from_wavefront,format!("tests/obj/valid/{}.obj",$obj)).expect(format!("{} failed",$obj).as_str())
        };
    }

    #[test]
    pub(super) fn mesh_test() {
        println!("->\ttest mesh");
        let triangle = test_valid_wavefront!("triangle");
        let _cube     = test_valid_wavefront!("cube");
        let _sphere   = test_valid_wavefront!("sphere");
        let _suzanne  = test_valid_wavefront!("suzanne");
        let _teapot   = test_valid_wavefront!("teapot");
        let _bunny    = test_valid_wavefront!("bunny");

        assert_eq!(triangle.vertecies_number(),3);
    }
}