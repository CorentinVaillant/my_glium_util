use crate::utils::types_util::Mat4;

pub trait Camera {
    fn projection_matrix(&self) -> Mat4;
}

pub struct OrthographicCam {
    right: f32,
    top: f32,
    far: f32,

    left: f32,
    bottom: f32,
    near: f32,
}

impl OrthographicCam {
    pub fn new(right: f32, top: f32, far: f32, left: f32, bottom: f32, near: f32) -> Self {
        Self {
            right,
            top,
            far,
            left,
            bottom,
            near,
        }
    }
}

impl Camera for OrthographicCam{
    fn projection_matrix(&self) -> Mat4 {
        
        let c = ((self.near+self.left)/2.,(self.bottom+self.top)/2.,self.near);
        let s = (2./(self.right-self.left),2./(self.top-self.bottom),-2./(self.far-self.near));

        [
            [s.0, 0.0, 0.0,-c.0],
            [0.0, s.1, 0.0,-c.1],
            [0.0, 0.0, s.2,-c.2],
            [0.0, 0.0, 0.0, 1.0]
        ].into()
    }
}