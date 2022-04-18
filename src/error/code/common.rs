use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type CommonErrorCode = ErrorCodeTranslator<CommonTranslator, CommonCode>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum CommonCode {
	CommonError = 0,
	SerdeSerializeError,
	SerdeDeserializeError,
	UTF8EncodingError,
	Utf8StrEncodingError,
	StdIoError,
	ChannelReceiveError,
	ChannelSendError,
	JsonMarshalingError,
	ProstEncodeError,
	ProstDecodeError,
	TryIntoConvertError,
	TryFromConvertError,
	ParseStringError,
	SystemTimeError,
	MutexLockFailed,
	JsonRPCParseError,
	OptionIsNone,
	SerdeGeneralError,
	HexDecodeError,
	Base64DecodeError,
	ReqwestGeneralError,
	LogGeneralError,
	BincodeSerializeError,
}

pub struct CommonTranslator {}
impl Translate<CommonCode> for CommonTranslator {
	fn translate(&self, code: CommonCode) -> &'static str {
		match code {
			CommonCode::CommonError => "common error",
			// basic
			CommonCode::OptionIsNone => "option is none",
			CommonCode::SystemTimeError => "system time error",
			CommonCode::TryIntoConvertError => "try_into conversion error",
			CommonCode::TryFromConvertError => "try_from conversion error",
			CommonCode::ParseStringError => "parse string error",
			CommonCode::MutexLockFailed => "mutex lock failed",
			CommonCode::LogGeneralError => "log error",
			// serde
			CommonCode::SerdeGeneralError => "serde general error",
			CommonCode::SerdeSerializeError => "serde serialize failed",
			CommonCode::SerdeDeserializeError => "serde deserialize failed",
			CommonCode::JsonMarshalingError => "JSON encoding/decoding failure",
			CommonCode::BincodeSerializeError => "bincode serialization failure",
			// prost encoding/decoding
			CommonCode::ProstEncodeError => "prost encoding error",
			CommonCode::ProstDecodeError => "prost decoding error",
			CommonCode::UTF8EncodingError => "utf8 encoding failed",
			CommonCode::Utf8StrEncodingError => "utf8 string encoding failed",
			CommonCode::HexDecodeError => "hex decoding error",
			CommonCode::Base64DecodeError => "base64 decoding error",
			// io
			CommonCode::StdIoError => "standard I/O error",
			// channel (crossbeam etc.)
			CommonCode::ChannelReceiveError => "channel receive error",
			CommonCode::ChannelSendError => "channel send error",
			// web
			CommonCode::JsonRPCParseError => "json rpc parse error",
			CommonCode::ReqwestGeneralError => "reqwest error",
		}
	}
}

impl From<u16> for CommonCode {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(CommonCode::CommonError)
	}
}

pub fn new_common_error_code(code: CommonCode) -> CommonErrorCode {
	ErrorCodeTranslator::new(code as u16, CommonTranslator {})
}
