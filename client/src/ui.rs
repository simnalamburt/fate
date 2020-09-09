use glium::backend::Facade;
use glium::index;
use glium::{DrawError, Program, Surface, VertexBuffer};
use xmath::Matrix;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct UI {
    pub cursor: (f32, f32),
    pub width: u32,
    pub height: u32,
    vb: VertexBuffer<Vertex>,
    ib: index::NoIndices,
    program: Program,
    matrix: Matrix,
}

impl UI {
    pub fn new<F>(display: &F, width: u32, height: u32) -> UI
    where
        F: Facade,
    {
        let vb: VertexBuffer<Vertex> = VertexBuffer::new(display, &{
            vec![
                Vertex {
                    position: [-2.0, -2.0],
                },
                Vertex {
                    position: [-2.0, 3.0],
                },
                Vertex {
                    position: [3.0, -2.0],
                },
                Vertex {
                    position: [3.0, 3.0],
                },
            ]
        })
        .unwrap();
        let ib = index::NoIndices(index::PrimitiveType::TriangleStrip);
        let program = Program::from_source(
            display,
            r#"
            #version 410
            uniform vec2 cursor;
            uniform mat4 matrix;
            in vec2 position;

            void main() {
                gl_Position = matrix * vec4(position + cursor, 0.0, 1.0);
            }
        "#,
            r#"
            #version 410
            out vec3 color;

            void main() {
                color = vec3(1.0, 1.0, 1.0);
            }
        "#,
            None,
        )
        .unwrap();
        let matrix =
            Matrix::orthographic_off_center(0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        UI {
            cursor: (300.0, 300.0),
            width: width,
            height: height,
            vb: vb,
            ib: ib,
            program: program,
            matrix: matrix,
        }
    }

    pub fn draw<S>(&self, target: &mut S) -> Result<(), DrawError>
    where
        S: Surface,
    {
        let uniforms = uniform! {
            cursor: self.cursor,
            matrix: self.matrix.clone()
        };

        target.draw(
            &self.vb,
            &self.ib,
            &self.program,
            &uniforms,
            &Default::default(),
        )
    }

    pub fn move_cursor(&mut self, x: i32, y: i32) {
        self.cursor = (x as f32, (self.height as i32 - y) as f32)
    }

    pub fn cursor_on_game_coordinate(&self) -> (f32, f32) {
        (
            (self.cursor.0 - self.width as f32 / 2.0) / 10.0,
            (self.cursor.1 - self.height as f32 / 2.0) / 10.0,
        )
    }
}
