use glium::{Frame, DrawError};
use glium::framebuffer::SimpleFrameBuffer;
use xmath::Matrix;

pub trait Object {
    fn update(&mut self, elapsed: f32);
    fn draw(&self, target: &mut Frame, camera: &Matrix) -> Result<(), DrawError>;
    fn fill(&self, target: &mut SimpleFrameBuffer, camera: &Matrix) -> Result<(), DrawError>;
}

pub trait Move: Object {
    fn go(&mut self, dest: (f32, f32));
}
