use glium::{VertexBuffer, Program, Frame};
use glium::index::*;
use glium::backend::Facade;
use math::Matrix;

pub struct Nemo {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: (f64, f64),
    state: State,
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

enum State {
    /// Nemo is stopped
    Stopped,
    /// Nemo is moving
    Moving { dest: (f64, f64), theta: f64 },
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
            pos: (0.0, 0.0),
            state: State::Stopped,
        }
    }

    pub fn update(&mut self, elapsed: f64) {
        let mut next = None;

        match self.state {
            State::Stopped => {}
            State::Moving { dest, theta } => {
                let dx = dest.0 - self.pos.0;
                let dy = dest.1 - self.pos.1;

                let left_dist = (dx*dx + dy*dy).sqrt();

                let speed = 50.0;
                let diff = speed*elapsed;

                if left_dist <= diff {
                    // 도착
                    self.pos = dest;
                    next = Some(State::Stopped);
                } else {
                    self.pos.0 += diff*theta.cos();
                    self.pos.1 += diff*theta.sin();
                }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    pub fn draw(&self, mut target: Frame, world: Matrix) -> Frame {
        use glium::Surface;

        let uniforms = uniform! {
            // Note: (f64, f64) doesn't implement AsUniformValue
            pos: (self.pos.0 as f32, self.pos.1 as f32),
            matrix: world,
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()).unwrap();
        target
    }

    pub fn go(&mut self, dest: (f64, f64)) {
        let dx = dest.0 - self.pos.0;
        let dy = dest.1 - self.pos.1;

        let theta = dy.atan2(dx);
        self.state = State::Moving { dest: dest, theta: theta };
    }
}
