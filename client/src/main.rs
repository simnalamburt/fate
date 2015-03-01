#![feature(core, env, fs, path, io, simd, std_misc, plugin)]
#![plugin(glium_macros)]

#![feature(old_io)]

extern crate common;
extern crate glutin;
#[macro_use] extern crate glium;
extern crate obj;

pub mod math;
pub mod resources;
pub mod model;
pub mod shader;

use std::f32::consts;
use std::default::Default;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_depth_buffer(32)
        .with_title(common::PROJECT_NAME.to_string())
        .build_glium().unwrap();

    let (vb, ib) = model::load().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, vb);
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(ib));

    let (vs, fs) = shader::load().unwrap();
    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();

    // drawing a frame
    let uniforms = uniform! {
        matrix: {
            use math::{vec, Matrix};

            let view = Matrix::look_at(
                vec(2.0, 0.0, 9.0),
                vec(0.0, 1.0, 0.0),
                vec(0.0, 1.0, 0.0));

            let (width, height) = display.get_framebuffer_dimensions();
            let proj = Matrix::perspective_fov(consts::FRAC_PI_4, height as f32/width as f32, 0.001, 100.0);

            proj * view
        },
        light: (-1.0, -1.0, -1.0)
    };

    let params = glium::DrawParameters {
        depth_test: glium::DepthTest::IfMore,
        .. Default::default()
    };

    // the main loop
    // each cycle will draw once
    'main: loop {
        use std::old_io::timer;
        use std::time::Duration;
        use glium::Surface;

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
