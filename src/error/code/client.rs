use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type ClientErrorCode = ErrorCodeTranslator<ClientTranslator, ClientCode>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum ClientCode {
	GeneralClientError = 40000,
	Libp2pGeneralError,
	ParseLibp2pAddressError,
	IpfsGeneralError,
	RPCClientGeneralError,
	RocksdbGeneralError,
}

pub struct ClientTranslator {}
impl Translate<ClientCode> for ClientTranslator {
	fn translate(&self, code: ClientCode) -> &'static str {
		match code {
			ClientCode::GeneralClientError => "general client error",
			// libp2p
			ClientCode::Libp2pGeneralError => "libp2p error",
			ClientCode::ParseLibp2pAddressError => "parse libp2p address error",
			// ipfs
			ClientCode::IpfsGeneralError => "ipfs error",
			// rpc client
			ClientCode::RPCClientGeneralError => "rpc client general error",
			// rocksdb
			ClientCode::RocksdbGeneralError => "rocksdb error",
		}
	}
}

impl From<u16> for ClientCode {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(ClientCode::GeneralClientError)
	}
}

pub fn new_client_error_code(code: ClientCode) -> ClientErrorCode {
	ErrorCodeTranslator::new(code as u16, ClientTranslator {})
}
