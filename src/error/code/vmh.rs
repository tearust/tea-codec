use crate::error::translator::{ErrorCodeTranslator, Translate};

pub type VmhErrorCode = ErrorCodeTranslator<VmhTranslator>;

pub const GENERAL_VMH_ERROR: u16 = 20000;
pub const SOCKET_SEND_U64_ERROR: u16 = 20001;
pub const SOCKET_SEND_LOOP_ERROR: u16 = 20002;
pub const SOCKET_RECV_U64_ERROR: u16 = 20003;
pub const SOCKET_RECV_LOOP_ERROR: u16 = 20004;
pub const SOCKET_CLIENT_DISCONNECTED: u16 = 20005;
pub const SOCKET_SERVER_CLOSED: u16 = 20006;
pub const SOCKET_RECV_SIZE_MISMATCHED: u16 = 20007;
pub const SOCKET_SEND_SIZE_MISMATCHED: u16 = 20008;
pub const SOCKET_NIX_ERROR: u16 = 20009;
pub const QUIT_RECEIVER_LOOP: u16 = 20010;
pub const OPERATION_INVALID: u16 = 20011;
pub const SENDER_OPERATION_EXISTS: u16 = 20012;
pub const INBOUND_NET_ERROR: u16 = 20013;
pub const OUTBOUND_NET_ERROR: u16 = 20014;

pub struct VmhTranslator {}
impl Translate for VmhTranslator {
	fn translate(&self, code: u16) -> &'static str {
		match code {
			GENERAL_VMH_ERROR => "general vmh error",
			// socket basic
			SOCKET_SEND_U64_ERROR => "socket send u64 error",
			SOCKET_RECV_U64_ERROR => "socket receive u64 error",
			SOCKET_SEND_LOOP_ERROR => "socket send loop error",
			SOCKET_RECV_LOOP_ERROR => "socket receive loop error",
			SOCKET_CLIENT_DISCONNECTED => "client has disconnected",
			SOCKET_SERVER_CLOSED => "server has closed",
			SOCKET_RECV_SIZE_MISMATCHED => "received bytes not matched",
			SOCKET_SEND_SIZE_MISMATCHED => "send bytes not matched",
			SOCKET_NIX_ERROR => "socket nix error",
			// socket server/client
			QUIT_RECEIVER_LOOP => "receiver loop has terminated",
			// channel
			OPERATION_INVALID => "operation is invalid",
			SENDER_OPERATION_EXISTS => "sender operation already exists",
			INBOUND_NET_ERROR => "inbound net error",
			OUTBOUND_NET_ERROR => "outbound net error",
			_ => "unknown",
		}
	}
}

pub fn new_vmh_error_code(code: u16) -> VmhErrorCode {
	ErrorCodeTranslator::new(code, VmhTranslator {})
}
