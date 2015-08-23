use glium::{DrawError, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::index;
use xmath::Matrix;

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

pub struct UI {
    pub cursor: (f32, f32),
    vb: VertexBuffer<Vertex>,
    ib: index::NoIndices,
    program: Program,
    matrix: Matrix,
}

impl UI {
    pub fn new<F>(display: &F, width: f32, height: f32) -> UI where F: Facade {
        let vb: VertexBuffer<Vertex> = VertexBuffer::new(display, &{

            vec![
                Vertex { position: [ -2.0, -2.0 ] },
                Vertex { position: [ -2.0,  3.0 ] },
                Vertex { position: [  3.0, -2.0 ] },
                Vertex { position: [  3.0,  3.0 ] },
            ]
        }).unwrap();
        let ib = index::NoIndices(index::PrimitiveType::TriangleStrip);
        let program = Program::from_source(display,
                                                  r#"
            #version 410
            uniform vec2 cursor;
            uniform mat4 matrix;
            in vec2 position;

            void main() {
                gl_Position = matrix * vec4(position + cursor, 0.0, 1.0);
            }
        "#, r#"
            #version 410
            out vec3 color;

            void main() {
                color = vec3(1.0, 1.0, 1.0);
            }
        "#, None).unwrap();
        let matrix = Matrix::orthographic_off_center(0.0, width, 0.0, height, 0.0, 1.0);
        UI {
            cursor: (300.0, 300.0),
            vb: vb,
            ib: ib,
            program: program,
            matrix: matrix,
        }
    }

    pub fn draw<S>(&self, target: &mut S) -> Result<(), DrawError> where S: Surface {
        let uniforms = uniform! {
            cursor: self.cursor,
            matrix: self.matrix.clone()
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default())
    }
}
