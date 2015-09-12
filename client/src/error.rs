use std::io;
use glium::{vertex, index, texture};
use glium::program::ProgramCreationError;
use bincode::rustc_serialize::DecodingError;

#[derive(Debug)]
pub enum CreationError {
    IoError(io::Error),
    VertexBufferCreationError(vertex::BufferCreationError),
    IndexBufferCreationError(index::BufferCreationError),
    ProgramCreationError(ProgramCreationError),
    DecodingError(DecodingError),
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
implmnt!(DecodingError);

#[derive(Debug)]
pub enum DrawContextCreationError {
    ProgramCreationError(ProgramCreationError),
    TextureCreationError(texture::TextureCreationError),
}

impl From<ProgramCreationError> for DrawContextCreationError {
    fn from(err: ProgramCreationError) -> Self {
        DrawContextCreationError::ProgramCreationError(err)
    }
}
impl From<texture::TextureCreationError> for DrawContextCreationError {
    fn from(err: texture::TextureCreationError) -> Self {
        DrawContextCreationError::TextureCreationError(err)
    }
}
