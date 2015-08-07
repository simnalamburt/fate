use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::Facade;
use xmath::Matrix;
use traits::{Object, Move};
use error::CreationError;
use unit::{vec, Unit};

pub struct Minion {
    unit: Unit,
    state: State,
}

enum State {
    /// Nemo is stopped
    Stopped { time: f32 },
    /// Nemo is moving
    Moving { dest: (f32, f32) },
}

impl Minion {
    pub fn new<F: Facade>(facade: &F, pos: (f32, f32)) -> Result<Self, CreationError> {
        let unit = Unit {
            vb: try!(VertexBuffer::new(facade, &{
                vec![
                    vec(  2.0,  0.00 ),
                    vec( -2.0,  0.75 ),
                    vec( -2.0, -0.75 ),
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
                out vec3 color;

                void main() {
                    color = vec3(1.0, 0.5, 0.5);
                }
            "#, None)),
            pos: pos,
            angle: 0.0
        };

        Ok(Minion { unit: unit, state: State::Stopped { time: 0.0 } })
    }
}

impl Object for Minion {
    fn update(&mut self, elapsed: f32) {
        let mut next = None;

        match self.state {
            State::Stopped { ref mut time } => {
                *time += elapsed;
            }
            State::Moving { dest } => {
                let unit = &mut self.unit;

                let dx = dest.0 - unit.pos.0;
                let dy = dest.1 - unit.pos.1;

                let left_dist = (dx*dx + dy*dy).sqrt();

                let speed = 50.0;
                let diff = speed*elapsed;

                if left_dist <= diff {
                    // 도착
                    unit.pos = dest;
                    next = Some(State::Stopped { time: 0.0 });
                } else {
                    unit.pos.0 += diff*unit.angle.cos();
                    unit.pos.1 += diff*unit.angle.sin();
                }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    fn draw(&self, target: &mut Frame, camera: &Matrix) -> Result<(), DrawError> {
        self.unit.draw_without_uniforms(target, camera)
    }
}

impl Move for Minion {
    fn go(&mut self, dest: (f32, f32)) {
        let unit = &mut self.unit;
        if unit.pos == dest { return; }

        let dx = dest.0 - unit.pos.0;
        let dy = dest.1 - unit.pos.1;
        unit.angle = dy.atan2(dx);
        self.state = State::Moving { dest: dest };
    }
}


/// 미니언을 조종하는 객체
pub struct MinionController {
    minions: Vec<Minion>
}

impl MinionController {
    pub fn new<F: Facade>(facade: &F) -> Result<Self, CreationError> {
        Ok(MinionController {
            minions: vec![
                try!(Minion::new(facade, (17.0, 4.0))),
                try!(Minion::new(facade, (19.0, 2.0))),
                try!(Minion::new(facade, (20.0, 0.0))),
                try!(Minion::new(facade, (19.0,-2.0))),
                try!(Minion::new(facade, (17.0,-4.0))),
            ]
        })
    }
}

impl Object for MinionController {
    fn update(&mut self, elapsed: f32) {
        for minion in &mut self.minions {
            match minion.state {
                State::Stopped { time } if 1.5 <= time => {
                    use rand;
                    use rand::distributions::{IndependentSample, Range};

                    let range = Range::new(-10.0, 10.0);
                    let mut rng = rand::thread_rng();

                    let x = range.ind_sample(&mut rng);
                    let y = range.ind_sample(&mut rng);

                    minion.go((x, y));
                }
                _ => {}
            }

            minion.update(elapsed);
        }
    }

    fn draw(&self, target: &mut Frame, camera: &Matrix) -> Result<(), DrawError> {
        for minion in &self.minions {
            try!(minion.draw(target, &camera))
        }
        Ok(())
    }
}
