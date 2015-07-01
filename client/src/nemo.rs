use glium::{VertexBuffer, Program, Frame};
use glium::index::*;
use glium::backend::Facade;
use math::Matrix;

pub struct Nemo {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    state: State,
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

enum State {
    /// Nemo is stopped
    Stopped { pos: (f64, f64) },
    /// Nemo is moving (0 <= t < 1)
    Moving { src: (f64, f64), dest: (f64, f64), t: f64 }
}

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
            state: State::Stopped { pos: (0.0, 0.0) },
        }
    }

    pub fn update(&mut self, elapsed: f64) {
        let mut next = None;

        match self.state {
            State::Stopped { .. } => {}
            State::Moving { dest, ref mut t, .. } => {
                *t += elapsed / 0.5;
                if 1.0 <= *t { next = Some(State::Stopped { pos: dest }); }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    fn current_pos(&self) -> (f64, f64) {
        match self.state {
            State::Stopped { pos } => pos,
            State::Moving { src, dest, t } => {
                (src.0*(1.0 - t) + dest.0*t, src.1*(1.0 - t) + dest.1*t)
            }
        }
    }

    pub fn draw(&self, mut target: Frame, world: Matrix) -> Frame {
        use glium::Surface;

        let pos = self.current_pos();
        let uniforms = uniform! {
            pos: (pos.0 as f32, pos.1 as f32), // <- (f64, f64) doesn't implement AsUniformValue
            matrix: world
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()).unwrap();
        target
    }

    pub fn go(&mut self, dest: (f64, f64)) {
        let pos = self.current_pos();
        self.state = State::Moving { src: pos, dest: dest, t: 0.0 };
    }
}
