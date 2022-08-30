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

extern crate self as tea_codec;

/// The version of the codec as seen on crates.io
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[macro_use]
extern crate num_derive;

pub use errorx::define_scope;
use errorx::{CannotBeNone, Error, Scope};
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

pub trait ResultExt {
	type Value;
	type Error;
	fn err_into<E>(self) -> Result<Self::Value, E>
	where
		E: From<Self::Error>;
}

impl<T, E> ResultExt for std::result::Result<T, E> {
	type Value = T;
	type Error = E;
	fn err_into<E2>(self) -> Result<Self::Value, E2>
	where
		E2: From<E>,
	{
		self.map_err(From::from)
	}
}

pub trait OptionExt {
	type Value;
	fn ok_or_err<S>(self, name: impl Into<String>) -> Result<Self::Value, Error<S>>
	where
		S: Scope;
}

impl<T> OptionExt for Option<T> {
	type Value = T;
	fn ok_or_err<S>(self, name: impl Into<String>) -> Result<Self::Value, Error<S>>
	where
		S: Scope,
	{
		self.ok_or_else(move || CannotBeNone(name.into()).into())
	}
}

pub mod error;
pub mod errorx;
#[cfg(test)]
mod tests;
