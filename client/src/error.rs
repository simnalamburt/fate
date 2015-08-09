use std::io;
use glium::vertex::BufferCreationError;
use glium::program::ProgramCreationError;

#[derive(Debug)]
pub enum CreationError {
    IoError(io::Error),
    BufferCreationError(BufferCreationError),
    ProgramCreationError(ProgramCreationError),
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
implmnt!(BufferCreationError);
implmnt!(ProgramCreationError);
