use log::{warn, ParseLevelError};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::array::TryFromSliceError;
use std::fmt::{Debug, Formatter};
use std::num::{ParseIntError, TryFromIntError};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
use thiserror::Error;

pub mod code;
pub mod translator;

pub use code::{
	client::{new_client_error_code, ClientCode},
	common::{new_common_error_code, CommonCode},
	layer1::{new_layer1_error_code, Layer1Code},
	service::{new_service_error_code, ServiceCode},
	vmh::{new_vmh_error_code, VmhCode},
	wascc::{new_wascc_error_code, WasccCode},
	ErrorCode,
};

#[derive(Error, Clone, Deserialize, Serialize)]
pub enum TeaError {
	#[error("Tea common error, details: `{0}`")]
	CommonError(String),

	#[error(transparent)]
	EncodedError(#[from] ErrorCode),

	#[error(transparent)]
	NewError(crate::errorx::Error<()>),
}

pub type TeaResult<T> = std::result::Result<T, TeaError>;
pub type Result<T, E = TeaError> = std::result::Result<T, E>;

pub fn option_none_error<S: AsRef<str>>(msg: S) -> TeaError {
	code::common::new_common_error_code(CommonCode::OptionIsNone)
		.to_error_code(Some(msg.as_ref().to_string()), None)
}

pub fn discard_message_error<S: AsRef<str>>(msg: S) -> TeaError {
	code::wascc::new_wascc_error_code(WasccCode::DiscardMessageError)
		.to_error_code(Some(msg.as_ref().to_string()), None)
}

pub fn tea_err<T, E: Into<TeaError>>(e: Result<T, E>) -> TeaResult<T> {
	e.map_err::<TeaError, _>(|e| e.into())
}

impl TeaError {
	pub fn parse_error_code(&self) -> Option<ErrorCode> {
		match self {
			TeaError::EncodedError(e) => Some(e.clone()),
			TeaError::CommonError(s) => {
				warn!("common error emit: {}", s);
				Some(new_common_error_code(CommonCode::CommonError).into())
			}
			TeaError::NewError(x) => Some(ErrorCode::new_nested(
				CommonCode::NewError as _,
				x.summary().map(Into::into).unwrap_or_default(),
				x.detail().map(Into::into),
				x.inner()
					.iter()
					.filter_map(|x| {
						let error: TeaError = (*x).clone().into();
						error.parse_error_code()
					})
					.collect::<Vec<_>>()
					.pop(),
			)),
		}
	}
}

impl From<prost::EncodeError> for TeaError {
	fn from(e: prost::EncodeError) -> Self {
		new_common_error_code(CommonCode::ProstEncodeError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<prost::DecodeError> for TeaError {
	fn from(e: prost::DecodeError) -> Self {
		new_common_error_code(CommonCode::ProstDecodeError)
			.to_error_code(Some(format!("{:?}", e)), None)
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
		new_common_error_code(CommonCode::TryFromConvertError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<ParseIntError> for TeaError {
	fn from(e: ParseIntError) -> Self {
		new_common_error_code(CommonCode::ParseStringError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<SystemTimeError> for TeaError {
	fn from(e: SystemTimeError) -> Self {
		new_common_error_code(CommonCode::SystemTimeError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<FromUtf8Error> for TeaError {
	fn from(e: FromUtf8Error) -> Self {
		new_common_error_code(CommonCode::UTF8EncodingError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<Utf8Error> for TeaError {
	fn from(e: Utf8Error) -> Self {
		new_common_error_code(CommonCode::UTF8EncodingError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<serde_json::Error> for TeaError {
	fn from(e: Error) -> Self {
		new_common_error_code(CommonCode::SerdeGeneralError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<std::io::Error> for TeaError {
	fn from(e: std::io::Error) -> Self {
		new_common_error_code(CommonCode::StdIoError).to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<log::SetLoggerError> for TeaError {
	fn from(e: log::SetLoggerError) -> Self {
		new_common_error_code(CommonCode::LogGeneralError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<ParseLevelError> for TeaError {
	fn from(e: ParseLevelError) -> Self {
		new_common_error_code(CommonCode::LogGeneralError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<bincode::Error> for TeaError {
	fn from(e: bincode::Error) -> Self {
		new_common_error_code(CommonCode::BincodeSerializeError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<TryFromSliceError> for TeaError {
	fn from(e: TryFromSliceError) -> Self {
		new_common_error_code(CommonCode::TryFromConvertError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<base64::DecodeError> for TeaError {
	fn from(e: base64::DecodeError) -> Self {
		new_common_error_code(CommonCode::Base64DecodeError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl From<hex::FromHexError> for TeaError {
	fn from(e: hex::FromHexError) -> Self {
		new_common_error_code(CommonCode::HexDecodeError)
			.to_error_code(Some(format!("{:?}", e)), None)
	}
}

impl<S> From<crate::errorx::Error<S>> for TeaError {
	fn from(x: crate::errorx::Error<S>) -> Self {
		match x.back_cast() {
			Ok(e) => e,
			Err(x) => TeaError::NewError(x.into_scope()),
		}
	}
}

impl Debug for TeaError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TeaError::EncodedError(code) => write!(f, "{:?}", code),
			TeaError::CommonError(msg) => write!(f, "Unknown error: {}", msg),
			TeaError::NewError(err) => Debug::fmt(err, f),
		}
	}
}
