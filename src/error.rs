use thiserror::Error;

#[derive(Error, Debug)]
pub enum TeaError {

    #[error(transparent)]
    Other(#[from] std::io::Error),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;