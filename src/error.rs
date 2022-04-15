use crate::error::code::ErrorCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

pub mod code;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum TeaError {
	#[error("Tea common error, details: `{0}`")]
	CommonError(String),

	#[error("Failed to serialize, details: `{0}`")]
	SerializeError(String),

	#[error("Failed to de-serialize, details: `{0}`")]
	DeserializeError(String),

	#[error(transparent)]
	EncodedError(#[from] ErrorCode),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;

pub fn into_err<T>(r: Result<T, Box<dyn std::error::Error>>) -> TeaResult<T> {
	r.map_err(|e| TeaError::CommonError(format!("{}", e)))
}
