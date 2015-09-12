use glium::Program;
use glium::backend::Facade;
use glium::texture::MipmapsOption::NoMipmap;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat::U8U8U8U8;
use xmath::{Vector3, Matrix};
use error::DrawContextCreationError;

pub struct DrawContext {
    pub camera: Matrix,
    pub texture_for_object_picking: Texture2d,
    pub fill_id_program: Program,
}

impl DrawContext {
    pub fn new<F>(display: &F, width: u32, height: u32) -> Result<DrawContext, DrawContextCreationError> where F: Facade {
        use std::f32::consts;

        let texture = try!(Texture2d::empty_with_format(display, U8U8U8U8, NoMipmap, width, height));
        let fill_id_program = try!(Program::from_source(display, r#"
            #version 410
            uniform mat4 matrix;
            in vec3 position;
            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#, r#"
            #version 410
            uniform vec4 id;
            out vec4 color;
            void main() {
                color = id;
            }
        "#, None));

        let focus = Vector3::new(0.0, 0.0, 4.0);
        let dir = Vector3::new(-0.5773502691896258, 0.5773502691896258, -0.5773502691896258);
        let dist = 34.64101615137754;
        let camera = Matrix::look_at(focus - dir * dist, focus, Vector3::new(0.0, 0.0, 1.0));
        let proj = Matrix::perspective(consts::FRAC_PI_4, width as f32/height as f32, 0.001, 1000.0);

        Ok(DrawContext {
            camera: camera * proj,
            texture_for_object_picking: texture,
            fill_id_program: fill_id_program,
        })
    }

    pub fn clear_object_picking_buffer(&self) {
        use glium::Surface;
        let mut object_picking_buffer = self.texture_for_object_picking.as_surface();
        object_picking_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
    }
}
