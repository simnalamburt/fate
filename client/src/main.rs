extern crate common;
extern crate time;
#[macro_use] extern crate glium;

pub mod math;
mod nemo;

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
        (dim.0 as f64, dim.1 as f64)
    };


    //
    // Game
    //
    let mut nemo = nemo::Nemo::new(&display);
    let world = math::Matrix::orthographic(width as f32/10.0, height as f32/10.0, 0.0, 1.0);


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
    let matrix_ui = math::Matrix::orthographic_off_center(0.0, width as f32, 0.0, height as f32, 0.0, 1.0);


    let mut last = PreciseTime::now();

    // the main loop
    // each cycle will draw once
    'main: loop {
        use glium::Surface;

        //
        // Poll and handle the events received by the window
        //
        for event in display.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::ElementState::*;
            use glium::glutin::MouseButton::*;

            match event {
                MouseMoved((x, y)) => cursor = (x as f64, height - y as f64),
                MouseInput(Pressed, Left) => nemo.go({
                    // 마우스 좌표계 ~ 게임 좌표계 변환
                    ((cursor.0 - width/2.0)/10.0, (cursor.1 - height/2.0)/10.0)
                }),
                Closed => break 'main,
                _ => ()
            }
        }


        //
        // Update
        //
        let now = PreciseTime::now();
        let delta = last.to(now).num_nanoseconds().unwrap() as f64 / 1.0E+9;
        print!("FPS: {}\n\x1b[1A", 1.0/delta);
        last = now;

        nemo.update(delta);


        //
        // Render
        //
        let uniforms_ui = uniform! {
            cursor: (cursor.0 as f32, cursor.1 as f32), // <- (f64, f64) doesn't implement AsUniformValue
            matrix: matrix_ui.clone()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        let mut target = nemo.draw(target, world.clone()); // ㅇㅅㅇ?
        target.draw(&vb_ui, &ib_ui, &program_ui, &uniforms_ui, &Default::default()).unwrap();
        let _ = target.finish();
    }
}
