use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type ServiceErrorCode = ErrorCodeTranslator<ServiceTranslator, ServiceCode>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum ServiceCode {
	GeneralServiceError = 50000,
	GlueSQLError,
	StateGeneralError,
	HttpExecutionError,
	BondingGeneralError,
	DbNotFoundError,
	InvalidTransactionContext,
}

impl From<u16> for ServiceCode {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(ServiceCode::GeneralServiceError)
	}
}

pub struct ServiceTranslator {}
impl Translate<ServiceCode> for ServiceTranslator {
	fn translate(&self, code: ServiceCode) -> &'static str {
		match code {
			// general
			ServiceCode::GeneralServiceError => "general service error",
			// gluedb
			ServiceCode::GlueSQLError => "glue sql error",
			ServiceCode::DbNotFoundError => "gluedb not found",
			// state machine
			ServiceCode::StateGeneralError => "state machine error",
			ServiceCode::BondingGeneralError => "bonding curve error",
			ServiceCode::InvalidTransactionContext => "invalid transaction context",
			// utility
			ServiceCode::HttpExecutionError => "http execution error",
		}
	}
}

pub fn new_service_error_code(code: ServiceCode) -> ServiceErrorCode {
	ErrorCodeTranslator::new(code as u16, ServiceTranslator {})
}
