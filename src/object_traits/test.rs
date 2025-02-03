#[cfg(test)]

#[test]
pub(super) fn transformation_test() {
    use crate::{object_traits::Rotation, utils::macro_util::chrono};

    chrono!({
        println!("->\ttest transformation");

        let axis = (core::f32::consts::FRAC_PI_2, (1., 0., 0.));
        let rotation = Rotation::from_axis(axis.0, axis.1);
        assert_eq!(axis, rotation.to_axis());
    })
}