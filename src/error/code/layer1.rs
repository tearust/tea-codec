use crate::error::code::ErrorCode;

pub struct Layer1ErrorCode(pub u16);

pub const FAILED_TO_FIND_BLOCK: u16 = 30001;

impl Layer1ErrorCode {
	fn translate(&self) -> &'static str {
		match self.0 {
			FAILED_TO_FIND_BLOCK => "Failed to find layer1 block at specified height",
			_ => "unknown",
		}
	}
}

impl ToString for Layer1ErrorCode {
	fn to_string(&self) -> String {
		self.translate().to_string()
	}
}

impl From<u16> for Layer1ErrorCode {
	fn from(v: u16) -> Self {
		Layer1ErrorCode(v)
	}
}

impl Into<ErrorCode> for Layer1ErrorCode {
	fn into(self) -> ErrorCode {
		ErrorCode(self.0, self.to_string())
	}
}
