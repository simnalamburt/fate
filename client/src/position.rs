use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::*;
use glium::uniforms::*;
use xmath::Matrix;
use traits::*;

pub struct Position2D {
	pub vb: VertexBuffer<Vertex2D>,
    pub ib: NoIndices,
    pub program: Program,
    pub pos: (f32, f32),
    pub angle: f32
}

#[derive(Clone, Copy)]
pub struct Vertex2D { pub position: [f32; 2] }
implement_vertex!(Vertex2D, position);

#[allow(dead_code)]
pub struct Position3D {
	vb: VertexBuffer<Vertex3D>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32), // 3D ?
    angle: f32 // Quaternion ?
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vertex3D { position: [f32; 3] }
implement_vertex!(Vertex3D, position);

impl Position2D {
	pub fn draw<'n, T, R>(&self, target: &mut Frame, camera: &Matrix, uniforms: Option<UniformsStorage<'n, T, R>>) 
	-> Result<(), DrawError> where T: AsUniformValue, R: Uniforms {
		use glium::Surface;

		// TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        if let Some(u) = uniforms {
        	let u = u.add("matrix", local * world * camera);
        	try!(target.draw(&self.vb, &self.ib, &self.program, &u, &Default::default()));
        } else {
        	let u = uniform! { matrix: local * world * camera, };
        	try!(target.draw(&self.vb, &self.ib, &self.program, &u, &Default::default()));
        };

        Ok(())
	}
}