use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type CommonErrorCode = ErrorCodeTranslator<CommonTranslator>;

pub const SERDE_SERIALIZE_ERROR: u16 = 1;
pub const SERDE_DESERIALIZE_ERROR: u16 = 2;
pub const UTF8_ENCODING_ERROR: u16 = 3;
pub const UTF8_STR_ENCODING_ERROR: u16 = 4;

pub struct CommonTranslator {}
impl Translate for CommonTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			SERDE_SERIALIZE_ERROR => "serde serialize failed",
			SERDE_DESERIALIZE_ERROR => "serde deserialize failed",
			UTF8_ENCODING_ERROR => "utf8 encoding failed",
			UTF8_STR_ENCODING_ERROR => "utf8 string encoding failed",
			_ => "unknown",
		}
	}
}

pub fn new_common_error_code(code: u16) -> CommonErrorCode {
	ErrorCodeTranslator::new(code, CommonTranslator {})
}
