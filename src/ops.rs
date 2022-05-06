pub mod crypto;
pub mod env;
pub mod layer1;
pub mod libp2p;
pub mod persist;
pub mod replica;
pub mod state_receiver;
pub mod token_state;

pub const GENERAL_REPLY_PREFIX: &str = "reply";

/// The general reply subject
pub fn general_callback_reply(actor: &str, seq_number: u64) -> String {
	format!("{}.{}.{}", GENERAL_REPLY_PREFIX, actor, seq_number)
}
