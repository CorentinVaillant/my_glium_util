pub struct Camera {
    pub position: [f32; 3],
    pub right_vector: [f32; 3],
    pub up_vector: [f32; 3],
    pub direction_vector: [f32; 3],
}

impl Into<[[f32; 4]; 4]> for Camera {
    fn into(self) -> [[f32; 4]; 4] {
        let r = self.right_vector;
        let u = self.up_vector;
        let d = self.direction_vector;
        let p = self.position;
        let m1 = [
            [r[0], r[1], r[2], 0.],
            [u[0], u[1], u[2], 0.],
            [d[0], d[1], d[2], 0.],
            [0., 0., 0., 1.],
        ];
        let m2 = [
            [1., 0., 0., -p[0]],
            [0., 1., 0., -p[1]],
            [0., 0., 1., -p[2]],
            [0., 0., 0., 1.],
        ];

        return mat4_mutl(m1, m2);
    }
}

fn mat4_mutl(m1: [[f32; 4]; 4], m2: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    //naive algorithm
    let mut result = [[0.; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] = result[i][j] + m1[i][k] * m2[k][j];
            }
        }
    }
    result
}

//TODO
//? https://learnopengl.com/Getting-started/Camera
