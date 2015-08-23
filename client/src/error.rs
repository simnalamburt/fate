use std::io;
use glium::{vertex, index, texture};
use glium::program::ProgramCreationError;
use obj::ObjError;

#[derive(Debug)]
pub enum CreationError {
    IoError(io::Error),
    VertexBufferCreationError(vertex::BufferCreationError),
    IndexBufferCreationError(index::BufferCreationError),
    ProgramCreationError(ProgramCreationError),
    ObjError(ObjError),
}

macro_rules! implmnt {
    ($name:ident, $error:ty) => {
        impl From<$error> for CreationError {
            fn from(err: $error) -> Self {
                CreationError::$name(err)
            }
        }
    };

    ($name:ident) => ( implmnt!($name, $name); )
}

implmnt!(IoError, io::Error);
implmnt!(VertexBufferCreationError, vertex::BufferCreationError);
implmnt!(IndexBufferCreationError, index::BufferCreationError);
implmnt!(ProgramCreationError);
implmnt!(ObjError);

#[derive(Debug)]
pub enum DrawContextCreationError {
    TextureCreationError(texture::TextureCreationError),
}

impl From<texture::TextureCreationError> for DrawContextCreationError {
    fn from(err: texture::TextureCreationError) -> Self {
        DrawContextCreationError::TextureCreationError(err)
    }
}
