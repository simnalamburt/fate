use glium::Frame;
use xmath::Matrix;

pub trait Unit {
    fn update(&mut self, elapsed: f32);
    fn draw(&self, mut target: Frame, camera: Matrix) -> Frame;
}
