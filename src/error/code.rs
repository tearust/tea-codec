use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub mod client;
pub mod common;
pub mod layer1;
pub mod service;
pub mod vmh;
pub mod wascc;

#[derive(Error, Clone, Deserialize, Serialize)]
pub struct ErrorCode {
	code: u16,
	description: String,
	details: Option<String>,
	inner: Box<Option<ErrorCode>>,
}

impl PartialEq for ErrorCode {
	fn eq(&self, other: &Self) -> bool {
		self.code_chain().eq(&other.code_chain())
	}
}

impl ErrorCode {
	pub fn new(code: u16, description: String, details: Option<String>) -> Self {
		ErrorCode {
			code,
			description,
			details,
			inner: Default::default(),
		}
	}

	pub fn new_slim(code: u16, description: String) -> Self {
		ErrorCode {
			code,
			description,
			details: Default::default(),
			inner: Default::default(),
		}
	}

	pub fn new_nested(
		code: u16,
		description: String,
		details: Option<String>,
		inner: Option<ErrorCode>,
	) -> Self {
		ErrorCode {
			code,
			description,
			details,
			inner: Box::new(inner),
		}
	}

	pub fn code(&self) -> u16 {
		self.code
	}

	pub fn inner(&self) -> &Option<ErrorCode> {
		self.inner.as_ref()
	}

	pub fn details(&self) -> Option<&String> {
		self.details.as_ref()
	}

	pub fn contains(&self, code: u16) -> bool {
		if self.code == code {
			true
		} else {
			if let Some(inner) = self.inner.as_ref() {
				return inner.contains(code);
			}

			false
		}
	}

	fn error_chain_message(&self) -> String {
		let mut rtn = self.error_message(&self.description, self.details.as_ref());
		if let Some(e) = self.inner.as_ref() {
			rtn = format!("{}\r\t{}", rtn, e.error_chain_message());
		}
		rtn
	}

	fn error_message(&self, desc: &str, details: Option<&String>) -> String {
		if let Some(details) = details {
			format!("{}: {}, details: {}", self.print_code(), desc, details)
		} else {
			format!("{}: {}", self.print_code(), desc)
		}
	}

	fn print_code(&self) -> String {
		// TEC is the abbreviation for "Tea Error Code"
		format!("TEC-{}", self.code)
	}

	fn code_chain(&self) -> Vec<u16> {
		let mut chain = Vec::new();
		let mut cursor = Some(self);
		while let Some(e) = cursor {
			chain.push(e.code);
			cursor = (*e.inner).as_ref();
		}
		chain
	}
}

impl Debug for ErrorCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.error_chain_message())
	}
}

impl Display for ErrorCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.error_chain_message())
	}
}
