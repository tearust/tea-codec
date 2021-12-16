pub mod crypto;
pub mod layer1;

pub const GENERAL_REPLY_PREFIX: &str = "reply";
pub const LAYER1_ASYNC_REPLY_PREFIX: &str = "layer1_async";

/// The general reply subject
pub fn general_callback_reply(actor: &str, uuid: &str) -> String {
	format!("{}.{}.{}", GENERAL_REPLY_PREFIX, actor, uuid)
}

pub fn layer1_async_reply(actor: &str, uuid: &str) -> String {
	format!("{}.{}.{}", LAYER1_ASYNC_REPLY_PREFIX, actor, uuid)
}