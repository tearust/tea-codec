use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub mod common;
pub mod layer1;
pub mod wascc;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCode {
	pub code: u16,
	pub description: String,
	pub details: String,
}

impl Display for ErrorCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:?}",
			self.error_message(&self.description, &self.details)
		)
	}
}

impl PartialEq for ErrorCode {
	fn eq(&self, other: &Self) -> bool {
		self.code.eq(&other.code)
	}
}

impl ErrorCode {
	pub fn new(code: u16, description: String, details: String) -> Self {
		ErrorCode {
			code,
			description,
			details,
		}
	}

	pub fn new_slim(code: u16, description: String) -> Self {
		ErrorCode {
			code,
			description,
			details: Default::default(),
		}
	}

	fn error_message(&self, desc: &str, details: &str) -> String {
		if details.is_empty() {
			format!("{}: {}", self.print_code(), desc)
		} else {
			format!("{}: {}, details: {}", self.print_code(), desc, details)
		}
	}

	fn print_code(&self) -> String {
		// TEC is the abbreviation for "Tea Error Code"
		format!("TEC-{}", self.code)
	}
}
