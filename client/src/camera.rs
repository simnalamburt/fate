use math::{vec, Vector, Matrix};

pub struct Camera {
    focus: Vector,
    dir: Vector,
    dist: f32,

    dirty: bool,
    matrix: Matrix
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            focus: vec(0.0, 0.0, 4.0),
            dir: vec(-0.5773502691896258, 0.5773502691896258, -0.5773502691896258),
            dist: 34.64101615137754,

            dirty: true,
            matrix: Matrix::new()
        }
    }

    pub fn matrix(&mut self) -> &Matrix {
        if self.dirty {
            self.matrix = Matrix::look_at(
                self.focus - self.dir * vec(self.dist, self.dist, self.dist),
                self.focus,
                vec(0.0, 0.0, 1.0));
            self.dirty = false;
        }

        &self.matrix
    }

    pub fn zoom(&mut self, delta: f64) {
        self.dirty = true;
        self.dist += (delta as f32) * 0.01
    }
}
