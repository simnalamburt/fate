use std::num::Float;
use std::mem::zeroed;
use std::ops::*;
use glium::uniforms::*;

#[simd]
#[derive(Copy, Debug)]
pub struct Vector { x: f32, y: f32, z: f32 }

pub fn vec(x: f32, y: f32, z: f32) -> Vector { Vector { x: x, y: y, z: z } }

pub fn dot(lhs: Vector, rhs: Vector) -> f32 {
    let c = lhs * rhs;
    c.x + c.y + c.z
}

pub fn cross(lhs: Vector, rhs: Vector) -> Vector {
    Vector {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x
    }
}

impl Vector {
    pub fn new() -> Self { unsafe { zeroed() } }

    pub fn length(self) -> f32 { self.length_sq().sqrt() }
    pub fn length_sq(self) -> f32 { dot(self, self) }

    pub fn normalize(self) -> Self {
        let mut len = self.length();
        if len > 0.0 { len = 1.0/len; }
        self * vec(len, len, len)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self { Vector::new() - self }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix { m: [[f32; 4]; 4] }

pub const IDENTITY: Matrix = Matrix {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
};

impl Matrix {
    pub fn new() -> Self { unsafe { zeroed() } }
    pub fn identity() -> Self { IDENTITY }

    pub fn look_at(eye: Vector, focus: Vector, up: Vector) -> Self {
        Matrix::look_to(eye, focus - eye, up)
    }

    pub fn look_to(eye: Vector, dir: Vector, up: Vector) -> Self {
        // assert!(!XMVector3Equal(EyeDirection, XMVectorZero()));
        // assert!(!XMVector3IsInfinite(EyeDirection));
        // assert!(!XMVector3Equal(UpDirection, XMVectorZero()));
        // assert!(!XMVector3IsInfinite(UpDirection));

        let neg_eye = -eye;
        let neg_dir = -dir;

        let r2 = neg_dir.normalize();
        let r0 = cross(up, r2).normalize();
        let r1 = cross(r2, r0);

        let d0 = dot(r0, neg_eye);
        let d1 = dot(r1, neg_eye);
        let d2 = dot(r2, neg_eye);

        Matrix {
            m: [
                [r0.x, r0.y, r0.z, d0],
                [r1.x, r1.y, r1.z, d1],
                [r2.x, r2.y, r2.z, d2],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_x(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

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
        let (sin, cos) = rad.sin_cos();

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
        let (sin, cos) = rad.sin_cos();

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

    /// aspect: Width / Height
    pub fn perspective_fov(fov: f32, aspect: f32, near_z: f32, far_z: f32) -> Self {
        let (sin, cos) = (0.5 * fov).sin_cos();
        let f = cos/sin;
        let range = far_z/(near_z - far_z);

        Matrix {
            m: [
                [f/aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
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

    pub fn transpose(self) -> Self {
        Matrix {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
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
