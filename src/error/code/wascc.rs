use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type WasccErrorCode = ErrorCodeTranslator<WasccTranslator>;

pub const BAD_DISPATCH: u16 = 10001;
pub const GENERAL_HOST_ERROR: u16 = 10002;

pub struct WasccTranslator {}
impl Translate for WasccTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			BAD_DISPATCH => "bad dispatch",
			GENERAL_HOST_ERROR => "general host error",
			_ => "unknown",
		}
	}
}

pub fn new_wascc_error_code(code: u16) -> WasccErrorCode {
	ErrorCodeTranslator::new(code, WasccTranslator {})
}
