use std::io;
use glium::vertex::BufferCreationError;
use glium::program::ProgramCreationError;

#[derive(Debug)]
pub enum CreationError {
    IoError(io::Error),
    BufferCreationError(BufferCreationError),
    ProgramCreationError(ProgramCreationError),
}

impl From<io::Error> for CreationError {
    fn from(err: io::Error) -> CreationError {
        CreationError::IoError(err)
    }
}

impl From<BufferCreationError> for CreationError {
    fn from(err: BufferCreationError) -> CreationError {
        CreationError::BufferCreationError(err)
    }
}

impl From<ProgramCreationError> for CreationError {
    fn from(err: ProgramCreationError) -> CreationError {
        CreationError::ProgramCreationError(err)
    }
}
