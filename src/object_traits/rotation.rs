use core::ops::{Add, AddAssign, Mul, MulAssign};

use my_rust_matrix_lib::my_matrix_lib::prelude::{EuclidianSpace, Field, Ring};

use crate::utils::types_util::{Mat3, Mat4, QuatF32};

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    value: QuatF32,
}

impl From<QuatF32> for Rotation {
    fn from(value: QuatF32) -> Self {
        Self {
            value: value.normalized(),
        }
    }
}

impl From<Rotation> for QuatF32 {
    fn from(value: Rotation) -> Self {
        value.value
    }
}

impl Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let rhs_q: QuatF32 = rhs.into();
        let self_q: QuatF32 = self.into();
        (rhs_q * self_q).into()
    }
}

impl AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul<f32> for Rotation {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.value.powf(rhs).into()
    }
}

impl MulAssign<f32> for Rotation {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Mul<i32> for Rotation {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        if rhs.is_positive() {
            let rhs: u16 = rhs.try_into().unwrap();
            self.value.r_powu(rhs).into()
        } else {
            let rhs: u16 = (-rhs).try_into().unwrap();
            self.value.r_powu(rhs).f_mult_inverse().into()
        }
    }
}

impl MulAssign<i32> for Rotation {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs
    }
}

impl Rotation {
    pub fn zero() -> Self {
        QuatF32::one().into()
    }
}

impl Rotation {
    pub fn from_axis(angle: f32, axis: (f32, f32, f32)) -> Self {
        let (b_x, b_y, b_z) = axis;
        let axis_length = (b_x * b_x + b_y * b_y + b_z * b_z).sqrt();
        let normalized_axis = (b_x / axis_length, b_y / axis_length, b_z / axis_length);

        let a_div2 = angle / 2.0;
        let sin_a_div2 = f32::sin(a_div2);

        let q: QuatF32 = (
            f32::cos(a_div2),
            sin_a_div2 * normalized_axis.0,
            sin_a_div2 * normalized_axis.1,
            sin_a_div2 * normalized_axis.2,
        )
            .into();

        Self { value: q }
    }

    pub fn to_axis(self) -> (f32, (f32, f32, f32)) {
        let im_lenght = self.value.im.length();
        if im_lenght == 0. {
            return (0., (0., 0., 0.));
        }

        let angle = 2. * f32::atan2(im_lenght, self.value.re);
        let axis = self.value.im / im_lenght;

        (angle, (axis[0], axis[1], axis[2]))
    }

    pub fn to_mat4(self) -> Mat4 {
        let (a, b, c, d): (f32, f32, f32, f32) = self.value.into();

        let s = 2. / (a * a + b * b + c * c + d * d);
        let (bs, cs, ds) = (b * s, c * s, d * s);
        let (ab, ac, ad) = (a * bs, a * cs, a * ds);
        let (bb, bc, bd) = (b * bs, b * cs, b * ds);
        let (cc, cd, dd) = (c * cs, c * ds, d * ds);

        [
            [1. - cc - dd, bc - ad, bd + ac, 0.],
            [bc + ad, 1. - bb - dd, cd - ab, 0.],
            [bd - ac, cd + ab, 1. - bb - cc, 0.],
            [0., 0., 0., 1.],
        ]
        .into()
    }

    pub fn to_mat3(self) -> Mat3 {
        let (a, b, c, d): (f32, f32, f32, f32) = self.value.into();

        let s = 2. / (a * a + b * b + c * c + d * d);
        let (bs, cs, ds) = (b * s, c * s, d * s);
        let (ab, ac, ad) = (a * bs, a * cs, a * ds);
        let (bb, bc, bd) = (b * bs, b * cs, b * ds);
        let (cc, cd, dd) = (c * cs, c * ds, d * ds);

        [
            [1. - cc - dd, bc - ad, bd + ac],
            [bc + ad, 1. - bb - dd, cd - ab],
            [bd - ac, cd + ab, 1. - bb - cc],
        ]
        .into()
    }
}
