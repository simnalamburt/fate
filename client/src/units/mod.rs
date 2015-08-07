mod nemo;
mod minion;

use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::NoIndices;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use xmath::Matrix;
pub use self::nemo::Nemo;
pub use self::minion::{Minion, MinionController};

struct Unit {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32),
    angle: f32
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2]
}

fn vec(x: f32, y: f32) -> Vertex {
    Vertex { position: [x, y] }
}

implement_vertex!(Vertex, position);

impl Unit {
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
        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default())
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
        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default())
    }
}
