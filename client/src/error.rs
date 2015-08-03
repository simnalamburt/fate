use glium::vertex::BufferCreationError;
use glium::program::ProgramCreationError;

#[derive(Debug)]
pub enum CreationError {
    BufferCreationError(BufferCreationError),
    ProgramCreationError(ProgramCreationError),
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
