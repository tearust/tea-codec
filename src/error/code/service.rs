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
	InvalidValidator,
	InvalidTxnRequest,
	AsyncCanceled,
	AsyncNotFinished,
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
			// state receiver
			ServiceCode::InvalidValidator => "i'm not a validator",
			ServiceCode::InvalidTxnRequest => "txn request send from other node is invalid",
			// utility
			ServiceCode::HttpExecutionError => "http execution error",
			// async
			ServiceCode::AsyncCanceled => "async canceled",
			ServiceCode::AsyncNotFinished => "async not finished",
		}
	}
}

pub fn new_service_error_code(code: ServiceCode) -> ServiceErrorCode {
	ErrorCodeTranslator::new(code as u16, ServiceTranslator {})
}
