#![feature(core, env, fs, io, std_misc, plugin)]
#![plugin(glium_macros)]

#![feature(old_io, old_path)]

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

    let program = {
        use std::fs::File;
        use std::io::prelude::*;

        let mut vs = String::new();
        let mut fs = String::new();
        let res = std::env::current_exe().unwrap().dir_path().join("..").join("res");
        File::open(&res.join("vertex.glsl")).unwrap().read_to_string(&mut vs).unwrap();
        File::open(&res.join("fragment.glsl")).unwrap().read_to_string(&mut fs).unwrap();

        glium::Program::from_source(&display, &vs, &fs, None).unwrap()
    };

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
