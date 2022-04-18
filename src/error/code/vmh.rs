use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type VmhErrorCode = ErrorCodeTranslator<VmhTranslator, VmhCode>;

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum VmhCode {
	GeneralVmhError = 20000,
	SocketSendU64Error,
	SocketSendLoopError,
	SocketRecvU64Error,
	SocketRecvLoopError,
	SocketClientDisconnected,
	SocketServerClosed,
	SocketRecvSizeMismatched,
	SocketSendSizeMismatched,
	SocketNixError,
	QuitReceiverLoop,
	OperationInvalid,
	SenderOperationExists,
	InboundNetError,
	OutboundNetError,
}

impl From<u16> for VmhCode {
	fn from(v: u16) -> Self {
		num_traits::FromPrimitive::from_u16(v).unwrap_or(VmhCode::GeneralVmhError)
	}
}

pub struct VmhTranslator {}
impl Translate<VmhCode> for VmhTranslator {
	fn translate(&self, code: VmhCode) -> &'static str {
		match code {
			VmhCode::GeneralVmhError => "general vmh error",
			// socket basic
			VmhCode::SocketSendU64Error => "socket send u64 error",
			VmhCode::SocketRecvU64Error => "socket receive u64 error",
			VmhCode::SocketSendLoopError => "socket send loop error",
			VmhCode::SocketRecvLoopError => "socket receive loop error",
			VmhCode::SocketClientDisconnected => "client has disconnected",
			VmhCode::SocketServerClosed => "server has closed",
			VmhCode::SocketRecvSizeMismatched => "received bytes not matched",
			VmhCode::SocketSendSizeMismatched => "send bytes not matched",
			VmhCode::SocketNixError => "socket nix error",
			// socket server/client
			VmhCode::QuitReceiverLoop => "receiver loop has terminated",
			// channel
			VmhCode::OperationInvalid => "operation is invalid",
			VmhCode::SenderOperationExists => "sender operation already exists",
			VmhCode::InboundNetError => "inbound net error",
			VmhCode::OutboundNetError => "outbound net error",
		}
	}
}

pub fn new_vmh_error_code(code: VmhCode) -> VmhErrorCode {
	ErrorCodeTranslator::new(code as u16, VmhTranslator {})
}
