use glium::Frame;
use xmath::Matrix;

pub trait Object {
    fn update(&mut self, elapsed: f32);
    fn draw(&self, mut target: Frame, camera: Matrix) -> Frame;
}

pub trait Unit: Object {
    fn go(&mut self, dest: (f32, f32));
}
