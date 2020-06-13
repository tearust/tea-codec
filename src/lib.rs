
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
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(item: T) -> ::std::result::Result<Vec<u8>, Box<dyn ::std::error::Error>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn ::std::error::Error>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}

pub mod task_in_block;
