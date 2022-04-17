use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type ClientErrorCode = ErrorCodeTranslator<ClientTranslator>;

pub const GENERAL_CLIENT_ERROR: u16 = 40000;
pub const LIBP2P_GENERAL_ERROR: u16 = 40001;
pub const PARSE_LIBP2P_ADDRESS_ERROR: u16 = 40002;
pub const IPFS_GENERAL_ERROR: u16 = 40003;
pub const RPC_CLIENT_GENERAL_ERROR: u16 = 40004;
pub const ROCKSDB_GENERAL_ERROR: u16 = 40005;

pub struct ClientTranslator {}
impl Translate for ClientTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			GENERAL_CLIENT_ERROR => "general client error",
			// libp2p
			LIBP2P_GENERAL_ERROR => "libp2p error",
			PARSE_LIBP2P_ADDRESS_ERROR => "parse libp2p address error",
			// ipfs
			IPFS_GENERAL_ERROR => "ipfs error",
			// rpc client
			RPC_CLIENT_GENERAL_ERROR => "rpc client general error",
			// rocksdb
			ROCKSDB_GENERAL_ERROR => "rocksdb error",
			_ => "unknown",
		}
	}
}

pub fn new_client_error_code(code: u16) -> ClientErrorCode {
	ErrorCodeTranslator::new(code, ClientTranslator {})
}
