use xmath::Matrix;

pub struct DrawContext {
    pub camera: Matrix,
}

impl DrawContext {
    pub fn new(width: f32, height: f32) -> DrawContext {
        DrawContext {
            camera: Matrix::orthographic(width/10.0, height/10.0, 0.0, 1.0),
        }
    }
}
