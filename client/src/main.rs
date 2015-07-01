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

        let uniforms_ui = uniform! {
            cursor: cursor,
            matrix: matrix_ui.clone()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vb_ui, &ib_ui, &program_ui, &uniforms_ui, &Default::default()).unwrap();
        let _ = target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            use glium::glutin::Event::*;

            match event {
                MouseMoved((x, y)) => cursor = (x as f32, height - y as f32),
                Closed => break 'main,
                _ => ()
            }
        }

        let now = PreciseTime::now();
        print!("FPS: {}\n\x1b[1A", 1.0E+9 / last.to(now).num_nanoseconds().unwrap() as f64);
        last = now;
    }
}
