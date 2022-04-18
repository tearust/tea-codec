use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type Layer1ErrorCode = ErrorCodeTranslator<Layer1Translator, Layer1Code>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Layer1Code {
	GeneralLayer1Error = 30000,
	FailedToFindBlock,
	UnknownMinerClass,
	UnknownMinerStatus,
	ParseAccountError,
	GetStorageError,
	Layer1ClientRPCGeneral,
	ParseAddressError,
	Layer1EventError,
	ParseH256Error,
	ParseTeaIdError,
	ParseBalanceError,
}

impl From<u16> for Layer1Code {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(Layer1Code::GeneralLayer1Error)
	}
}

pub struct Layer1Translator {}
impl Translate<Layer1Code> for Layer1Translator {
	fn translate(&self, code: Layer1Code) -> &'static str {
		match code {
			Layer1Code::GeneralLayer1Error => "general layer1 error",
			// general
			Layer1Code::FailedToFindBlock => "Failed to find layer1 block at specified height",
			Layer1Code::ParseAccountError => "parse account error",
			Layer1Code::ParseAddressError => "parse address error",
			Layer1Code::ParseH256Error => "parse h256 error",
			Layer1Code::ParseTeaIdError => "parse TEA ID error",
			Layer1Code::ParseBalanceError => "parse balance error",
			// cml
			Layer1Code::UnknownMinerClass => "unknown miner class convert value",
			Layer1Code::UnknownMinerStatus => "unknown miner status convert value",
			// layer1 client
			Layer1Code::GetStorageError => "get storage failed",
			Layer1Code::Layer1ClientRPCGeneral => "layer1 client RPC general error",
			// event
			Layer1Code::Layer1EventError => "layer1 event error",
		}
	}
}

pub fn new_layer1_error_code(code: Layer1Code) -> Layer1ErrorCode {
	ErrorCodeTranslator::new(code as u16, Layer1Translator {})
}
