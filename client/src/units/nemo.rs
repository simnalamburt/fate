use super::Unit;
use crate::draw_context::DrawContext;
use crate::error::CreationError;
use crate::resource::load_obj;
use crate::traits::{Move, Object};
use glium::backend::Facade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::{DrawError, Frame};

pub struct Nemo {
    unit: Unit,
    state: State,
}

enum State {
    /// Nemo is stopped
    Stopped,
    /// Nemo is moving
    Moving { dest: (f32, f32) },
    /// Nemo is using Q (0 <= t < 1)
    QSkill { t: f32 },
}

impl Nemo {
    pub fn new<F: Facade>(facade: &F) -> Result<Self, CreationError> {
        let bear = load_obj("rilakkuma")?;

        let unit = Unit::new(
            facade,
            bear.vertex_buffer(facade)?,
            bear.index_buffer(facade)?,
            r#"
                #version 410
                uniform mat4 matrix;
                in vec3 position;

                void main() {
                    gl_Position = matrix * vec4(position, 1.0);
                }
            "#,
            r#"
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
            "#,
            (0.0, 0.0),
        )?;

        Ok(Nemo {
            unit,
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
                let unit = &mut self.unit;

                let dx = dest.0 - unit.pos.0;
                let dy = dest.1 - unit.pos.1;

                let left_dist = (dx * dx + dy * dy).sqrt();

                let speed = 50.0;
                let diff = speed * elapsed;

                if left_dist <= diff {
                    // 도착
                    unit.pos = dest;
                    next = Some(State::Stopped);
                } else {
                    unit.pos.0 += diff * unit.angle.cos();
                    unit.pos.1 += diff * unit.angle.sin();
                }
            }
            State::QSkill { ref mut t } => {
                *t += elapsed;

                if 1.0 <= *t {
                    next = Some(State::Stopped);
                }
            }
        };

        if let Some(next) = next {
            self.state = next;
        }
    }

    fn draw(&self, target: &mut Frame, draw_context: &DrawContext) -> Result<(), DrawError> {
        let uniforms = uniform! {
            q: match self.state { State::QSkill { .. } => 1, _ => 0 }
        };
        self.unit.draw(target, uniforms, draw_context)
    }

    fn fill(
        &self,
        target: &mut SimpleFrameBuffer,
        draw_context: &DrawContext,
    ) -> Result<(), DrawError> {
        self.unit.fill(target, draw_context)
    }
}

impl Move for Nemo {
    fn go(&mut self, dest: (f32, f32)) {
        if let State::QSkill { .. } = self.state {
            return;
        }

        let unit = &mut self.unit;
        if unit.pos == dest {
            return;
        }

        let dx = dest.0 - unit.pos.0;
        let dy = dest.1 - unit.pos.1;
        unit.angle = dy.atan2(dx);
        self.state = State::Moving { dest };
    }
}

impl Nemo {
    pub fn q(&mut self) {
        self.state = State::QSkill { t: 0.0 };
    }
}
