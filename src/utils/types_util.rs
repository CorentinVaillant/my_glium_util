use my_rust_matrix_lib::my_matrix_lib::{
    prelude::{Matrix, VectorMath},
    quaternion::Quaternion,
};

pub type Arr3F32 = [f32; 3];
pub type QuatF32 = Quaternion<f32>;
pub type Vec3 = VectorMath<f32, 3>;
pub type Mat4 = Matrix<f32, 4, 4>;
pub type Mat3 = Matrix<f32, 3, 3>;
