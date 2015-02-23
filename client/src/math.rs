use std::num::Float;
use std::ops::Mul;
use std::mem;
use glium::uniforms::*;

pub struct Matrix {
    m: [[f32; 4]; 4]
}

impl Matrix {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    pub fn identity() -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_x(rad: f32) -> Self {
        let (sin, cos) = Float::sin_cos(rad);

        Matrix {
            m: [
                [1.0, 0.0,  0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin,  cos, 0.0],
                [0.0, 0.0,  0.0, 1.0],
            ]
        }
    }

    pub fn rotation_y(rad: f32) -> Self {
        let (sin, cos) = Float::sin_cos(rad);

        Matrix {
            m: [
                [ cos, 0.0, sin, 0.0],
                [ 0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_z(rad: f32) -> Self {
        let (sin, cos) = Float::sin_cos(rad);

        Matrix {
            m: [
                [cos, -sin, 0.0, 0.0],
                [sin,  cos, 0.0, 0.0],
                [0.0,  0.0, 1.0, 0.0],
                [0.0,  0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn perspective(width: f32, height: f32, near_z: f32, far_z: f32) -> Self {
        let two_near_z = near_z + near_z;
        let range = far_z/(near_z - far_z);

        Matrix {
            m: [
                [two_near_z/width, 0.0, 0.0, 0.0],
                [0.0, two_near_z/height, 0.0, 0.0],
                [0.0, 0.0, range, range*near_z],
                [0.0, 0.0, -1.0, 0.0],
            ]
        }
    }

    /// aspect: Height / Width
    pub fn perspective_fov(fov: f32, aspect: f32, near_z: f32, far_z: f32) -> Self {
        let (sin, cos) = Float::sin_cos(0.5 * fov);
        let height = cos/sin;
        let width = height/aspect;
        let range = far_z/(near_z - far_z);

        Matrix {
            m: [
                [width, 0.0, 0.0, 0.0],
                [0.0, height, 0.0, 0.0],
                [0.0, 0.0, range, range*near_z],
                [0.0, 0.0, -1.0, 0.0],
            ]
        }
    }

    pub fn translation(offset_x: f32, offset_y: f32, offset_z: f32) -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, offset_x],
                [0.0, 1.0, 0.0, offset_y],
                [0.0, 0.0, 1.0, offset_z],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::new();

        // Cache the invariants in registers
        let mut x; let mut y; let mut z; let mut w;
        macro_rules! row {
            ($col:expr) => ({
                x = self.m[$col][0];
                y = self.m[$col][1];
                z = self.m[$col][2];
                w = self.m[$col][3];
                result.m[$col][0] = (rhs.m[0][0]*x)+(rhs.m[1][0]*y)+(rhs.m[2][0]*z)+(rhs.m[3][0]*w);
                result.m[$col][1] = (rhs.m[0][1]*x)+(rhs.m[1][1]*y)+(rhs.m[2][1]*z)+(rhs.m[3][1]*w);
                result.m[$col][2] = (rhs.m[0][2]*x)+(rhs.m[1][2]*y)+(rhs.m[2][2]*z)+(rhs.m[3][2]*w);
                result.m[$col][3] = (rhs.m[0][3]*x)+(rhs.m[1][3]*y)+(rhs.m[2][3]*z)+(rhs.m[3][3]*w);
            })
        }
        row!(0); row!(1); row!(2); row!(3);

        result
    }
}

impl IntoUniformValue<'static> for Matrix {
    fn into_uniform_value(self) -> UniformValue<'static> {
        UniformValue::Mat4(self.m, true)
    }
}
