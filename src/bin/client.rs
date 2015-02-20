#![feature(core, std_misc, plugin)]
#![plugin(glium_macros)]

#![feature(old_io)]

extern crate glutin;
#[macro_use] extern crate glium;
extern crate glium_practice;

use glium::{Surface, DisplayBuild};
use glium_practice::math::Matrix;

fn main() {

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium().unwrap();

    let vertex_buffer = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        glium::VertexBuffer::new(&display, vec![
            Vertex { position: [ -0.5,  0.0, -10.0 ], color: [ 1.00, 0.71, 0.00 ] },
            Vertex { position: [ -0.5,  0.5, -10.0 ], color: [ 1.00, 0.85, 0.78 ] },
            Vertex { position: [  0.5,  0.5, -10.0 ], color: [ 1.00, 1.00, 1.00 ] },
            Vertex { position: [  0.9,  0.0, -10.0 ], color: [ 0.00, 0.51, 1.00 ] },
            Vertex { position: [  0.8,  1.0, -10.0 ], color: [ 0.47, 0.74, 1.00 ] },
            Vertex { position: [ -0.5,  0.5, -10.0 ], color: [ 1.00, 1.00, 1.00 ] },
        ])
    };

    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(vec![
        0, 1, 2,
        3, 4, 5 as u16
    ]));

    let program = glium::Program::from_source(&display,
        // vertex shader
        "   #version 410

            uniform mat4 matrix;

            in vec3 position;
            in vec3 color;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }
        ",

        // fragment shader
        "   #version 410

            out vec4 color;

            void main() {
                color = vec4(1, 0.93, 0.56, 1);
            }
        ",

        // optional geometry shader
        None
    ).unwrap();

    // drawing a frame
    let uniforms = uniform! {
        matrix: {
            let (width, height) = display.get_framebuffer_dimensions();
            Matrix::perspective_fov(std::f32::consts::FRAC_PI_4, height as f32/width as f32, 0.001, 1000.0)
        }
    };

    let params = glium::DrawParameters {
        depth_function: glium::DepthFunction::IfMore,
        .. std::default::Default::default()
    };

    // the main loop
    // each cycle will draw once
    'main: loop {
        use std::old_io::timer;
        use std::time::Duration;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
