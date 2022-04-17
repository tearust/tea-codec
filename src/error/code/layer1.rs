use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type Layer1ErrorCode = ErrorCodeTranslator<Layer1Translator>;

pub const FAILED_TO_FIND_BLOCK: u16 = 30001;
pub const UNKNOWN_MINER_CLASS: u16 = 30002;
pub const UNKNOWN_MINER_STATUS: u16 = 30003;
pub const PARSE_ACCOUNT_ERROR: u16 = 30004;
pub const GET_STORAGE_ERROR: u16 = 30005;
pub const LAYER1_CLIENT_RPC_GENERAL: u16 = 30006;
pub const PARSE_ADDRESS_ERROR: u16 = 30007;
pub const LAYER1_EVENT_ERROR: u16 = 30008;
pub const PARSE_H256_ERROR: u16 = 30009;
pub const PARSE_TEA_ID_ERROR: u16 = 30010;
pub const PARSE_BALANCE_ERROR: u16 = 30011;

pub struct Layer1Translator {}
impl Translate for Layer1Translator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			// general
			FAILED_TO_FIND_BLOCK => "Failed to find layer1 block at specified height",
			PARSE_ACCOUNT_ERROR => "parse account error",
			PARSE_ADDRESS_ERROR => "parse address error",
			PARSE_H256_ERROR => "parse h256 error",
			PARSE_TEA_ID_ERROR => "parse TEA ID error",
			PARSE_BALANCE_ERROR => "parse balance error",
			// cml
			UNKNOWN_MINER_CLASS => "unknown miner class convert value",
			UNKNOWN_MINER_STATUS => "unknown miner status convert value",
			// layer1 client
			GET_STORAGE_ERROR => "get storage failed",
			LAYER1_CLIENT_RPC_GENERAL => "layer1 client RPC general error",
			// event
			LAYER1_EVENT_ERROR => "layer1 event error",
			_ => "unknown",
		}
	}
}

pub fn new_layer1_error_code(code: u16) -> Layer1ErrorCode {
	ErrorCodeTranslator::new(code, Layer1Translator {})
}
