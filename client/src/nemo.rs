use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::*;
use glium::backend::Facade;
use xmath::Matrix;
use traits::*;
use error::CreationError;

pub struct Nemo {
    //position: Position2D,
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
    /// Nemo is using Q (0 <= t < 1)
    QSkill { t: f32 }
}

impl Nemo {
    pub fn new<F: Facade>(facade: &F) -> Result<Self, CreationError> {
        Ok(Nemo {
            vb: try!(VertexBuffer::new(facade, &{
                vec![
                    Vertex { position: [   4.0,   0.0 ] },
                    Vertex { position: [  -4.0,   1.5 ] },
                    Vertex { position: [  -4.0,  -1.5 ] },
                ]
            })),
            ib: NoIndices(PrimitiveType::TriangleStrip),
            program: try!(Program::from_source(facade, r#"
                #version 410
                uniform mat4 matrix;
                in vec2 position;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            "#, r#"
                #version 410
                uniform int q;
                out vec3 color;

                void main() {
                    if (q == 1) {
                        color = vec3(0.533333, 0.4, 1.0);
                    } else {
                        color = vec3(1.0, 0.82745, 0.14118);
                    }
                }
            "#, None)),
            pos: (0.0, 0.0),
            angle: 0.0,
            state: State::Stopped,
        })
    }
}

impl Object for Nemo {
    fn update(&mut self, elapsed: f32) {
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
            State::QSkill { ref mut t } => {
                *t += elapsed;

                if 1.0 <= *t {
                    next = Some(State::Stopped);
                }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    fn draw(&self, target: &mut Frame, camera: &Matrix) -> Result<(), DrawError> {
        use glium::Surface;

        // TODO: Cache
        let local = Matrix::rotation_z(self.angle);
        let world = Matrix::translation(self.pos.0, self.pos.1, 0.0);

        let uniforms = uniform! {
            matrix: local * world * camera,
            q: match self.state { State::QSkill { .. } => 1, _ => 0 }
        };

        try!(target.draw(&self.vb, &self.ib, &self.program, &uniforms, &Default::default()));
        Ok(())
    }
}

impl Unit for Nemo {
    fn go(&mut self, dest: (f32, f32)) {
        match self.state {
            State::QSkill { .. } => return,
            _ => ()
        }

        if self.pos == dest { return; }

        let dx = dest.0 - self.pos.0;
        let dy = dest.1 - self.pos.1;
        self.angle = dy.atan2(dx);
        self.state = State::Moving { dest: dest };
    }
}

impl Nemo {
    pub fn q(&mut self) {
        self.state = State::QSkill { t: 0.0 };
    }
}
