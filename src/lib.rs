//! tea-codec
//!
//! This codec library is an enhanced version of Kevin Hoffman's original [waSCC Codec](https://github.com/wascc/wascc-codec)
//! with the support of [tea-kvp-provider](https://github.com/tearust/tea-kvp-provider).
//!
//! # About the Tea Project
//!
//! Tea Project (Trusted Execution & Attestation) is a Wasm runtime build on top of RoT(Root of Trust)
//! from both trusted hardware environment and blockchain technologies. Developer, Host and Consumer
//! do not have to trust any others to not only protecting privacy but also preventing cyber attacks.
//! The execution environment under remoted attestation can be verified by blockchain consensys.
//! Crypto economy is used as motivation that hosts are willing run trusted computing nodes. This
//! platform can be used by CDN providers, IPFS Nodes or existing cloud providers to enhance existing
//! infrastructure to be more secure and trustless.
//!
//! Introduction [blog post](https://medium.com/@pushbar/0-of-n-cover-letter-of-the-trusted-webassembly-runtime-on-ipfs-12a4fd8c4338)
//!
//! Project [repo](http://github.com/tearust). More and more repo will be exposed soon.
//!
//! Yet to come //! project site [( not completed yet) http://www.t-rust.com/](http://www.t-rust.com/)
//!
//! Contact: kevin.zhang.canada_at_gmail_dot_com.
//!
//! We are just started, all kinds of help are welcome!
//!

/// The version of the codec as seen on crates.io
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[macro_use]
extern crate serde_derive;
extern crate log;

extern crate rmp_serde as rmps;
use crate::error::{TeaError, TeaResult};
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(item: T) -> TeaResult<Vec<u8>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())
        .map_err(|e| TeaError::SerializeError(format!("{}", e)))?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(buf: &[u8]) -> TeaResult<T> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(TeaError::DeserializeError(format!("{}", e))),
    }
}

pub const OP_DELAY_PUBLISH: &'static str = "DelayPublish";
pub const OP_TEST_RELAY: &'static str = "OP_TEST_RELAY";
pub const OP_INTERCOM_MESSAGE: &str = "IntercomMessage";

pub const OP_NITRO_ATTESTATION_DOC: &str = "AttestationDocument";
pub const OP_NITRO_GEN_RANDOM: &str = "GenerateRandom";

///List all actor names vs their actor pubkeys
///
pub const ACTOR_NAME_SIMPLE: &'static str = "simple-actor";
pub const ACTOR_PUBKEY_SIMPLE: &'static str =
    "MAPIZDLP2GN7QDSS3I74NCZEQOU2YXGZ4UROPRLDJI6ZZFT4TLJTOIGZ";
pub const ACTOR_NAME_INTERCOM: &'static str = "intercom-actor";
pub const HOST: &'static str = "host";
pub const ACTOR_PUBKEY_INTERCOM: &'static str =
    "MAXJLKOWEXXNPMWYXJMBNTGZAURUCLPL7JYDA4L357IICGMSC4ATJUZM";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayMessage {
    pub delay_seconds: u64,
    /// The message subject or topic
    pub subject: String,
    /// The reply-to field of the subject. This will be empty if there is no reply subject
    pub reply_to: String,
    /// The raw bytes of the message. Encoding/contents is determined by applications out of band
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

pub mod error;
pub mod ipfs_codec;
pub mod ra;
pub mod task;
pub mod task_in_block;
