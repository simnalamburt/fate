use draw_context::DrawContext;
use glium::{Frame, DrawError};
use glium::framebuffer::SimpleFrameBuffer;

pub trait Object {
    fn update(&mut self, elapsed: f32);
    fn draw(&self, target: &mut Frame, draw_context: &DrawContext) -> Result<(), DrawError>;
    fn fill(&self, target: &mut SimpleFrameBuffer, draw_context: &DrawContext) -> Result<(), DrawError>;
}

pub trait Move: Object {
    fn go(&mut self, dest: (f32, f32));
}
