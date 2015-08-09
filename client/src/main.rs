extern crate common;
extern crate time;
extern crate xmath;
#[macro_use] extern crate glium;
extern crate rand;

mod traits;
mod error;
mod units;

use std::default::Default;
use time::PreciseTime;
use units::{Nemo, Minion, MinionController};

#[allow(dead_code)]
fn main() {
    use glium::DisplayBuild;

    let depth_bits_candidates: [u8; 3] = [ 32, 24, 16 ];
    fn build_display(depth_size: u8) -> Option<glium::backend::glutin_backend::GlutinFacade> {
        glium::glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_depth_buffer(depth_size)
            .with_title(common::PROJECT_NAME.to_string())
            .build_glium().ok()
    }
    let display = (|| -> Option<glium::backend::glutin_backend::GlutinFacade> {
        for depth_bits in depth_bits_candidates.iter() {
            match build_display(*depth_bits) {
                Some(display) => return Some(display),
                None => continue
            }
        }
        None
    })().unwrap();

    //
    // Basics
    //
    let (width, height) = {
        let dim = display.get_framebuffer_dimensions();
        (dim.0 as f32, dim.1 as f32)
    };


    //
    // Game
    //
    let mut nemo = Nemo::new(&display).unwrap();
    let mut minions = vec![
        Minion::new(&display, (-17.0, 4.0)).unwrap(),
        Minion::new(&display, (-19.0, 2.0)).unwrap(),
        Minion::new(&display, (-20.0, 0.0)).unwrap(),
        Minion::new(&display, (-19.0,-2.0)).unwrap(),
        Minion::new(&display, (-17.0,-4.0)).unwrap(),
    ];
    let mut controller = MinionController::new(&display).unwrap();
    let camera = xmath::Matrix::orthographic(width/10.0, height/10.0, 0.0, 1.0);


    //
    // Parameters for UI
    //
    let vb_ui = glium::VertexBuffer::new(&display, &{
        #[derive(Clone, Copy)]
        struct Vertex { position: [f32; 2] }

        implement_vertex!(Vertex, position);

        vec![
            Vertex { position: [ -2.0, -2.0 ] },
            Vertex { position: [ -2.0,  3.0 ] },
            Vertex { position: [  3.0, -2.0 ] },
            Vertex { position: [  3.0,  3.0 ] },
        ]
    }).unwrap();
    let ib_ui = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program_ui = glium::Program::from_source(&display,
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
    let mut cursor = (300.0, 300.0);
    let matrix_ui = xmath::Matrix::orthographic_off_center(0.0, width, 0.0, height, 0.0, 1.0);


    let mut last = PreciseTime::now();

    // the main loop
    // each cycle will draw once
    'main: loop {
        use glium::Surface;
        use traits::Object;

        //
        // Poll and handle the events received by the window
        //
        for event in display.poll_events() {
            use glium::glutin::{Event, ElementState, MouseButton};
            use glium::glutin::VirtualKeyCode as vkey;

            match event {
                Event::MouseMoved((x, y)) => cursor = (x as f32, height - y as f32),
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    use traits::Move;

                    // 마우스 좌표계 ~ 게임 좌표계 변환
                    let dest = ((cursor.0 - width/2.0)/10.0, (cursor.1 - height/2.0)/10.0);
                    nemo.go(dest)
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(vkey::Q)) => nemo.q(),
                Event::Closed => break 'main,
                _ => ()
            }
        }


        //
        // Update
        //
        // TODO: Limit framerate
        let now = PreciseTime::now();
        let delta = last.to(now).num_nanoseconds().unwrap() as f32 / 1.0E+9;
        print!("FPS: {}\n\x1b[1A", 1.0/delta);
        last = now;

        nemo.update(delta);
        for m in &mut minions {
            m.update(delta);
        }
        controller.update(delta);


        //
        // Render
        //
        let uniforms_ui = uniform! {
            cursor: cursor,
            matrix: matrix_ui.clone()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        nemo.draw(&mut target, &camera).unwrap();
        for minion in &minions {
            minion.draw(&mut target, &camera).unwrap();
        }
        controller.draw(&mut target, &camera).unwrap();

        target.draw(&vb_ui, &ib_ui, &program_ui, &uniforms_ui, &Default::default()).unwrap();
        let _ = target.finish();
    }
}
