use my_rust_matrix_lib::my_matrix_lib::prelude::{Matrix, VectorMath};

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

#[inline]
pub fn transformation_mat(z_rot:f32,y_rot:f32,x_rot:f32,trans : VectorMath<f32,4>)->Matrix<f32,4,4>{
    let r_z = z_rotation_mat(z_rot);
    let r_y = y_rotation_mat(y_rot);
    let r_x = x_rotation_mat(x_rot);

    let rot = r_z * r_y * r_x;

    [
        [rot[0][0],rot[0][1],rot[0][2],trans[0]],
        [rot[1][0],rot[1][1],rot[1][2],trans[1]],
        [rot[2][0],rot[2][1],rot[2][2],trans[2]],
        [0.       ,0.      ,0.      ,1.        ]
        
    ].into()
}