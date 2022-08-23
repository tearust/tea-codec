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

#![feature(thin_box)]
#![feature(auto_traits, negative_impls, never_type)]
#![feature(min_specialization)]
#![feature(generic_associated_types)]

/// The version of the codec as seen on crates.io
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[macro_use]
extern crate num_derive;

use serde::de::DeserializeOwned;
use serde::Serialize;
pub type Result<T, E = errorx::Error<errorx::Global>> = std::result::Result<T, E>;

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(item: &T) -> Result<Vec<u8>>
where
	T: Serialize,
{
	let buf = bincode::serialize(item)?;
	Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<T, B>(buf: B) -> Result<T>
where
	T: DeserializeOwned,
	B: AsRef<[u8]>,
{
	Ok(bincode::deserialize_from(buf.as_ref())?)
}

pub mod error;
pub mod errorx;
#[cfg(test)]
mod tests;
