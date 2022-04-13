use thiserror::Error;

#[derive(Error, Debug)]
pub enum TeaError {
	#[error("Tea common error, details: `{0}`")]
	CommonError(String),

	#[error("Failed to serialize, details: `{0}`")]
	SerializeError(String),

	#[error("Failed to de-serialize, details: `{0}`")]
	DeserializeError(String),

	#[error("Failed to find layer1 block at specified height")]
	FailedToFindBlockError,

	#[error(transparent)]
	Other(#[from] anyhow::Error),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;

pub fn into_err<T>(r: Result<T, Box<dyn std::error::Error>>) -> TeaResult<T> {
	r.map_err(|e| TeaError::CommonError(format!("{}", e)))
}
