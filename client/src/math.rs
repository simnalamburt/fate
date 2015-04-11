use std::mem::zeroed;
use std::ops::*;
use glium::uniforms::*;

#[simd]
#[derive(Clone, Copy, Debug)]
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
                [r0.x, r1.x, r2.x, 0.0],
                [r0.y, r1.y, r2.y, 0.0],
                [r0.z, r1.z, r2.z, 0.0],
                [d0,   d1,   d2,   1.0],
            ]
        }
    }

    pub fn rotation_x(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [1.0,  0.0, 0.0, 0.0],
                [0.0,  cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0,  0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_y(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0,  0.0, 0.0],
                [sin, 0.0,  cos, 0.0],
                [0.0, 0.0,  0.0, 1.0],
            ]
        }
    }

    pub fn rotation_z(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [ cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn orthographic_off_center(view_left: f32, view_right: f32, view_bottom: f32, view_top: f32, near_z: f32, far_z: f32) -> Self {
        // reciprocal width and height
        let r_width = 1.0/(view_right - view_left);
        let r_height = 1.0/(view_top - view_bottom);
        let range = 1.0/(near_z-far_z);

        Matrix {
            m: [
                [r_width + r_width, 0.0, 0.0, 0.0],
                [0.0, r_height + r_height, 0.0, 0.0],
                [0.0, 0.0, range, 0.0],
                [-(view_left + view_right)*r_width, -(view_top + view_bottom)*r_height, range*near_z, 1.0],
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
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range*near_z, 0.0],
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
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range*near_z, 0.0],
            ]
        }
    }

    pub fn translation(ox: f32, oy: f32, oz: f32) -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ox,  oy,  oz,  1.0],
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
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix { &self * &rhs }
}

impl<'a> Mul<Matrix> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix { self * &rhs }
}

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'a Matrix) -> Matrix { &self * rhs }
}

impl<'a, 'b> Mul<&'a Matrix> for &'b Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'a Matrix) -> Matrix {
        macro_rules! row {
            ($col:expr) => ({
                let [x, y, z, w] = self.m[$col];
                [
                    (rhs.m[0][0]*x)+(rhs.m[1][0]*y)+(rhs.m[2][0]*z)+(rhs.m[3][0]*w),
                    (rhs.m[0][1]*x)+(rhs.m[1][1]*y)+(rhs.m[2][1]*z)+(rhs.m[3][1]*w),
                    (rhs.m[0][2]*x)+(rhs.m[1][2]*y)+(rhs.m[2][2]*z)+(rhs.m[3][2]*w),
                    (rhs.m[0][3]*x)+(rhs.m[1][3]*y)+(rhs.m[2][3]*z)+(rhs.m[3][3]*w),
                ]
            })
        }

        Matrix { m: [ row!(0), row!(1), row!(2), row!(3) ] }
    }
}

impl IntoUniformValue<'static> for Matrix {
    fn into_uniform_value(self) -> UniformValue<'static> {
        UniformValue::Mat4(self.m)
    }
}
