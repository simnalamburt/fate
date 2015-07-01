extern crate common;
extern crate time;
#[macro_use] extern crate glium;

pub mod math;
pub mod resources;
pub mod shader;

use std::default::Default;
use time::PreciseTime;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_depth_buffer(32)
        .with_title(common::PROJECT_NAME.to_string())
        .build_glium().unwrap();


    //
    // Basics
    //
    let (width, height) = {
        let dim = display.get_framebuffer_dimensions();
        (dim.0 as f32, dim.1 as f32)
    };


    //
    // Nemo
    //
    let vb_nemo = glium::VertexBuffer::new(&display, {
        #[derive(Clone, Copy)]
        struct Vertex { position: [f32; 2] }

        implement_vertex!(Vertex, position);

        vec![
            Vertex { position: [ -10.0, -10.0 ] },
            Vertex { position: [ -10.0,  10.0 ] },
            Vertex { position: [  10.0, -10.0 ] },
            Vertex { position: [  10.0,  10.0 ] },
        ]
    });
    let ib_nemo = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program_nemo = glium::Program::from_source(&display,
        r#"
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
        "#, None).unwrap();
    let mut pos_nemo = (0.0, 0.0);

    let matrix_game = math::Matrix::orthographic(width/10.0, height/10.0, 0.0, 1.0);


    //
    // Parameters for UI
    //
    let vb_ui = glium::VertexBuffer::new(&display, {
        #[derive(Clone, Copy)]
        struct Vertex { position: [f32; 2] }

        implement_vertex!(Vertex, position);

        vec![
            Vertex { position: [ -2.0, -2.0 ] },
            Vertex { position: [ -2.0,  3.0 ] },
            Vertex { position: [  3.0, -2.0 ] },
            Vertex { position: [  3.0,  3.0 ] },
        ]
    });
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
    let matrix_ui = math::Matrix::orthographic_off_center(0.0, width, 0.0, height, 0.0, 1.0);


    let mut last = PreciseTime::now();

    // the main loop
    // each cycle will draw once
    'main: loop {
        use glium::Surface;

        let uniforms_nemo = uniform! {
            pos: pos_nemo,
            matrix: matrix_game.clone()
        };

        let uniforms_ui = uniform! {
            cursor: cursor,
            matrix: matrix_ui.clone()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vb_nemo, &ib_nemo, &program_nemo, &uniforms_nemo, &Default::default()).unwrap();
        target.draw(&vb_ui, &ib_ui, &program_ui, &uniforms_ui, &Default::default()).unwrap();
        let _ = target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::ElementState::*;
            use glium::glutin::MouseButton::*;

            match event {
                MouseMoved((x, y)) => cursor = (x as f32, height - y as f32),
                MouseInput(Pressed, Left) => pos_nemo = {
                    // 마우스 좌표계 ~ 게임 좌표계 변환
                    ((cursor.0 - width/2.0)/10.0, (cursor.1 - height/2.0)/10.0)
                },
                Closed => break 'main,
                _ => ()
            }
        }

        let now = PreciseTime::now();
        print!("FPS: {}\n\x1b[1A", 1.0E+9 / last.to(now).num_nanoseconds().unwrap() as f64);
        last = now;
    }
}
