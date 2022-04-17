use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type ServiceErrorCode = ErrorCodeTranslator<ServiceTranslator>;

pub const GENERAL_SERVICE_ERROR: u16 = 50000;
pub const GLUE_SQL_ERROR: u16 = 50001;
pub const STATE_GENERAL_ERROR: u16 = 50002;
pub const HTTP_EXECUTION_ERROR: u16 = 50003;
pub const BONDING_GENERAL_ERROR: u16 = 50004;
pub const DB_NOT_FOUND_ERROR: u16 = 50005;
pub const INVALID_TRANSACTION_CONTEXT: u16 = 50006;

pub struct ServiceTranslator {}
impl Translate for ServiceTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			// general
			GENERAL_SERVICE_ERROR => "general service error",
			// gluedb
			GLUE_SQL_ERROR => "glue sql error",
			DB_NOT_FOUND_ERROR => "gluedb not found",
			// state machine
			STATE_GENERAL_ERROR => "state machine error",
			BONDING_GENERAL_ERROR => "bonding curve error",
			INVALID_TRANSACTION_CONTEXT => "invalid transaction context",
			// utility
			HTTP_EXECUTION_ERROR => "http execution error",

			_ => "unknown",
		}
	}
}

pub fn new_service_error_code(code: u16) -> ServiceErrorCode {
	ErrorCodeTranslator::new(code, ServiceTranslator {})
}
