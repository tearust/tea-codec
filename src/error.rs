use crate::error::code::ErrorCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

pub mod code;
pub mod translator;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum TeaError {
	#[error("Tea common error, details: `{0}`")]
	CommonError(String),

	#[error(transparent)]
	EncodedError(#[from] ErrorCode),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;
