use crate::error::code::common::{
	BASE64_DECODE_ERROR, BINCODE_SERIALIZE_ERROR, COMMON_ERROR, HEX_DECODE_ERROR,
	LOG_GENERAL_ERROR, PARSE_STRING_ERROR, PROST_DECODE_ERROR, PROST_ENCODE_ERROR,
	SERDE_GENERAL_ERROR, STD_IO_ERROR, SYSTEM_TIME_ERROR, TRY_FROM_CONVERT_ERROR,
	UTF8_ENCODING_ERROR,
};
use crate::error::code::ErrorCode;
use crate::new_common_error_code;
use base64::DecodeError;
use hex::FromHexError;
use log::{warn, ParseLevelError, SetLoggerError};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::array::TryFromSliceError;
use std::fmt::Debug;
use std::num::{ParseIntError, TryFromIntError};
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
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

pub fn option_none_error<S: AsRef<str>>(msg: S) -> TeaError {
	code::common::new_common_error_code(code::common::OPTION_IS_NONE)
		.to_error_code(Some(format!("{}", msg.as_ref())), None)
}

pub fn discard_message_error<S: AsRef<str>>(msg: S) -> TeaError {
	code::wascc::new_wascc_error_code(code::wascc::DISCARD_MESSAGE_ERROR)
		.to_error_code(Some(format!("{}", msg.as_ref())), None)
}

impl TeaError {
	pub fn parse_error_code(&self) -> Option<ErrorCode> {
		match self {
			TeaError::EncodedError(e) => Some(e.clone()),
			TeaError::CommonError(s) => {
				warn!("common error emit: {}", s);
				Some(new_common_error_code(COMMON_ERROR).into())
			}
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

impl From<String> for TeaError {
	fn from(s: String) -> Self {
		TeaError::CommonError(s)
	}
}

impl From<&str> for TeaError {
	fn from(s: &str) -> Self {
		TeaError::CommonError(s.to_string())
	}
}

impl From<TryFromIntError> for TeaError {
	fn from(e: TryFromIntError) -> Self {
		new_common_error_code(TRY_FROM_CONVERT_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<ParseIntError> for TeaError {
	fn from(e: ParseIntError) -> Self {
		new_common_error_code(PARSE_STRING_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<SystemTimeError> for TeaError {
	fn from(e: SystemTimeError) -> Self {
		new_common_error_code(SYSTEM_TIME_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<FromUtf8Error> for TeaError {
	fn from(e: FromUtf8Error) -> Self {
		new_common_error_code(UTF8_ENCODING_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<serde_json::Error> for TeaError {
	fn from(e: Error) -> Self {
		new_common_error_code(SERDE_GENERAL_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<std::io::Error> for TeaError {
	fn from(e: std::io::Error) -> Self {
		new_common_error_code(STD_IO_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<log::SetLoggerError> for TeaError {
	fn from(e: SetLoggerError) -> Self {
		new_common_error_code(LOG_GENERAL_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<ParseLevelError> for TeaError {
	fn from(e: ParseLevelError) -> Self {
		new_common_error_code(LOG_GENERAL_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<bincode::Error> for TeaError {
	fn from(e: bincode::Error) -> Self {
		new_common_error_code(BINCODE_SERIALIZE_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<TryFromSliceError> for TeaError {
	fn from(e: TryFromSliceError) -> Self {
		new_common_error_code(TRY_FROM_CONVERT_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<base64::DecodeError> for TeaError {
	fn from(e: DecodeError) -> Self {
		new_common_error_code(BASE64_DECODE_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<hex::FromHexError> for TeaError {
	fn from(e: FromHexError) -> Self {
		new_common_error_code(HEX_DECODE_ERROR).to_error_code(Some(format!("{:?}", e)), None)
	}
}
