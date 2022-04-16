use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type CommonErrorCode = ErrorCodeTranslator<CommonTranslator>;

pub const SERDE_SERIALIZE_ERROR: u16 = 1;
pub const SERDE_DESERIALIZE_ERROR: u16 = 2;
pub const UTF8_ENCODING_ERROR: u16 = 3;
pub const UTF8_STR_ENCODING_ERROR: u16 = 4;
pub const STD_IO_ERROR: u16 = 5;
pub const CHANNEL_RECEIVE_ERROR: u16 = 6;
pub const CHANNEL_SEND_ERROR: u16 = 7;
pub const JSON_MARSHALING_ERROR: u16 = 8;

pub struct CommonTranslator {}
impl Translate for CommonTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			// serde
			SERDE_SERIALIZE_ERROR => "serde serialize failed",
			SERDE_DESERIALIZE_ERROR => "serde deserialize failed",
			JSON_MARSHALING_ERROR => "JSON encoding/decoding failure",
			// utf8
			UTF8_ENCODING_ERROR => "utf8 encoding failed",
			UTF8_STR_ENCODING_ERROR => "utf8 string encoding failed",
			// io
			STD_IO_ERROR => "standard I/O error",
			// channel (crossbeam etc.)
			CHANNEL_RECEIVE_ERROR => "channel receive error",
			CHANNEL_SEND_ERROR => "channel send error",
			_ => "unknown",
		}
	}
}

pub fn new_common_error_code(code: u16) -> CommonErrorCode {
	ErrorCodeTranslator::new(code, CommonTranslator {})
}
