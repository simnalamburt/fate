use glium::backend::Facade;
use glium::texture::MipmapsOption::NoMipmap;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat::U8U8U8U8;
use xmath::Matrix;
use error::DrawContextCreationError;

pub struct DrawContext {
    pub camera: Matrix,
    pub texture_for_object_picking: Texture2d,
}

impl DrawContext {
    pub fn new<F>(display: &F, width: u32, height: u32) -> Result<DrawContext, DrawContextCreationError> where F: Facade {
        let texture = try!(Texture2d::empty_with_format(display, U8U8U8U8, NoMipmap, width, height));
        Ok(DrawContext {
            camera: Matrix::orthographic(width as f32/10.0, height as f32/10.0, 0.0, 1.0),
            texture_for_object_picking: texture,
        })
    }

    pub fn clear_object_picking_buffer(&self) {
        use glium::Surface;
        let mut object_picking_buffer = self.texture_for_object_picking.as_surface();
        object_picking_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
    }
}
