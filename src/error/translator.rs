use crate::error::code::ErrorCode;
use crate::TeaError;

pub trait Translate {
	fn translate(&self, code: u16) -> &'static str;
}

pub struct ErrorCodeTranslator<T>
where
	T: Translate,
{
	pub code: u16,
	translator: T,
}

pub const FAILED_TO_FIND_BLOCK: u16 = 30001;

impl<T> ErrorCodeTranslator<T>
where
	T: Translate,
{
	pub fn new(code: u16, translator: T) -> Self {
		ErrorCodeTranslator { code, translator }
	}

	pub fn to_error_code(&self, details: String) -> TeaError {
		TeaError::EncodedError(ErrorCode::new(self.code, self.to_string(), details))
	}
}

impl<T> ToString for ErrorCodeTranslator<T>
where
	T: Translate,
{
	fn to_string(&self) -> String {
		self.translator.translate(self.code).to_string()
	}
}

impl<T> Into<ErrorCode> for ErrorCodeTranslator<T>
where
	T: Translate,
{
	fn into(self) -> ErrorCode {
		ErrorCode::new_slim(self.code, self.to_string())
	}
}
