use crate::error::code::ErrorCode;
use crate::TeaError;
use std::marker::PhantomData;

pub trait Translate<C>
where
	C: From<u16>,
{
	fn translate(&self, code: C) -> &'static str;
}

pub struct ErrorCodeTranslator<T, C>
where
	T: Translate<C>,
	C: From<u16>,
{
	pub code: u16,
	translator: T,
	phantom_data: PhantomData<C>,
}

impl<T, C> ErrorCodeTranslator<T, C>
where
	T: Translate<C>,
	C: From<u16>,
{
	pub fn new(code: u16, translator: T) -> Self {
		ErrorCodeTranslator {
			code,
			translator,
			phantom_data: PhantomData::default(),
		}
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
		match e.parse_error_code() {
			Some(code) => self.to_error_code(None, Some(code)),
			None => self.to_error_code(Some(format!("{:?}", e)), None),
		}
	}
}

impl<T, C> ToString for ErrorCodeTranslator<T, C>
where
	T: Translate<C>,
	C: From<u16>,
{
	fn to_string(&self) -> String {
		self.translator.translate(C::from(self.code)).to_string()
	}
}

impl<T, C> Into<ErrorCode> for ErrorCodeTranslator<T, C>
where
	T: Translate<C>,
	C: From<u16>,
{
	fn into(self) -> ErrorCode {
		ErrorCode::new_slim(self.code, self.to_string())
	}
}
