use math::{Vector, Matrix};

pub struct Camera {
    eye: Vector, focus: Vector, up: Vector,
    dirty: bool, matrix: Matrix
}

impl Camera {
    pub fn new(eye: Vector, focus: Vector, up: Vector) -> Self {
        Camera {
            eye: eye, focus: focus, up: up,
            dirty: true, matrix: Matrix::new()
        }
    }

    pub fn matrix(&mut self) -> &Matrix {
        if self.dirty {
            self.matrix = Matrix::look_at(self.eye, self.focus, self.up);
            self.dirty = false;
        }

        &self.matrix
    }
}
