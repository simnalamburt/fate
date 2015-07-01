use glium::{VertexBuffer, Program, Frame};
use glium::index::*;
use glium::backend::Facade;
use math::Matrix;

pub struct Nemo {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32),
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

impl Nemo {
    pub fn new<F: Facade>(facade: &F) -> Self {
        Nemo {
            vb: VertexBuffer::new(facade, {
                vec![
                    Vertex { position: [ -10.0, -10.0 ] },
                    Vertex { position: [ -10.0,  10.0 ] },
                    Vertex { position: [  10.0, -10.0 ] },
                    Vertex { position: [  10.0,  10.0 ] },
                ]
            }),
            ib: NoIndices(PrimitiveType::TriangleStrip),
            program: Program::from_source(facade, r#"
                #version 410
                uniform vec2 pos;
                uniform mat4 matrix;
                in vec2 position;

                void main() {
                    gl_Position = matrix * vec4(position + pos, 0.0, 1.0);
                }
            "#, r#"
                #version 410
                out vec3 color;

                void main() {
                    color = vec3(1.0, 0.82745, 0.14118);
                }
            "#, None).unwrap(),
            pos: (0.0, 0.0)
        }
    }

    pub fn draw(&self, mut target: Frame, world: Matrix) -> Frame {
        use glium::Surface;

        let uniforms = uniform! {
            pos: self.pos,
            matrix: world
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()).unwrap();
        target
    }

    pub fn go(&mut self, dest: (f32, f32)) {
        self.pos = dest;
    }
}
