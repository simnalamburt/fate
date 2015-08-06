use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::*;
use glium::backend::Facade;
use glium::uniforms::UniformStorage;
use xmath::Matrix;
use traits::*;

pub struct Position2D {
	vb: VertexBuffer<Vertex2D>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32),
    angle: f32
}

struct Vertex2D { position: [f32; 2] }
implement_vertex!(Vertex2D, position);

pub struct Position3D {
	vb: VertexBuffer<Vertex3D>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32), // 3D ?
    angle: f32 // Quaternion ?
}

struct Vertex3D { position: [f32; 3] }
implement_vertex!(Vertex3D, position);

impl Position2D {
	pub fn draw(&self, target: &mut Frame, camera: &Matrix, uniforms: Option<UniformStorage>) 
	-> Result<(), DrawError> {
		use glium::Surface;

		// TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        let uniforms = if let Some(x) = uniforms {
        	x.add("matrix", local * world * camera)
        } else {
        	uniform! { matrix: local * world * camera, }
        }

        try!(target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()));
        Ok(())
	}
}