use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type Layer1ErrorCode = ErrorCodeTranslator<Layer1Translator>;

pub const FAILED_TO_FIND_BLOCK: u16 = 30001;

pub struct Layer1Translator {}
impl Translate for Layer1Translator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			FAILED_TO_FIND_BLOCK => "Failed to find layer1 block at specified height",
			_ => "unknown",
		}
	}
}

pub fn new_layer1_error_code(code: u16) -> Layer1ErrorCode {
	ErrorCodeTranslator::new(code, Layer1Translator {})
}
