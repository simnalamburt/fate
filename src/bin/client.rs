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

    let (vb, ib) = model::load().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, vb);
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(ib));

    let (vs, fs) = shader::load().unwrap();
    let program = glium::Program::from_source(&display, &vs, &fs, None).unwrap();

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

pub mod resources {
    use std::old_path::BytesContainer;
    use std::fs::File;
    use std::io::Result;
    use std::env::current_exe;

    pub fn load<T: BytesContainer>(name: T) -> Result<File> {
        let res = current_exe().unwrap().dir_path().join("..").join("res");

        File::open(&res.join(name))
    }
}

pub mod model {
    use std::io::Result;

    pub fn load() -> Result<(Vec<Vertex>, Vec<u16>)> {
        let vb = vec![
            Vertex::new(-0.5, 0.0, -10.0),
            Vertex::new(-0.5, 0.5, -10.0),
            Vertex::new( 0.5, 0.5, -10.0),
            Vertex::new( 0.9, 0.0, -10.0),
            Vertex::new( 0.8, 1.0, -10.0),
            Vertex::new(-0.5, 0.5, -10.0),
        ];

        let ib = vec![
            0, 1, 2,
            3, 4, 5 as u16
        ];

        Ok((vb, ib))
    }

    #[vertex_format]
    #[derive(Copy)]
    pub struct Vertex {
        position: [f32; 3],
    }

    impl Vertex {
        fn new(x: f32, y: f32, z: f32) -> Self {
            Vertex { position: [x, y, z] }
        }
    }
}

pub mod shader {
    use std::io::Result;
    use std::io::prelude::*;
    use resources;

    pub fn load() -> Result<(String, String)> {
        let mut vs = String::new();
        let mut fs = String::new();

        try!(resources::load("vertex.glsl")).read_to_string(&mut vs).unwrap();
        try!(resources::load("fragment.glsl")).read_to_string(&mut fs).unwrap();

        Ok((vs, fs))
    }
}
