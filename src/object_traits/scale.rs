use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::utils::types_util::{Arr3F32, Vec3};

pub struct Scale{
    value : Vec3
}

impl From<Vec3> for Scale {
    fn from(value: Vec3) -> Self {
        Scale { value }
    }
}

impl From<Arr3F32> for Scale{
    fn from(value: Arr3F32) -> Self {
        Scale{value:value.into()}
    }
}

impl From<Scale> for Vec3{
    fn from(value: Scale) -> Self {
        value.value
    }
}

impl From<Scale> for Arr3F32 {
    fn from(value: Scale) -> Self {
        value.value.into()
    }
}

impl From<f32> for Scale{
    fn from(f: f32) -> Self {
        [f;3].into()
    }
}

#[derive(Debug,Clone,Copy)]
pub enum F32TryFromScaleError {
    NotAndUnidirectionalVector
}

impl TryFrom<Scale> for f32{
    type Error = F32TryFromScaleError;

    fn try_from(value: Scale) -> Result<Self, Self::Error> {
        let [x,y,z] = value.into();
        match (x==y,y==z) {
            (true, true) => Ok(x),
            _ => Err(F32TryFromScaleError::NotAndUnidirectionalVector)
        }

    }
}

impl Scale {
    /// **This does not corespond to the scaling by 0. (the float)**  
    /// This this the operation to not scale at all (i.e scale by 1. the float)
    pub fn zero()->Self{
        <[f32; 3] as Into<Vec3>>::into([1.,1.,1.]).into()
    }

    pub fn inverse(&self)->Self{
        [
            1./self.value[0],
            1./self.value[1],
            1./self.value[2],
        ].into()
    }
}

impl Add for Scale{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let [x,y,z] =self.into();
        let [a,b,c] = rhs.into();

        [x*a,y*b,z*c].into()
    }
}

impl Sub for Scale{
    type Output = Scale;
    fn sub(self, rhs: Self) -> Self::Output {
        let [x,y,z] =self.into();
        let [a,b,c] = rhs.into();

        [x/a,y/b,z/c].into()
    }
}

impl AddAssign for Scale{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3{
            self.value[i] *= rhs.value[i]
        }
    }
}

impl SubAssign for Scale {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3{
            self.value[i] /= rhs.value[i]
        }
    }
}

impl Mul<f32> for Scale {
    type Output = Scale;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.value * rhs).into()
    }
}

impl MulAssign<f32> for Scale{
    fn mul_assign(&mut self, rhs: f32) {
        self.value *= rhs
    }
}