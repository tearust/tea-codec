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

pub const OP_GET_ENV_VAR: &str = "GetEnvVar";
pub const OP_GET_SYSTEM_TIME: &str = "GetSystemTime";
pub const OP_CURRENT_TIMESTAMP: &str = "CurrentTimestamp";
pub const OP_TASK_START: &str = "TaskStart";
pub const OP_TASK_END: &str = "TaskEnd";

pub const OP_GET_TEA_ID: &str = "GetTeaId";
pub const OP_EPHEMERAL_PUB_KEY: &str = "GetEphemeralPubKey";
pub const OP_EPHEMERAL_PRI_KEY: &str = "GetEphemeralPriKey";

pub const OP_NITRO_ATTESTATION_DOC: &str = "AttestationDocument";
pub const OP_NITRO_GEN_RANDOM: &str = "GenerateRandom";
pub const OP_NITRO_GEN_UUID: &str = "GenerateUuid";
pub const OP_NITRO_VERIFY_ATTESTATION: &str = "VerifyAttestation";
pub const OP_NITRO_VERIFICATION_PRCS: &str = "VerificationPcrs";
pub const OP_NITRO_GET_ATTESTATION_DOC_PARAMS: &str = "GetAttestationDocParams";

/// This op is called frequently in a infinity loop
/// It will check this replica conveyor to find if there is any
/// txn TSID passed the global immutable checkpoint.
/// Global immutable is a point-of-no-return that no other existing
/// replica can insert new txn ahead. This means this txn can
/// be executed without convern of rebasing.
/// After this op is called, the tsid should be sent (called) to
/// the txn handler(or executor). Once the executor returns, the state has
/// been changed / updated by the executor. Then loop back again
/// to call this function the second time to find out next ready
/// txn. If there is no ready txn, then waiti a short period of time
/// then try again
pub const OP_POP_READY_TSID: &str = "PopReadyTsid";

///List all actor names vs their actor pubkeys
///
// pub const ACTOR_NAME_SIMPLE: &'static str = "simple-actor";
pub const ACTOR_PUBKEY_SIMPLE: &'static str =
	"MAPIZDLP2GN7QDSS3I74NCZEQOU2YXGZ4UROPRLDJI6ZZFT4TLJTOIGZ";
// pub const ACTOR_NAME_RA: &'static str = "ra";
pub const ACTOR_PUBKEY_RA: &'static str =
	"MDAELGEJDJPPMBHCVTXRCS6TYMEVOSEYLYHPDVWVBBRHNQ6KNIEBHNHQ";
pub const ACTOR_PUBKEY_DELEGATE: &str = "MAVB7LZ22BBLNW5SL3EZP7A6NTFGZFHFJVSFSPUINTLR5YQXR3LZVO7H";
pub const ACTOR_PUBKEY_TAPP_BBS: &str = "MDDRF3P5VBJ3WRP7YZ7BPU63C33ABJ52ZHANJH6YB3ALM7Q6T63DHUUM";
pub const ACTOR_PUBKEY_TAPP_FLUENCER: &str =
	"MBXQX7LCZOJXLS5F2QR6R2R3SG3D3RN7LWEVRJ3IVB6B2I6ZSJRNSCJR";

pub const ACTOR_PUBKEY_TAPPSTORE: &str = "MCSUSRIOFUNAKY5PCJKFS5WV56TLU533GELP4VADUPCAINFTBQMQE2VC";
// pub const ACTOR_NAME_INTERCOM: &'static str = "intercom-actor";
pub const HOST: &'static str = "host";
pub const ACTOR_PUBKEY_INTERCOM: &'static str =
	"MAXJLKOWEXXNPMWYXJMBNTGZAURUCLPL7JYDA4L357IICGMSC4ATJUZM";
pub const ACTOR_PUBKEY_REPORT: &str = "MBIDQCFY3PQAVHYUD56LXBG7VGYVLZRBZD4B6IXRS4UTSVYREL42UMDD";
pub const ACTOR_PUBKEY_VERSION: &str = "MBGSCA2SS6OZBTA4XAHCXZV45U67YC5FQVZYP5FRAMJFYUAFV3M6CRFR";
pub const ACTOR_PUBKEY_REPLICA_SERVICE: &str =
	"MD22BZQPUTOOYD5ER4EHZJH5IMQ37II36S4GIULVUB752EJRCZ3PLB2K";
pub const ACTOR_PUBKEY_TOKENSTATE_SERVICE: &str =
	"MBUOKG5KNAETVFYEOF7RODLJA7MDYWVZEBD2OOB57QAJBIQ4DOG3GJ3X";
pub const ACTOR_PUBKEY_LIBP2P_SERVICE: &str =
	"MCVOJGYDKP4SLQUQU4IWV2SN6LPDA3F4INOYHVVVNX6OGQXKIHR7W23D";

pub const ACTOR_PUBKEY_PARTY_CONTRACT: &str =
	"MARZR5YX7J4YFJMVNNH4QUTKXUIAPN7FRU5TSYJPQCAHWLYQUBEETTJI";

pub const ACTOR_PUBKEY_STATE_RECEIVER: &str =
	"MC737AMWEFY2IVC466Z2JW33J6NYOTHZPN4HGBXKLPEVO4PXEFKGYKI7";

///List all providers names vs their capability ids
///
pub const VMH_CAPABILITY_ID: &str = "tea:vmh-provider";
pub const INTERCOM_CAPABILITY_ID: &str = "tea:intercom";
pub const NITRO_CAPABILITY_ID: &str = "tea:nitro";
pub const KVP_CAPABILITY_ID: &str = "tea:keyvalue";
pub const ENV_CAPABILITY_ID: &str = "tea:env";
pub const RAFT_CAPABILITY_ID: &str = "tea:raft";
pub const LAYER1_CAPABILITY_ID: &str = "tea:layer1";
pub const ORBITDB_CAPABILITY_ID: &str = "tea:orbitdb";
pub const REPLICA_CAPABILITY_ID: &str = "tea:replica";
pub const TOKENSTATE_CAPABILITY_ID: &str = "tea:tokenstate";
pub const LIBP2P_CAPABILITY_ID: &str = "tea:libp2p";
pub const CONSOLE_CAPABILITY_ID: &str = "tea:console";
pub const CRYPTO_CAPABILITY_ID: &str = "tea:crypto";

pub const LIBP2P_REGISTER_SOCKET_NAME: &str = "/tmp/libp2p-register.socket";

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
pub mod ops;
pub mod ra;
pub mod task;
pub mod task_in_block;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadActorMessage {
	pub manifest: String,
	#[serde(with = "serde_bytes")]
	#[serde(default)]
	pub wasm_bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunActorWithParams {
	pub manifest: String,
	#[serde(with = "serde_bytes")]
	#[serde(default)]
	pub actor_bytes: Vec<u8>,
	#[serde(with = "serde_bytes")]
	#[serde(default)]
	pub params: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorVersionMessage {
	pub version: String,
}
