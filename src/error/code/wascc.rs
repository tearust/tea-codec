use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type WasccErrorCode = ErrorCodeTranslator<WasccTranslator>;

pub const BAD_DISPATCH: u16 = 10001;
pub const GENERAL_HOST_ERROR: u16 = 10002;
pub const NO_SUCH_FUNCTION: u16 = 10003;
pub const WASM_MISC: u16 = 10004;
pub const HOST_CALL_FAILURE: u16 = 10005;
pub const GUEST_CALL_FAILURE: u16 = 10006;
pub const WAPC_GENERAL_ERROR: u16 = 10007;
pub const WASCAP_GENERAL_ERROR: u16 = 10008;
pub const HOST_AUTHORIZATION_ERROR: u16 = 10009;
pub const CAPABILITY_PROVIDER_ERROR: u16 = 10010;
pub const MISC_HOST_ERROR: u16 = 10011;
pub const PLUGIN_ERROR: u16 = 10012;
pub const MIDDLEWARE_ERROR: u16 = 10013;
pub const ACTOR_TO_ACTOR_CALL_NOT_EXIST: u16 = 10014;
pub const INVOCATION_ERROR: u16 = 10015;
pub const KEY_VALUE_ERROR: u16 = 10016;
pub const MESSAGING_ERROR: u16 = 10017;
pub const ENV_VAR_ERROR: u16 = 10018;
pub const DISCARD_MESSAGE_ERROR: u16 = 10019;

pub struct WasccTranslator {}
impl Translate for WasccTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			// wapc
			GENERAL_HOST_ERROR => "general host error",
			NO_SUCH_FUNCTION => "No such function in Wasm module",
			WASM_MISC => "WebAssembly failure",
			HOST_CALL_FAILURE => "Error occurred during host call",
			GUEST_CALL_FAILURE => "Guest call failure",
			WAPC_GENERAL_ERROR => "waPC error",
			WASCAP_GENERAL_ERROR => "Embedded JWT Failure",
			// wascc host
			HOST_AUTHORIZATION_ERROR => "Module authorization failure",
			CAPABILITY_PROVIDER_ERROR => "Capability provider failure",
			MISC_HOST_ERROR => "waSCC Host error",
			PLUGIN_ERROR => "Plugin error",
			MIDDLEWARE_ERROR => "Middleware error",
			ACTOR_TO_ACTOR_CALL_NOT_EXIST => "Attempted actor-to-actor call to non-existent target",
			INVOCATION_ERROR => "Invocation failure",
			// providers & dispatch
			BAD_DISPATCH => "bad dispatch",
			KEY_VALUE_ERROR => "Key/value store error",
			MESSAGING_ERROR => "Messaging error",
			ENV_VAR_ERROR => "Environment variable error",
			DISCARD_MESSAGE_ERROR => "discard message",
			_ => "unknown",
		}
	}
}

pub fn new_wascc_error_code(code: u16) -> WasccErrorCode {
	ErrorCodeTranslator::new(code, WasccTranslator {})
}
