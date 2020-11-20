use thiserror::Error;

#[derive(Error, Debug)]
pub enum TeaError {
    #[error("Failed to serialize, details: `{0}`")]
    SerializeError(String),

    #[error("Failed to de-serialize, details: `{0}`")]
    DeserializeError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;
