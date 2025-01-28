use core::ops::{Add, AddAssign, Mul, MulAssign};
use core::ops::Index;

use my_rust_matrix_lib::my_matrix_lib::prelude::VectorSpace;

use crate::utils::types_util::{Arr3F32, Vec3};

#[derive(Debug,Clone, Copy)]
pub struct Translation{
    inner:Vec3
}

impl Translation{
    pub fn zero()->Self{
        Vec3::v_space_zero().into()
    }

    pub fn inverse(&self)->Self{
        (-self.inner).into()
    }
}


impl From<Vec3> for Translation{
    fn from(value: Vec3) -> Self {
        Self { inner: value }
    }
}

impl From<Translation> for Vec3{
    fn from(value: Translation) -> Self {
        value.inner
    }
}

impl From<Arr3F32> for Translation{
    fn from(value: Arr3F32) -> Self {
        Self{
            inner : value.into()
        }
    }
}

impl From<Translation> for Arr3F32{
    fn from(value: Translation) -> Self {
        value.inner.into()
    }
}

impl  Add for Translation{
    type Output=Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.inner + rhs.inner).into()
    }
}

impl AddAssign for Translation {
    fn add_assign(&mut self, rhs: Self) {
        self.inner+=rhs.inner;
    }
}

impl Mul<f32> for Translation{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        (self.inner*rhs).into()
    }
}

impl MulAssign<f32> for Translation{
    fn mul_assign(&mut self, rhs: f32) {
        self.inner*=rhs
    }
}

impl Index<char> for Translation{
    type Output = f32;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x'=>&self.inner[0],
            'y'=>&self.inner[1],
            'z'=>&self.inner[2],
             _ =>panic!("only use x y or z to index Translation")
        }
    }
}