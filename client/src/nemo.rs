use glium::{VertexBuffer, Program, Frame};
use glium::index::*;
use glium::backend::Facade;
use xmath::Matrix;

pub struct Nemo {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32),
    angle: f32,
    state: State,
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

enum State {
    /// Nemo is stopped
    Stopped,
    /// Nemo is moving
    Moving { dest: (f32, f32) },
}

impl Nemo {
    pub fn new<F: Facade>(facade: &F) -> Self {
        Nemo {
            vb: VertexBuffer::new(facade, {
                vec![
                    Vertex { position: [   4.0,   0.0 ] },
                    Vertex { position: [  -4.0,   1.5 ] },
                    Vertex { position: [  -4.0,  -1.5 ] },
                ]
            }),
            ib: NoIndices(PrimitiveType::TriangleStrip),
            program: Program::from_source(facade, r#"
                #version 410
                uniform mat4 matrix;
                in vec2 position;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            "#, r#"
                #version 410
                out vec3 color;

                void main() {
                    color = vec3(1.0, 0.82745, 0.14118);
                }
            "#, None).unwrap(),
            pos: (0.0, 0.0),
            angle: 0.0,
            state: State::Stopped,
        }
    }

    pub fn update(&mut self, elapsed: f32) {
        let mut next = None;

        match self.state {
            State::Stopped => {}
            State::Moving { dest } => {
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
                    self.pos.0 += diff*self.angle.cos();
                    self.pos.1 += diff*self.angle.sin();
                }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    pub fn draw(&self, mut target: Frame, camera: Matrix) -> Frame {
        use glium::Surface;

        // TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        let uniforms = uniform! {
            matrix: local * world * camera,
        };

        target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()).unwrap();
        target
    }

    pub fn go(&mut self, dest: (f32, f32)) {
        if self.pos == dest { return; }

        let dx = dest.0 - self.pos.0;
        let dy = dest.1 - self.pos.1;
        self.angle = dy.atan2(dx);
        self.state = State::Moving { dest: dest };
    }
}
