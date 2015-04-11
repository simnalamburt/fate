#![feature(simd, plugin, slice_patterns)]

extern crate common;
extern crate time;
extern crate glutin;
#[macro_use] extern crate glium;
extern crate obj;

pub mod math;
pub mod resources;
pub mod model;
pub mod shader;
pub mod camera;

use std::f32::consts;
use std::default::Default;
use time::PreciseTime;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
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
    // Parameters for world
    //
    let (vb_world, ib_world) = {
        let (vb, ib) = model::load().unwrap();
        (
            glium::VertexBuffer::new(&display, vb),
            glium::IndexBuffer::new(&display, glium::index::TrianglesList(ib))
        )
    };

    let program_world = {
        let (vs, fs) = shader::load().unwrap();
        glium::Program::from_source(&display, &vs, &fs, None).unwrap()
    };

    let mut camera = camera::Camera::new();

    let uniforms_world = uniform! {
        matrix: {
            use math::Matrix;

            let world = Matrix::rotation_x(consts::FRAC_PI_2);
            let view = camera.matrix();
            let proj = Matrix::perspective_fov(consts::FRAC_PI_4, width/height, 0.1, 100.0);

            world * view * proj
        },
        light: (-1.0, -1.0, -1.0)
    };

    let params_world = glium::DrawParameters {
        depth_write: true,
        depth_test: glium::DepthTest::IfLess,
        .. Default::default()
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
        target.draw(&vb_world, &ib_world, &program_world, &uniforms_world, &params_world).unwrap();
        target.draw(&vb_ui, &ib_ui, &program_ui, &uniforms_ui, &Default::default()).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            use glutin::Event::*;

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
