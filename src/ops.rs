pub mod crypto;
pub mod env;
pub mod kvp;
pub mod layer1;
pub mod libp2p;
pub mod persist;
pub mod replica;
pub mod state_receiver;
pub mod token_state;

pub const GENERAL_REPLY_PREFIX: &str = "reply";
pub const GENERAL_INTERFACE_PREFIX: &str = "actor";

/// The general reply subject
pub fn general_callback_reply(actor: &str, seq_number: u64) -> String {
	format!("{}.{}.{}", GENERAL_REPLY_PREFIX, actor, seq_number)
}

pub fn general_actor_interface(actor: &str, action: &str) -> String {
	format!("{}.{}.{}", GENERAL_INTERFACE_PREFIX, actor, action)
}
