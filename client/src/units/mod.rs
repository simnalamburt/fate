mod minion;
mod nemo;

pub use self::minion::{Minion, MinionController};
pub use self::nemo::Nemo;
use crate::draw_context::DrawContext;
use crate::error::CreationError;
use glium::backend::Facade;
use glium::draw_parameters::DrawParameters;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use glium::{DrawError, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use obj::Vertex;
use std::sync::atomic::{AtomicUsize, Ordering};
use xmath::Matrix;

type Position = (f32, f32);
type Id = usize;

struct Unit {
    id: Id,
    vb: VertexBuffer<Vertex>,
    ib: IndexBuffer<u16>,
    program: Program,
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
    pub fn new<'a, F: Facade>(
        facade: &F,
        vertex_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u16>,
        vertex_shader: &'a str,
        fragment_shader: &'a str,
        position: Position,
    ) -> Result<Self, CreationError> {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Ok(Unit {
            id,
            vb: vertex_buffer,
            ib: index_buffer,
            program: Program::from_source(facade, vertex_shader, fragment_shader, None)?,
            pos: position,
            angle: 0.0,
        })
    }

    fn draw<'n, T, R>(
        &self,
        target: &mut Frame,
        uniforms: UniformsStorage<'n, T, R>,
        draw_context: &DrawContext,
    ) -> Result<(), DrawError>
    where
        T: AsUniformValue,
        R: Uniforms,
    {
        // TODO: Cache
        let uniforms = uniforms.add("matrix", matrix(self, &draw_context.camera));
        draw_internal(target, &self, &self.program, &uniforms)
    }

    fn draw_without_uniforms(
        &self,
        target: &mut Frame,
        draw_context: &DrawContext,
    ) -> Result<(), DrawError> {
        // TODO: Cache
        let uniforms = uniform! { matrix: matrix(self, &draw_context.camera) };
        draw_internal(target, &self, &self.program, &uniforms)
    }

    fn fill(
        &self,
        target: &mut SimpleFrameBuffer,
        draw_context: &DrawContext,
    ) -> Result<(), DrawError> {
        let red = ((self.id >> 24) & 0xFF) as f32 / 255.0;
        let green = ((self.id >> 16) & 0xFF) as f32 / 255.0;
        let blue = ((self.id >> 8) & 0xFF) as f32 / 255.0;
        let alpha = (self.id & 0xFF) as f32 / 255.0;
        let uniforms =
            uniform! { matrix: matrix(self, &draw_context.camera), id: [red, green, blue, alpha] };
        draw_internal(target, &self, &draw_context.fill_id_program, &uniforms)
    }
}

fn matrix(unit: &Unit, camera: &Matrix) -> Matrix {
    let local = Matrix::rotation_z(unit.angle);
    let world = Matrix::translation(unit.pos.0, unit.pos.1, 0.0);

    local * world * camera
}

fn draw_internal<S, U>(
    target: &mut S,
    unit: &Unit,
    program: &Program,
    uniforms: &U,
) -> Result<(), DrawError>
where
    S: Surface,
    U: Uniforms,
{
    let draw_parameters = DrawParameters {
        ..Default::default()
    };

    target.draw(&unit.vb, &unit.ib, program, uniforms, &draw_parameters)
}
