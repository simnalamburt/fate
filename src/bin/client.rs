#![feature(io, std_misc, plugin)]
#![plugin(glium_macros)]

extern crate glutin;
#[macro_use] extern crate glium;
extern crate glium_macros;

use math::Matrix;

fn main() {
    use glium::{Surface, DisplayBuild};

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
            Vertex { position: [ -0.9,  0.8, 0.0 ], color: [ 1.00, 0.71, 0.00 ] },
            Vertex { position: [ -0.8, -0.8, 1.0 ], color: [ 1.00, 0.85, 0.78 ] },
            Vertex { position: [  0.5,  0.0, 0.5 ], color: [ 1.00, 1.00, 1.00 ] },
            Vertex { position: [  0.9, -0.8, 0.5 ], color: [ 0.00, 0.51, 1.00 ] },
            Vertex { position: [  0.8,  0.8, 0.5 ], color: [ 0.47, 0.74, 1.00 ] },
            Vertex { position: [ -0.5,  0.0, 0.5 ], color: [ 1.00, 1.00, 1.00 ] },
        ])
    };

    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(vec![
        0, 1, 2,
        3, 4, 5 as u16
    ]));

    let program = glium::Program::from_source(&display,
        // vertex shader
        "   #version 110

            uniform mat4 matrix;

            attribute vec3 position;
            attribute vec3 color;

            varying vec3 v_color;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
                v_color = color;
            }
        ",

        // fragment shader
        "   #version 110
            varying vec3 v_color;

            void main() {
                gl_FragColor = vec4(v_color, 1.0);
            }
        ",

        // optional geometry shader
        None
    ).unwrap();

    let uniforms = uniform! {
        matrix: Matrix::one()
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

        // drawing a frame
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

mod math {
    use glium::uniforms::*;

    type Pos = (f32, f32, f32);
    type Dir = (f32, f32, f32);

    pub struct Matrix {
        m: [f32; 16]
    }

    impl Matrix {
        pub fn one() -> Self {
            Matrix {
                m: [
                    1.0, 0.0, 0.0, 1.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]
            }
        }
    }

    impl IntoUniformValue<'static> for Matrix {
        fn into_uniform_value(self) -> UniformValue<'static> {
            UniformValue::Mat4([
                [self.m[00], self.m[04], self.m[08], self.m[12]],
                [self.m[01], self.m[05], self.m[09], self.m[13]],
                [self.m[02], self.m[06], self.m[10], self.m[14]],
                [self.m[03], self.m[07], self.m[11], self.m[15]],
            ])
        }
    }
}
