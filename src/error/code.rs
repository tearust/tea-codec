use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub mod layer1;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCode(pub u16, pub String);

impl Display for ErrorCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.error_message(&self.1))
	}
}

impl ErrorCode {
	fn error_message(&self, desc: &str) -> String {
		format!("{}: {}", self.print_code(), desc)
	}

	fn print_code(&self) -> String {
		// TEC is the abbreviation for "Tea Error Code"
		format!("TEC-{}", self.0)
	}
}
