use std::num::Float;
use glium::uniforms::*;

pub type Pos = (f32, f32, f32);
pub type Dir = (f32, f32, f32);

pub struct Matrix {
    m: [[f32; 4]; 4]
}

impl Matrix {
    pub fn one() -> Self {
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
        let cos = Float::cos(rad);
        let sin = Float::sin(rad);
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
        let cos = Float::cos(rad);
        let sin = Float::sin(rad);
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
        let cos = Float::cos(rad);
        let sin = Float::sin(rad);
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
}

impl IntoUniformValue<'static> for Matrix {
    fn into_uniform_value(self) -> UniformValue<'static> {
        UniformValue::Mat4(self.m, true)
    }
}
