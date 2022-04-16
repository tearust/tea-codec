use crate::error::code::common::{PROST_DECODE_ERROR, PROST_ENCODE_ERROR};
use crate::error::code::ErrorCode;
use crate::new_common_error_code;
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

impl TeaError {
	pub fn parse_error_code(&self) -> Option<ErrorCode> {
		match self {
			TeaError::EncodedError(e) => Some(e.clone()),
			_ => None,
		}
	}
}

impl From<prost::EncodeError> for TeaError {
	fn from(e: prost::EncodeError) -> Self {
		new_common_error_code(PROST_ENCODE_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<prost::DecodeError> for TeaError {
	fn from(e: prost::DecodeError) -> Self {
		new_common_error_code(PROST_DECODE_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}
