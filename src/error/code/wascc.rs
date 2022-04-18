use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type WasccErrorCode = ErrorCodeTranslator<WasccTranslator, WasccCode>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum WasccCode {
	GeneralWasccError = 10000,
	BadDispatch,
	GeneralHostError,
	NoSuchFunction,
	WasmMisc,
	HostCallFailure,
	GuestCallFailure,
	WapcGeneralError,
	WascapGeneralError,
	HostAuthorizationError,
	CapabilityProviderError,
	MiscHostError,
	PluginError,
	MiddlewareError,
	ActorToActorCallNotExist,
	InvocationError,
	KeyValueError,
	MessagingError,
	EnvVarError,
	DiscardMessageError,
}

impl From<u16> for WasccCode {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(WasccCode::GeneralWasccError)
	}
}

pub struct WasccTranslator {}
impl Translate<WasccCode> for WasccTranslator {
	fn translate(&self, code: WasccCode) -> &'static str {
		match code {
			WasccCode::GeneralWasccError => "general wascc error",
			// wapc
			WasccCode::GeneralHostError => "general host error",
			WasccCode::NoSuchFunction => "No such function in Wasm module",
			WasccCode::WasmMisc => "WebAssembly failure",
			WasccCode::HostCallFailure => "Error occurred during host call",
			WasccCode::GuestCallFailure => "Guest call failure",
			WasccCode::WapcGeneralError => "waPC error",
			WasccCode::WascapGeneralError => "Embedded JWT Failure",
			// wascc host
			WasccCode::HostAuthorizationError => "Module authorization failure",
			WasccCode::CapabilityProviderError => "Capability provider failure",
			WasccCode::MiscHostError => "waSCC Host error",
			WasccCode::PluginError => "Plugin error",
			WasccCode::MiddlewareError => "Middleware error",
			WasccCode::ActorToActorCallNotExist => {
				"Attempted actor-to-actor call to non-existent target"
			}
			WasccCode::InvocationError => "Invocation failure",
			// providers & dispatch
			WasccCode::BadDispatch => "bad dispatch",
			WasccCode::KeyValueError => "Key/value store error",
			WasccCode::MessagingError => "Messaging error",
			WasccCode::EnvVarError => "Environment variable error",
			WasccCode::DiscardMessageError => "discard message",
		}
	}
}

pub fn new_wascc_error_code(code: WasccCode) -> WasccErrorCode {
	ErrorCodeTranslator::new(code as u16, WasccTranslator {})
}
