use glium::{VertexBuffer, Program, Frame};
use glium::index::*;
use glium::backend::Facade;
use xmath::Matrix;
use traits::Unit;

pub struct Minion {
    vb: VertexBuffer<Vertex>,
    ib: NoIndices,
    program: Program,
    pos: (f32, f32),
    angle: f32,
}

#[derive(Clone, Copy)]
struct Vertex { position: [f32; 2] }

implement_vertex!(Vertex, position);

impl Minion {
    pub fn new<F: Facade>(facade: &F, pos: (f32, f32)) -> Self {
        Minion {
            vb: VertexBuffer::new(facade, {
                vec![
                    Vertex { position: [  2.0,  0.00 ] },
                    Vertex { position: [ -2.0,  0.75 ] },
                    Vertex { position: [ -2.0, -0.75 ] },
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
                    color = vec3(1.0, 0.5, 0.5);
                }
            "#, None).unwrap(),
            pos: pos,
            angle: 0.0,
        }
    }
}

impl Unit for Minion {
    fn update(&mut self, _elapsed: f32) {
        // Does nothing
    }

    fn draw(&self, mut target: Frame, camera: Matrix) -> Frame {
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
}


/// 미니언을 조종하는 객체
pub struct MinionController {
    minions: Vec<Minion>
}

impl MinionController {
    pub fn new<F: Facade>(facade: &F) -> Self {
        MinionController {
            minions: vec![
                Minion::new(facade, (17.0, 4.0)),
                Minion::new(facade, (19.0, 2.0)),
                Minion::new(facade, (20.0, 0.0)),
                Minion::new(facade, (19.0,-2.0)),
                Minion::new(facade, (17.0,-4.0)),
            ]
        }
    }
}

impl Unit for MinionController {
    fn update(&mut self, elapsed: f32) {
        for minion in &mut self.minions {
            minion.update(elapsed);
        }
    }

    fn draw(&self, target: Frame, camera: Matrix) -> Frame {
        self.minions.iter().fold(target, |target, ref minion| {
            minion.draw(target, camera.clone())
        })
    }
}
