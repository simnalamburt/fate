mod nemo;
mod minion;

use draw_context::DrawContext;
use error::CreationError;
use glium::{VertexBuffer, IndexBuffer, Program, Frame, DrawError, Surface};
use glium::backend::Facade;
use glium::draw_parameters::DrawParameters;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};
use xmath::Matrix;
use obj::Vertex;
pub use self::nemo::Nemo;
pub use self::minion::{Minion, MinionController};

type Position = (f32, f32);
type Id = usize;

struct Unit {
    id: Id,
    vb: VertexBuffer<Vertex>,
    ib: IndexBuffer<u16>,
    program: Program,
    fill_id_program: Program,
    pos: Position,
    angle: f32,
}

fn vec(x: f32, y: f32) -> Vertex {
    Vertex {
        position: [x, y, 0.0],
        normal: [0.0, 0.0, 0.0],
    }
}

impl Unit {
    pub fn new<'a, F: Facade>(facade: &F,
                          vertex_buffer: VertexBuffer<Vertex>,
                          index_buffer: IndexBuffer<u16>,
                          vertex_shader: &'a str,
                          fragment_shader: &'a str,
                          position: Position)
        -> Result<Self, CreationError>
    {
        static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);;
        Ok(Unit {
            id: id,
            vb: vertex_buffer,
            ib: index_buffer,
            program: try!(Program::from_source(facade, vertex_shader, fragment_shader, None)),
            fill_id_program: try!(Program::from_source(facade, r#"
            #version 410
            uniform mat4 matrix;
            in vec3 position;
            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }"#, r#"
            #version 410
            uniform vec4 id;
            out vec4 color;
            void main() {
                color = id;
            }"#, None)),
            pos: position,
            angle: 0.0,
        })
    }

    fn draw<'n, T, R>(&self,
                      target: &mut Frame,
                      uniforms: UniformsStorage<'n, T, R>,
                      draw_context: &DrawContext)
        -> Result<(), DrawError> where T: AsUniformValue, R: Uniforms
    {
        // TODO: Cache
        let uniforms = uniforms.add("matrix", matrix(self, &draw_context.camera));
        draw_internal(target, &self, &self.program, &uniforms)
    }

    fn draw_without_uniforms(&self,
                             target: &mut Frame,
                             draw_context: &DrawContext)
        -> Result<(), DrawError>
    {
        // TODO: Cache
        let uniforms = uniform! { matrix: matrix(self, &draw_context.camera) };
        draw_internal(target, &self, &self.program, &uniforms)
    }

    fn fill(&self, target: &mut SimpleFrameBuffer, draw_context: &DrawContext) -> Result<(), DrawError> {
        let red = ((self.id >> 24) & 0xFF) as f32 / 255.0;
        let green = ((self.id >> 16) & 0xFF) as f32 / 255.0;
        let blue = ((self.id >> 8) & 0xFF) as f32 / 255.0;
        let alpha = (self.id & 0xFF) as f32 / 255.0;
        let uniforms = uniform! { matrix: matrix(self, &draw_context.camera), id: [red, green, blue, alpha] };
        draw_internal(target, &self, &self.fill_id_program, &uniforms)
    }
}

fn matrix(unit: &Unit, camera:&Matrix) -> Matrix {
    let local = Matrix::rotation_z(unit.angle);
    let world = Matrix::translation(unit.pos.0, unit.pos.1, 0.0);

    local * world * camera
}

fn draw_internal<S, U>(target: &mut S, unit: &Unit, program: &Program, uniforms: &U) -> Result<(), DrawError> where S: Surface, U: Uniforms {
    let draw_parameters = DrawParameters {
        .. Default::default()
    };

    target.draw(&unit.vb, &unit.ib, program, uniforms, &draw_parameters)
}
