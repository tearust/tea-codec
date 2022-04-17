use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type CommonErrorCode = ErrorCodeTranslator<CommonTranslator>;

pub const COMMON_ERROR: u16 = 0;
pub const SERDE_SERIALIZE_ERROR: u16 = 1;
pub const SERDE_DESERIALIZE_ERROR: u16 = 2;
pub const UTF8_ENCODING_ERROR: u16 = 3;
pub const UTF8_STR_ENCODING_ERROR: u16 = 4;
pub const STD_IO_ERROR: u16 = 5;
pub const CHANNEL_RECEIVE_ERROR: u16 = 6;
pub const CHANNEL_SEND_ERROR: u16 = 7;
pub const JSON_MARSHALING_ERROR: u16 = 8;
pub const PROST_ENCODE_ERROR: u16 = 9;
pub const PROST_DECODE_ERROR: u16 = 10;
pub const TRY_INTO_CONVERT_ERROR: u16 = 11;
pub const TRY_FROM_CONVERT_ERROR: u16 = 12;
pub const PARSE_STRING_ERROR: u16 = 13;
pub const SYSTEM_TIME_ERROR: u16 = 14;
pub const MUTEX_LOCK_FAILED: u16 = 15;
pub const JSON_RPC_PARSE_ERROR: u16 = 16;
pub const OPTION_IS_NONE: u16 = 17;
pub const SERDE_GENERAL_ERROR: u16 = 18;
pub const HEX_DECODE_ERROR: u16 = 19;
pub const BASE64_DECODE_ERROR: u16 = 20;
pub const REQWEST_GENERAL_ERROR: u16 = 21;
pub const LOG_GENERAL_ERROR: u16 = 22;
pub const BINCODE_SERIALIZE_ERROR: u16 = 23;

pub struct CommonTranslator {}
impl Translate for CommonTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			COMMON_ERROR => "common error",
			// basic
			OPTION_IS_NONE => "option is none",
			SYSTEM_TIME_ERROR => "system time error",
			TRY_INTO_CONVERT_ERROR => "try_into conversion error",
			TRY_FROM_CONVERT_ERROR => "try_from conversion error",
			PARSE_STRING_ERROR => "parse string error",
			MUTEX_LOCK_FAILED => "mutex lock failed",
			LOG_GENERAL_ERROR => "log error",
			// serde
			SERDE_GENERAL_ERROR => "serde general error",
			SERDE_SERIALIZE_ERROR => "serde serialize failed",
			SERDE_DESERIALIZE_ERROR => "serde deserialize failed",
			JSON_MARSHALING_ERROR => "JSON encoding/decoding failure",
			BINCODE_SERIALIZE_ERROR => "bincode serialization failure",
			// prost encoding/decoding
			PROST_ENCODE_ERROR => "prost encoding error",
			PROST_DECODE_ERROR => "prost decoding error",
			UTF8_ENCODING_ERROR => "utf8 encoding failed",
			UTF8_STR_ENCODING_ERROR => "utf8 string encoding failed",
			HEX_DECODE_ERROR => "hex decoding error",
			BASE64_DECODE_ERROR => "base64 decoding error",
			// io
			STD_IO_ERROR => "standard I/O error",
			// channel (crossbeam etc.)
			CHANNEL_RECEIVE_ERROR => "channel receive error",
			CHANNEL_SEND_ERROR => "channel send error",
			// web
			JSON_RPC_PARSE_ERROR => "json rpc parse error",
			REQWEST_GENERAL_ERROR => "reqwest error",
			_ => "unknown",
		}
	}
}

pub fn new_common_error_code(code: u16) -> CommonErrorCode {
	ErrorCodeTranslator::new(code, CommonTranslator {})
}
