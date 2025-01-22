use crate::utils::types_util::QuatF32;


#[derive(Debug,Clone,Copy)]
pub struct Rotation{
    value : QuatF32
}

impl From<QuatF32> for Rotation {
    fn from(value: QuatF32) -> Self {
        Self { value: value.normalized() }
    }
}

impl From<Rotation> for QuatF32{
    fn from(value: Rotation) -> Self {
        value.value
    }
}

impl core::ops::Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let a:QuatF32 = rhs.into();
        let b:QuatF32 = self.into();
        (a*b).into()
    }
}

impl core::ops::AddAssign for Rotation{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Rotation {
    pub fn zero()->Self{
        QuatF32::one().into()
    }
}

impl Rotation{
    pub fn from_axis(angle:f32,axis : (f32,f32,f32))->Self{
        let (b_x,b_y,b_z) = axis;
        let a_div2 = angle/2.;
        let sin_a_div2 = f32::sin(a_div2);

        let q:QuatF32 = (
            f32::cos(a_div2),
            sin_a_div2 * f32::cos(b_x),
            sin_a_div2 * f32::cos(b_y),
            sin_a_div2 * f32::cos(b_z)
        ).into();

        q.into()
    }
}