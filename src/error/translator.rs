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

impl<T> ErrorCodeTranslator<T>
where
	T: Translate,
{
	pub fn new(code: u16, translator: T) -> Self {
		ErrorCodeTranslator { code, translator }
	}

	pub fn to_error_code(&self, details: Option<String>, inner: Option<ErrorCode>) -> TeaError {
		TeaError::EncodedError(ErrorCode::new_nested(
			self.code,
			self.to_string(),
			details,
			inner,
		))
	}

	pub fn error_from_nested(&self, e: TeaError) -> TeaError {
		let error_code = match e.parse_error_code() {
			Some(code) => ErrorCode::new_nested(self.code, self.to_string(), None, *code.inner),
			None => ErrorCode::new(self.code, self.to_string(), Some(format!("{:?}", e))),
		};
		TeaError::EncodedError(error_code)
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
