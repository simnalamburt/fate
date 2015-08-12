mod nemo;
mod minion;

use error::CreationError;
use glium::{VertexBuffer, IndexBuffer, Program, Frame, DrawError};
use glium::backend::Facade;
use glium::draw_parameters::DrawParameters;
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
    fn new<'a, F: Facade>(facade: &F,
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
            pos: position,
            angle: 0.0,
        })
    }

    fn draw<'n, T, R>(&self,
                          target: &mut Frame,
                          camera: &Matrix,
                          uniforms: UniformsStorage<'n, T, R>)
        -> Result<(), DrawError> where T: AsUniformValue, R: Uniforms
    {
        use glium::Surface;

        // TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        let uniforms = uniforms.add("matrix", local * world * camera);

        let draw_parameters = DrawParameters {
            .. Default::default()
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &draw_parameters)
    }

    fn draw_without_uniforms(&self,
                                 target: &mut Frame,
                                 camera: &Matrix)
        -> Result<(), DrawError>
    {
        use glium::Surface;

        // TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        let uniforms = uniform! { matrix: local * world * camera };

        let draw_parameters = DrawParameters {
            .. Default::default()
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &draw_parameters)
    }
}
