use my_rust_matrix_lib::my_matrix_lib::prelude::Matrix;

#[inline]
pub fn x_rotation_mat(angle :f32)->Matrix<f32,4,4>{
    let c = angle.cos();
    let s = angle.sin();
    
    [
        [1.,0.,0.,0.],
        [0.,c ,-s,0.],
        [0.,s ,c ,0.],
        [0.,0.,0.,1.]
    ].into()
}

#[inline]
pub fn y_rotation_mat(angle :f32)->Matrix<f32,4,4>{
    let c = angle.cos();
    let s = angle.sin();
    
    [
        [c ,0.,s ,0.],
        [0.,1.,0.,0.],
        [-s,0.,c ,0.],
        [0.,0.,0.,1.]
    ].into()
}

#[inline]
pub fn z_rotation_mat(angle :f32)->Matrix<f32,4,4>{
    let c = angle.cos();
    let s = angle.sin();
    
    [
        [c ,-s,0.,0.],
        [s ,c ,0.,0.],
        [0.,0.,1.,0.],
        [0.,0.,0.,1.]
    ].into()
}

pub fn rotation_mat((x,y,z):(f32,f32,f32))->Matrix<f32,4,4>{
    z_rotation_mat(z)
    * y_rotation_mat(y)
    * x_rotation_mat(x)
}