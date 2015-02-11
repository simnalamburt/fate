extern crate glium;

pub use math::Matrix;

mod math {
    use glium::uniforms::*;

    type Pos = (f32, f32, f32);
    type Dir = (f32, f32, f32);

    pub struct Matrix {
        m: [f32; 16]
    }

    impl Matrix {
        pub fn one() -> Self {
            Matrix {
                m: [
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]
            }
        }
    }

    impl IntoUniformValue<'static> for Matrix {
        fn into_uniform_value(self) -> UniformValue<'static> {
            UniformValue::Mat4([
                [self.m[00], self.m[04], self.m[08], self.m[12]],
                [self.m[01], self.m[05], self.m[09], self.m[13]],
                [self.m[02], self.m[06], self.m[10], self.m[14]],
                [self.m[03], self.m[07], self.m[11], self.m[15]],
            ])
        }
    }
}
