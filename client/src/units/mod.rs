mod nemo;
mod minion;

use error::CreationError;
use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::backend::Facade;
use glium::draw_parameters::DrawParameters;
use glium::draw_parameters::StencilOperation;
use glium::draw_parameters::StencilTest;
use glium::index::NoIndices;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use std::sync::atomic::ATOMIC_USIZE_INIT;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use xmath::Matrix;
pub use self::nemo::Nemo;
pub use self::minion::{Minion, MinionController};

type Position = (f32, f32);
type Id = usize;

struct Unit {
    id: Id,
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: Position,
    angle: f32,
    cooldown: f32,
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
    fn new<'a, F: Facade>(facade: &F, vertices: &[Vertex], indices: NoIndices, vertex_shader: &'a str, fragment_shader: &'a str, position: Position) -> Result<Self, CreationError> {
        static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);;
        Ok(Unit {
            id: id,
            vb: try!(VertexBuffer::new(facade, vertices)),
            ib: indices,
            program: try!(Program::from_source(facade, vertex_shader, fragment_shader, None)),
            pos: position,
            angle: 0.0,
            cooldown: 0.0,
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
            stencil_test_clockwise: StencilTest::AlwaysPass,
            stencil_test_counter_clockwise: StencilTest::AlwaysPass,
            stencil_reference_value_clockwise: self.id as i32,
            stencil_reference_value_counter_clockwise: self.id as i32,
            stencil_pass_depth_fail_operation_clockwise: StencilOperation::Keep,
            stencil_pass_depth_fail_operation_counter_clockwise: StencilOperation::Keep,
            stencil_depth_pass_operation_clockwise: StencilOperation::Replace,
            stencil_depth_pass_operation_counter_clockwise: StencilOperation::Replace,
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
            stencil_test_clockwise: StencilTest::AlwaysPass,
            stencil_test_counter_clockwise: StencilTest::AlwaysPass,
            stencil_reference_value_clockwise: self.id as i32,
            stencil_reference_value_counter_clockwise: self.id as i32,
            stencil_pass_depth_fail_operation_clockwise: StencilOperation::Keep,
            stencil_pass_depth_fail_operation_counter_clockwise: StencilOperation::Keep,
            stencil_depth_pass_operation_clockwise: StencilOperation::Replace,
            stencil_depth_pass_operation_counter_clockwise: StencilOperation::Replace,
            .. Default::default()
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &draw_parameters)
    }
}
