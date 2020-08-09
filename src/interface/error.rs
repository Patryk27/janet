use thiserror::Error;

pub type InterfaceResult<T> = Result<T, InterfaceError>;

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("Unknown command")]
    UnknownCommand,
}
