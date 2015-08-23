use glium::backend::Facade;
use glium::texture::MipmapsOption::NoMipmap;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat::U8U8U8U8;
use xmath::Matrix;

pub struct DrawContext {
    pub camera: Matrix,
    pub texture_for_object_picking: Texture2d,
}

impl DrawContext {
    pub fn new<F>(display: &F, width: u32, height: u32) -> DrawContext where F: Facade {
        DrawContext {
            camera: Matrix::orthographic(width as f32/10.0, height as f32/10.0, 0.0, 1.0),
            texture_for_object_picking: Texture2d::empty_with_format(display, U8U8U8U8, NoMipmap, width, height).unwrap(),
        }
    }
}
