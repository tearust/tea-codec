#![feature(prelude_import)]
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
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate self as tea_codec;
/// The version of the codec as seen on crates.io
pub const VERSION: &str = "0.1.1";
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
    fn ok_or_err_else<S, N, F>(self, name_factory: F) -> Result<Self::Value, Error<S>>
    where
        S: Scope,
        N: Into<String>,
        F: FnOnce() -> N;
}
impl<T> OptionExt for Option<T> {
    type Value = T;
    fn ok_or_err<S>(self, name: impl Into<String>) -> Result<Self::Value, Error<S>>
    where
        S: Scope,
    {
        self.ok_or_else(move || CannotBeNone(name.into()).into())
    }
    fn ok_or_err_else<S, N, F>(self, name_factory: F) -> Result<Self::Value, Error<S>>
    where
        S: Scope,
        N: Into<String>,
        F: FnOnce() -> N,
    {
        self.ok_or_else(move || CannotBeNone(name_factory().into()).into())
    }
}
pub mod error {
    use log::{warn, ParseLevelError};
    use serde::{Deserialize, Serialize};
    use serde_json::Error;
    use std::array::TryFromSliceError;
    use std::fmt::{Debug, Formatter};
    use std::num::{ParseIntError, TryFromIntError};
    use std::str::Utf8Error;
    use std::string::FromUtf8Error;
    use std::time::SystemTimeError;
    use thiserror::Error;
    pub mod code {
        use serde::{Deserialize, Serialize};
        use std::fmt::{Debug, Display, Formatter};
        use thiserror::Error;
        pub mod client {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type ClientErrorCode = ErrorCodeTranslator<ClientTranslator, ClientCode>;
            pub enum ClientCode {
                GeneralClientError = 40000,
                Libp2pGeneralError,
                ParseLibp2pAddressError,
                IpfsGeneralError,
                RPCClientGeneralError,
                RocksdbGeneralError,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ClientCode {}
            #[automatically_derived]
            impl ::core::clone::Clone for ClientCode {
                #[inline]
                fn clone(&self) -> ClientCode {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_ClientCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ClientCode {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == ClientCode::GeneralClientError as i64 {
                            Some(ClientCode::GeneralClientError)
                        } else if n == ClientCode::Libp2pGeneralError as i64 {
                            Some(ClientCode::Libp2pGeneralError)
                        } else if n == ClientCode::ParseLibp2pAddressError as i64 {
                            Some(ClientCode::ParseLibp2pAddressError)
                        } else if n == ClientCode::IpfsGeneralError as i64 {
                            Some(ClientCode::IpfsGeneralError)
                        } else if n == ClientCode::RPCClientGeneralError as i64 {
                            Some(ClientCode::RPCClientGeneralError)
                        } else if n == ClientCode::RocksdbGeneralError as i64 {
                            Some(ClientCode::RocksdbGeneralError)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_ClientCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ClientCode {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                ClientCode::GeneralClientError => {
                                    ClientCode::GeneralClientError as i64
                                }
                                ClientCode::Libp2pGeneralError => {
                                    ClientCode::Libp2pGeneralError as i64
                                }
                                ClientCode::ParseLibp2pAddressError => {
                                    ClientCode::ParseLibp2pAddressError as i64
                                }
                                ClientCode::IpfsGeneralError => {
                                    ClientCode::IpfsGeneralError as i64
                                }
                                ClientCode::RPCClientGeneralError => {
                                    ClientCode::RPCClientGeneralError as i64
                                }
                                ClientCode::RocksdbGeneralError => {
                                    ClientCode::RocksdbGeneralError as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            pub struct ClientTranslator {}
            impl Translate<ClientCode> for ClientTranslator {
                fn translate(&self, code: ClientCode) -> &'static str {
                    match code {
                        ClientCode::GeneralClientError => "general client error",
                        ClientCode::Libp2pGeneralError => "libp2p error",
                        ClientCode::ParseLibp2pAddressError => {
                            "parse libp2p address error"
                        }
                        ClientCode::IpfsGeneralError => "ipfs error",
                        ClientCode::RPCClientGeneralError => "rpc client general error",
                        ClientCode::RocksdbGeneralError => "rocksdb error",
                    }
                }
            }
            impl From<u16> for ClientCode {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(ClientCode::GeneralClientError)
                }
            }
            pub fn new_client_error_code(code: ClientCode) -> ClientErrorCode {
                ErrorCodeTranslator::new(code as u16, ClientTranslator {})
            }
        }
        pub mod common {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type CommonErrorCode = ErrorCodeTranslator<CommonTranslator, CommonCode>;
            pub enum CommonCode {
                CommonError = 0,
                NewError,
                SerdeSerializeError,
                SerdeDeserializeError,
                UTF8EncodingError,
                Utf8StrEncodingError,
                StdIoError,
                ChannelReceiveError,
                ChannelSendError,
                JsonMarshalingError,
                ProstEncodeError,
                ProstDecodeError,
                TryIntoConvertError,
                TryFromConvertError,
                ParseStringError,
                SystemTimeError,
                MutexLockFailed,
                JsonRPCParseError,
                OptionIsNone,
                SerdeGeneralError,
                HexDecodeError,
                Base64DecodeError,
                ReqwestGeneralError,
                LogGeneralError,
                BincodeSerializeError,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for CommonCode {}
            #[automatically_derived]
            impl ::core::clone::Clone for CommonCode {
                #[inline]
                fn clone(&self) -> CommonCode {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_CommonCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for CommonCode {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == CommonCode::CommonError as i64 {
                            Some(CommonCode::CommonError)
                        } else if n == CommonCode::NewError as i64 {
                            Some(CommonCode::NewError)
                        } else if n == CommonCode::SerdeSerializeError as i64 {
                            Some(CommonCode::SerdeSerializeError)
                        } else if n == CommonCode::SerdeDeserializeError as i64 {
                            Some(CommonCode::SerdeDeserializeError)
                        } else if n == CommonCode::UTF8EncodingError as i64 {
                            Some(CommonCode::UTF8EncodingError)
                        } else if n == CommonCode::Utf8StrEncodingError as i64 {
                            Some(CommonCode::Utf8StrEncodingError)
                        } else if n == CommonCode::StdIoError as i64 {
                            Some(CommonCode::StdIoError)
                        } else if n == CommonCode::ChannelReceiveError as i64 {
                            Some(CommonCode::ChannelReceiveError)
                        } else if n == CommonCode::ChannelSendError as i64 {
                            Some(CommonCode::ChannelSendError)
                        } else if n == CommonCode::JsonMarshalingError as i64 {
                            Some(CommonCode::JsonMarshalingError)
                        } else if n == CommonCode::ProstEncodeError as i64 {
                            Some(CommonCode::ProstEncodeError)
                        } else if n == CommonCode::ProstDecodeError as i64 {
                            Some(CommonCode::ProstDecodeError)
                        } else if n == CommonCode::TryIntoConvertError as i64 {
                            Some(CommonCode::TryIntoConvertError)
                        } else if n == CommonCode::TryFromConvertError as i64 {
                            Some(CommonCode::TryFromConvertError)
                        } else if n == CommonCode::ParseStringError as i64 {
                            Some(CommonCode::ParseStringError)
                        } else if n == CommonCode::SystemTimeError as i64 {
                            Some(CommonCode::SystemTimeError)
                        } else if n == CommonCode::MutexLockFailed as i64 {
                            Some(CommonCode::MutexLockFailed)
                        } else if n == CommonCode::JsonRPCParseError as i64 {
                            Some(CommonCode::JsonRPCParseError)
                        } else if n == CommonCode::OptionIsNone as i64 {
                            Some(CommonCode::OptionIsNone)
                        } else if n == CommonCode::SerdeGeneralError as i64 {
                            Some(CommonCode::SerdeGeneralError)
                        } else if n == CommonCode::HexDecodeError as i64 {
                            Some(CommonCode::HexDecodeError)
                        } else if n == CommonCode::Base64DecodeError as i64 {
                            Some(CommonCode::Base64DecodeError)
                        } else if n == CommonCode::ReqwestGeneralError as i64 {
                            Some(CommonCode::ReqwestGeneralError)
                        } else if n == CommonCode::LogGeneralError as i64 {
                            Some(CommonCode::LogGeneralError)
                        } else if n == CommonCode::BincodeSerializeError as i64 {
                            Some(CommonCode::BincodeSerializeError)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_CommonCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for CommonCode {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                CommonCode::CommonError => CommonCode::CommonError as i64,
                                CommonCode::NewError => CommonCode::NewError as i64,
                                CommonCode::SerdeSerializeError => {
                                    CommonCode::SerdeSerializeError as i64
                                }
                                CommonCode::SerdeDeserializeError => {
                                    CommonCode::SerdeDeserializeError as i64
                                }
                                CommonCode::UTF8EncodingError => {
                                    CommonCode::UTF8EncodingError as i64
                                }
                                CommonCode::Utf8StrEncodingError => {
                                    CommonCode::Utf8StrEncodingError as i64
                                }
                                CommonCode::StdIoError => CommonCode::StdIoError as i64,
                                CommonCode::ChannelReceiveError => {
                                    CommonCode::ChannelReceiveError as i64
                                }
                                CommonCode::ChannelSendError => {
                                    CommonCode::ChannelSendError as i64
                                }
                                CommonCode::JsonMarshalingError => {
                                    CommonCode::JsonMarshalingError as i64
                                }
                                CommonCode::ProstEncodeError => {
                                    CommonCode::ProstEncodeError as i64
                                }
                                CommonCode::ProstDecodeError => {
                                    CommonCode::ProstDecodeError as i64
                                }
                                CommonCode::TryIntoConvertError => {
                                    CommonCode::TryIntoConvertError as i64
                                }
                                CommonCode::TryFromConvertError => {
                                    CommonCode::TryFromConvertError as i64
                                }
                                CommonCode::ParseStringError => {
                                    CommonCode::ParseStringError as i64
                                }
                                CommonCode::SystemTimeError => {
                                    CommonCode::SystemTimeError as i64
                                }
                                CommonCode::MutexLockFailed => {
                                    CommonCode::MutexLockFailed as i64
                                }
                                CommonCode::JsonRPCParseError => {
                                    CommonCode::JsonRPCParseError as i64
                                }
                                CommonCode::OptionIsNone => CommonCode::OptionIsNone as i64,
                                CommonCode::SerdeGeneralError => {
                                    CommonCode::SerdeGeneralError as i64
                                }
                                CommonCode::HexDecodeError => {
                                    CommonCode::HexDecodeError as i64
                                }
                                CommonCode::Base64DecodeError => {
                                    CommonCode::Base64DecodeError as i64
                                }
                                CommonCode::ReqwestGeneralError => {
                                    CommonCode::ReqwestGeneralError as i64
                                }
                                CommonCode::LogGeneralError => {
                                    CommonCode::LogGeneralError as i64
                                }
                                CommonCode::BincodeSerializeError => {
                                    CommonCode::BincodeSerializeError as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            pub struct CommonTranslator {}
            impl Translate<CommonCode> for CommonTranslator {
                fn translate(&self, code: CommonCode) -> &'static str {
                    match code {
                        CommonCode::CommonError => "common error",
                        CommonCode::NewError => "new error",
                        CommonCode::OptionIsNone => "option is none",
                        CommonCode::SystemTimeError => "system time error",
                        CommonCode::TryIntoConvertError => "try_into conversion error",
                        CommonCode::TryFromConvertError => "try_from conversion error",
                        CommonCode::ParseStringError => "parse string error",
                        CommonCode::MutexLockFailed => "mutex lock failed",
                        CommonCode::LogGeneralError => "log error",
                        CommonCode::SerdeGeneralError => "serde general error",
                        CommonCode::SerdeSerializeError => "serde serialize failed",
                        CommonCode::SerdeDeserializeError => "serde deserialize failed",
                        CommonCode::JsonMarshalingError => {
                            "JSON encoding/decoding failure"
                        }
                        CommonCode::BincodeSerializeError => {
                            "bincode serialization failure"
                        }
                        CommonCode::ProstEncodeError => "prost encoding error",
                        CommonCode::ProstDecodeError => "prost decoding error",
                        CommonCode::UTF8EncodingError => "utf8 encoding failed",
                        CommonCode::Utf8StrEncodingError => "utf8 string encoding failed",
                        CommonCode::HexDecodeError => "hex decoding error",
                        CommonCode::Base64DecodeError => "base64 decoding error",
                        CommonCode::StdIoError => "standard I/O error",
                        CommonCode::ChannelReceiveError => "channel receive error",
                        CommonCode::ChannelSendError => "channel send error",
                        CommonCode::JsonRPCParseError => "json rpc parse error",
                        CommonCode::ReqwestGeneralError => "reqwest error",
                    }
                }
            }
            impl From<u16> for CommonCode {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(CommonCode::CommonError)
                }
            }
            pub fn new_common_error_code(code: CommonCode) -> CommonErrorCode {
                ErrorCodeTranslator::new(code as u16, CommonTranslator {})
            }
        }
        pub mod layer1 {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type Layer1ErrorCode = ErrorCodeTranslator<Layer1Translator, Layer1Code>;
            pub enum Layer1Code {
                GeneralLayer1Error = 30000,
                FailedToFindBlock,
                UnknownMinerClass,
                UnknownMinerStatus,
                ParseAccountError,
                GetStorageError,
                Layer1ClientRPCGeneral,
                ParseAddressError,
                Layer1EventError,
                ParseH256Error,
                ParseTeaIdError,
                ParseBalanceError,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Layer1Code {}
            #[automatically_derived]
            impl ::core::clone::Clone for Layer1Code {
                #[inline]
                fn clone(&self) -> Layer1Code {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_Layer1Code: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for Layer1Code {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == Layer1Code::GeneralLayer1Error as i64 {
                            Some(Layer1Code::GeneralLayer1Error)
                        } else if n == Layer1Code::FailedToFindBlock as i64 {
                            Some(Layer1Code::FailedToFindBlock)
                        } else if n == Layer1Code::UnknownMinerClass as i64 {
                            Some(Layer1Code::UnknownMinerClass)
                        } else if n == Layer1Code::UnknownMinerStatus as i64 {
                            Some(Layer1Code::UnknownMinerStatus)
                        } else if n == Layer1Code::ParseAccountError as i64 {
                            Some(Layer1Code::ParseAccountError)
                        } else if n == Layer1Code::GetStorageError as i64 {
                            Some(Layer1Code::GetStorageError)
                        } else if n == Layer1Code::Layer1ClientRPCGeneral as i64 {
                            Some(Layer1Code::Layer1ClientRPCGeneral)
                        } else if n == Layer1Code::ParseAddressError as i64 {
                            Some(Layer1Code::ParseAddressError)
                        } else if n == Layer1Code::Layer1EventError as i64 {
                            Some(Layer1Code::Layer1EventError)
                        } else if n == Layer1Code::ParseH256Error as i64 {
                            Some(Layer1Code::ParseH256Error)
                        } else if n == Layer1Code::ParseTeaIdError as i64 {
                            Some(Layer1Code::ParseTeaIdError)
                        } else if n == Layer1Code::ParseBalanceError as i64 {
                            Some(Layer1Code::ParseBalanceError)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_Layer1Code: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for Layer1Code {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                Layer1Code::GeneralLayer1Error => {
                                    Layer1Code::GeneralLayer1Error as i64
                                }
                                Layer1Code::FailedToFindBlock => {
                                    Layer1Code::FailedToFindBlock as i64
                                }
                                Layer1Code::UnknownMinerClass => {
                                    Layer1Code::UnknownMinerClass as i64
                                }
                                Layer1Code::UnknownMinerStatus => {
                                    Layer1Code::UnknownMinerStatus as i64
                                }
                                Layer1Code::ParseAccountError => {
                                    Layer1Code::ParseAccountError as i64
                                }
                                Layer1Code::GetStorageError => {
                                    Layer1Code::GetStorageError as i64
                                }
                                Layer1Code::Layer1ClientRPCGeneral => {
                                    Layer1Code::Layer1ClientRPCGeneral as i64
                                }
                                Layer1Code::ParseAddressError => {
                                    Layer1Code::ParseAddressError as i64
                                }
                                Layer1Code::Layer1EventError => {
                                    Layer1Code::Layer1EventError as i64
                                }
                                Layer1Code::ParseH256Error => {
                                    Layer1Code::ParseH256Error as i64
                                }
                                Layer1Code::ParseTeaIdError => {
                                    Layer1Code::ParseTeaIdError as i64
                                }
                                Layer1Code::ParseBalanceError => {
                                    Layer1Code::ParseBalanceError as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            impl From<u16> for Layer1Code {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(Layer1Code::GeneralLayer1Error)
                }
            }
            pub struct Layer1Translator {}
            impl Translate<Layer1Code> for Layer1Translator {
                fn translate(&self, code: Layer1Code) -> &'static str {
                    match code {
                        Layer1Code::GeneralLayer1Error => "general layer1 error",
                        Layer1Code::FailedToFindBlock => {
                            "Failed to find layer1 block at specified height"
                        }
                        Layer1Code::ParseAccountError => "parse account error",
                        Layer1Code::ParseAddressError => "parse address error",
                        Layer1Code::ParseH256Error => "parse h256 error",
                        Layer1Code::ParseTeaIdError => "parse TEA ID error",
                        Layer1Code::ParseBalanceError => "parse balance error",
                        Layer1Code::UnknownMinerClass => {
                            "unknown miner class convert value"
                        }
                        Layer1Code::UnknownMinerStatus => {
                            "unknown miner status convert value"
                        }
                        Layer1Code::GetStorageError => "get storage failed",
                        Layer1Code::Layer1ClientRPCGeneral => {
                            "layer1 client RPC general error"
                        }
                        Layer1Code::Layer1EventError => "layer1 event error",
                    }
                }
            }
            pub fn new_layer1_error_code(code: Layer1Code) -> Layer1ErrorCode {
                ErrorCodeTranslator::new(code as u16, Layer1Translator {})
            }
        }
        pub mod service {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type ServiceErrorCode = ErrorCodeTranslator<
                ServiceTranslator,
                ServiceCode,
            >;
            pub enum ServiceCode {
                GeneralServiceError = 50000,
                GlueSQLError,
                StateGeneralError,
                HttpExecutionError,
                BondingGeneralError,
                DbNotFoundError,
                InvalidTransactionContext,
                InvalidValidator,
                InvalidTxnRequest,
                AsyncCanceled,
                AsyncNotFinished,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ServiceCode {}
            #[automatically_derived]
            impl ::core::clone::Clone for ServiceCode {
                #[inline]
                fn clone(&self) -> ServiceCode {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_ServiceCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ServiceCode {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == ServiceCode::GeneralServiceError as i64 {
                            Some(ServiceCode::GeneralServiceError)
                        } else if n == ServiceCode::GlueSQLError as i64 {
                            Some(ServiceCode::GlueSQLError)
                        } else if n == ServiceCode::StateGeneralError as i64 {
                            Some(ServiceCode::StateGeneralError)
                        } else if n == ServiceCode::HttpExecutionError as i64 {
                            Some(ServiceCode::HttpExecutionError)
                        } else if n == ServiceCode::BondingGeneralError as i64 {
                            Some(ServiceCode::BondingGeneralError)
                        } else if n == ServiceCode::DbNotFoundError as i64 {
                            Some(ServiceCode::DbNotFoundError)
                        } else if n == ServiceCode::InvalidTransactionContext as i64 {
                            Some(ServiceCode::InvalidTransactionContext)
                        } else if n == ServiceCode::InvalidValidator as i64 {
                            Some(ServiceCode::InvalidValidator)
                        } else if n == ServiceCode::InvalidTxnRequest as i64 {
                            Some(ServiceCode::InvalidTxnRequest)
                        } else if n == ServiceCode::AsyncCanceled as i64 {
                            Some(ServiceCode::AsyncCanceled)
                        } else if n == ServiceCode::AsyncNotFinished as i64 {
                            Some(ServiceCode::AsyncNotFinished)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_ServiceCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ServiceCode {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                ServiceCode::GeneralServiceError => {
                                    ServiceCode::GeneralServiceError as i64
                                }
                                ServiceCode::GlueSQLError => {
                                    ServiceCode::GlueSQLError as i64
                                }
                                ServiceCode::StateGeneralError => {
                                    ServiceCode::StateGeneralError as i64
                                }
                                ServiceCode::HttpExecutionError => {
                                    ServiceCode::HttpExecutionError as i64
                                }
                                ServiceCode::BondingGeneralError => {
                                    ServiceCode::BondingGeneralError as i64
                                }
                                ServiceCode::DbNotFoundError => {
                                    ServiceCode::DbNotFoundError as i64
                                }
                                ServiceCode::InvalidTransactionContext => {
                                    ServiceCode::InvalidTransactionContext as i64
                                }
                                ServiceCode::InvalidValidator => {
                                    ServiceCode::InvalidValidator as i64
                                }
                                ServiceCode::InvalidTxnRequest => {
                                    ServiceCode::InvalidTxnRequest as i64
                                }
                                ServiceCode::AsyncCanceled => {
                                    ServiceCode::AsyncCanceled as i64
                                }
                                ServiceCode::AsyncNotFinished => {
                                    ServiceCode::AsyncNotFinished as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            impl From<u16> for ServiceCode {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(ServiceCode::GeneralServiceError)
                }
            }
            pub struct ServiceTranslator {}
            impl Translate<ServiceCode> for ServiceTranslator {
                fn translate(&self, code: ServiceCode) -> &'static str {
                    match code {
                        ServiceCode::GeneralServiceError => "general service error",
                        ServiceCode::GlueSQLError => "glue sql error",
                        ServiceCode::DbNotFoundError => "gluedb not found",
                        ServiceCode::StateGeneralError => "state machine error",
                        ServiceCode::BondingGeneralError => "bonding curve error",
                        ServiceCode::InvalidTransactionContext => {
                            "invalid transaction context"
                        }
                        ServiceCode::InvalidValidator => "i'm not a validator",
                        ServiceCode::InvalidTxnRequest => {
                            "txn request send from other node is invalid"
                        }
                        ServiceCode::HttpExecutionError => "http execution error",
                        ServiceCode::AsyncCanceled => "async canceled",
                        ServiceCode::AsyncNotFinished => "async not finished",
                    }
                }
            }
            pub fn new_service_error_code(code: ServiceCode) -> ServiceErrorCode {
                ErrorCodeTranslator::new(code as u16, ServiceTranslator {})
            }
        }
        pub mod vmh {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type VmhErrorCode = ErrorCodeTranslator<VmhTranslator, VmhCode>;
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
                IntercomActorNotSupported,
                IntercomRequestRejected,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for VmhCode {}
            #[automatically_derived]
            impl ::core::clone::Clone for VmhCode {
                #[inline]
                fn clone(&self) -> VmhCode {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_VmhCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for VmhCode {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == VmhCode::GeneralVmhError as i64 {
                            Some(VmhCode::GeneralVmhError)
                        } else if n == VmhCode::SocketSendU64Error as i64 {
                            Some(VmhCode::SocketSendU64Error)
                        } else if n == VmhCode::SocketSendLoopError as i64 {
                            Some(VmhCode::SocketSendLoopError)
                        } else if n == VmhCode::SocketRecvU64Error as i64 {
                            Some(VmhCode::SocketRecvU64Error)
                        } else if n == VmhCode::SocketRecvLoopError as i64 {
                            Some(VmhCode::SocketRecvLoopError)
                        } else if n == VmhCode::SocketClientDisconnected as i64 {
                            Some(VmhCode::SocketClientDisconnected)
                        } else if n == VmhCode::SocketServerClosed as i64 {
                            Some(VmhCode::SocketServerClosed)
                        } else if n == VmhCode::SocketRecvSizeMismatched as i64 {
                            Some(VmhCode::SocketRecvSizeMismatched)
                        } else if n == VmhCode::SocketSendSizeMismatched as i64 {
                            Some(VmhCode::SocketSendSizeMismatched)
                        } else if n == VmhCode::SocketNixError as i64 {
                            Some(VmhCode::SocketNixError)
                        } else if n == VmhCode::QuitReceiverLoop as i64 {
                            Some(VmhCode::QuitReceiverLoop)
                        } else if n == VmhCode::OperationInvalid as i64 {
                            Some(VmhCode::OperationInvalid)
                        } else if n == VmhCode::SenderOperationExists as i64 {
                            Some(VmhCode::SenderOperationExists)
                        } else if n == VmhCode::InboundNetError as i64 {
                            Some(VmhCode::InboundNetError)
                        } else if n == VmhCode::OutboundNetError as i64 {
                            Some(VmhCode::OutboundNetError)
                        } else if n == VmhCode::IntercomActorNotSupported as i64 {
                            Some(VmhCode::IntercomActorNotSupported)
                        } else if n == VmhCode::IntercomRequestRejected as i64 {
                            Some(VmhCode::IntercomRequestRejected)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_VmhCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for VmhCode {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                VmhCode::GeneralVmhError => VmhCode::GeneralVmhError as i64,
                                VmhCode::SocketSendU64Error => {
                                    VmhCode::SocketSendU64Error as i64
                                }
                                VmhCode::SocketSendLoopError => {
                                    VmhCode::SocketSendLoopError as i64
                                }
                                VmhCode::SocketRecvU64Error => {
                                    VmhCode::SocketRecvU64Error as i64
                                }
                                VmhCode::SocketRecvLoopError => {
                                    VmhCode::SocketRecvLoopError as i64
                                }
                                VmhCode::SocketClientDisconnected => {
                                    VmhCode::SocketClientDisconnected as i64
                                }
                                VmhCode::SocketServerClosed => {
                                    VmhCode::SocketServerClosed as i64
                                }
                                VmhCode::SocketRecvSizeMismatched => {
                                    VmhCode::SocketRecvSizeMismatched as i64
                                }
                                VmhCode::SocketSendSizeMismatched => {
                                    VmhCode::SocketSendSizeMismatched as i64
                                }
                                VmhCode::SocketNixError => VmhCode::SocketNixError as i64,
                                VmhCode::QuitReceiverLoop => {
                                    VmhCode::QuitReceiverLoop as i64
                                }
                                VmhCode::OperationInvalid => {
                                    VmhCode::OperationInvalid as i64
                                }
                                VmhCode::SenderOperationExists => {
                                    VmhCode::SenderOperationExists as i64
                                }
                                VmhCode::InboundNetError => VmhCode::InboundNetError as i64,
                                VmhCode::OutboundNetError => {
                                    VmhCode::OutboundNetError as i64
                                }
                                VmhCode::IntercomActorNotSupported => {
                                    VmhCode::IntercomActorNotSupported as i64
                                }
                                VmhCode::IntercomRequestRejected => {
                                    VmhCode::IntercomRequestRejected as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            impl From<u16> for VmhCode {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(VmhCode::GeneralVmhError)
                }
            }
            pub struct VmhTranslator {}
            impl Translate<VmhCode> for VmhTranslator {
                fn translate(&self, code: VmhCode) -> &'static str {
                    match code {
                        VmhCode::GeneralVmhError => "general vmh error",
                        VmhCode::SocketSendU64Error => "socket send u64 error",
                        VmhCode::SocketRecvU64Error => "socket receive u64 error",
                        VmhCode::SocketSendLoopError => "socket send loop error",
                        VmhCode::SocketRecvLoopError => "socket receive loop error",
                        VmhCode::SocketClientDisconnected => "client has disconnected",
                        VmhCode::SocketServerClosed => "server has closed",
                        VmhCode::SocketRecvSizeMismatched => "received bytes not matched",
                        VmhCode::SocketSendSizeMismatched => "send bytes not matched",
                        VmhCode::SocketNixError => "socket nix error",
                        VmhCode::QuitReceiverLoop => "receiver loop has terminated",
                        VmhCode::OperationInvalid => "operation is invalid",
                        VmhCode::SenderOperationExists => {
                            "sender operation already exists"
                        }
                        VmhCode::InboundNetError => "inbound net error",
                        VmhCode::OutboundNetError => "outbound net error",
                        VmhCode::IntercomActorNotSupported => {
                            "intercom actor not supported"
                        }
                        VmhCode::IntercomRequestRejected => "intercom request rejected",
                    }
                }
            }
            pub fn new_vmh_error_code(code: VmhCode) -> VmhErrorCode {
                ErrorCodeTranslator::new(code as u16, VmhTranslator {})
            }
        }
        pub mod wascc {
            use crate::error::translator::{ErrorCodeTranslator, Translate};
            pub type WasccErrorCode = ErrorCodeTranslator<WasccTranslator, WasccCode>;
            pub enum WasccCode {
                GeneralWasccError = 10000,
                BadDispatch,
                GeneralHostError,
                NoSuchFunction,
                WasmMisc,
                HostCallFailure,
                GuestCallFailure,
                WapcGeneralError,
                WascapGeneralError,
                HostAuthorizationError,
                CapabilityProviderError,
                MiscHostError,
                PluginError,
                MiddlewareError,
                ActorToActorCallNotExist,
                InvocationError,
                KeyValueError,
                MessagingError,
                EnvVarError,
                DiscardMessageError,
                ProviderOperationRejected,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for WasccCode {}
            #[automatically_derived]
            impl ::core::clone::Clone for WasccCode {
                #[inline]
                fn clone(&self) -> WasccCode {
                    *self
                }
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_FromPrimitive_FOR_WasccCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for WasccCode {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> Option<Self> {
                        if n == WasccCode::GeneralWasccError as i64 {
                            Some(WasccCode::GeneralWasccError)
                        } else if n == WasccCode::BadDispatch as i64 {
                            Some(WasccCode::BadDispatch)
                        } else if n == WasccCode::GeneralHostError as i64 {
                            Some(WasccCode::GeneralHostError)
                        } else if n == WasccCode::NoSuchFunction as i64 {
                            Some(WasccCode::NoSuchFunction)
                        } else if n == WasccCode::WasmMisc as i64 {
                            Some(WasccCode::WasmMisc)
                        } else if n == WasccCode::HostCallFailure as i64 {
                            Some(WasccCode::HostCallFailure)
                        } else if n == WasccCode::GuestCallFailure as i64 {
                            Some(WasccCode::GuestCallFailure)
                        } else if n == WasccCode::WapcGeneralError as i64 {
                            Some(WasccCode::WapcGeneralError)
                        } else if n == WasccCode::WascapGeneralError as i64 {
                            Some(WasccCode::WascapGeneralError)
                        } else if n == WasccCode::HostAuthorizationError as i64 {
                            Some(WasccCode::HostAuthorizationError)
                        } else if n == WasccCode::CapabilityProviderError as i64 {
                            Some(WasccCode::CapabilityProviderError)
                        } else if n == WasccCode::MiscHostError as i64 {
                            Some(WasccCode::MiscHostError)
                        } else if n == WasccCode::PluginError as i64 {
                            Some(WasccCode::PluginError)
                        } else if n == WasccCode::MiddlewareError as i64 {
                            Some(WasccCode::MiddlewareError)
                        } else if n == WasccCode::ActorToActorCallNotExist as i64 {
                            Some(WasccCode::ActorToActorCallNotExist)
                        } else if n == WasccCode::InvocationError as i64 {
                            Some(WasccCode::InvocationError)
                        } else if n == WasccCode::KeyValueError as i64 {
                            Some(WasccCode::KeyValueError)
                        } else if n == WasccCode::MessagingError as i64 {
                            Some(WasccCode::MessagingError)
                        } else if n == WasccCode::EnvVarError as i64 {
                            Some(WasccCode::EnvVarError)
                        } else if n == WasccCode::DiscardMessageError as i64 {
                            Some(WasccCode::DiscardMessageError)
                        } else if n == WasccCode::ProviderOperationRejected as i64 {
                            Some(WasccCode::ProviderOperationRejected)
                        } else {
                            None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _IMPL_NUM_ToPrimitive_FOR_WasccCode: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for WasccCode {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> Option<i64> {
                        Some(
                            match *self {
                                WasccCode::GeneralWasccError => {
                                    WasccCode::GeneralWasccError as i64
                                }
                                WasccCode::BadDispatch => WasccCode::BadDispatch as i64,
                                WasccCode::GeneralHostError => {
                                    WasccCode::GeneralHostError as i64
                                }
                                WasccCode::NoSuchFunction => {
                                    WasccCode::NoSuchFunction as i64
                                }
                                WasccCode::WasmMisc => WasccCode::WasmMisc as i64,
                                WasccCode::HostCallFailure => {
                                    WasccCode::HostCallFailure as i64
                                }
                                WasccCode::GuestCallFailure => {
                                    WasccCode::GuestCallFailure as i64
                                }
                                WasccCode::WapcGeneralError => {
                                    WasccCode::WapcGeneralError as i64
                                }
                                WasccCode::WascapGeneralError => {
                                    WasccCode::WascapGeneralError as i64
                                }
                                WasccCode::HostAuthorizationError => {
                                    WasccCode::HostAuthorizationError as i64
                                }
                                WasccCode::CapabilityProviderError => {
                                    WasccCode::CapabilityProviderError as i64
                                }
                                WasccCode::MiscHostError => WasccCode::MiscHostError as i64,
                                WasccCode::PluginError => WasccCode::PluginError as i64,
                                WasccCode::MiddlewareError => {
                                    WasccCode::MiddlewareError as i64
                                }
                                WasccCode::ActorToActorCallNotExist => {
                                    WasccCode::ActorToActorCallNotExist as i64
                                }
                                WasccCode::InvocationError => {
                                    WasccCode::InvocationError as i64
                                }
                                WasccCode::KeyValueError => WasccCode::KeyValueError as i64,
                                WasccCode::MessagingError => {
                                    WasccCode::MessagingError as i64
                                }
                                WasccCode::EnvVarError => WasccCode::EnvVarError as i64,
                                WasccCode::DiscardMessageError => {
                                    WasccCode::DiscardMessageError as i64
                                }
                                WasccCode::ProviderOperationRejected => {
                                    WasccCode::ProviderOperationRejected as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            impl From<u16> for WasccCode {
                fn from(v: u16) -> Self {
                    num_traits::FromPrimitive::from_u16(v)
                        .unwrap_or(WasccCode::GeneralWasccError)
                }
            }
            pub struct WasccTranslator {}
            impl Translate<WasccCode> for WasccTranslator {
                fn translate(&self, code: WasccCode) -> &'static str {
                    match code {
                        WasccCode::GeneralWasccError => "general wascc error",
                        WasccCode::GeneralHostError => "general host error",
                        WasccCode::NoSuchFunction => "No such function in Wasm module",
                        WasccCode::WasmMisc => "WebAssembly failure",
                        WasccCode::HostCallFailure => "Error occurred during host call",
                        WasccCode::GuestCallFailure => "Guest call failure",
                        WasccCode::WapcGeneralError => "waPC error",
                        WasccCode::WascapGeneralError => "Embedded JWT Failure",
                        WasccCode::HostAuthorizationError => {
                            "Module authorization failure"
                        }
                        WasccCode::CapabilityProviderError => {
                            "Capability provider failure"
                        }
                        WasccCode::MiscHostError => "waSCC Host error",
                        WasccCode::PluginError => "Plugin error",
                        WasccCode::MiddlewareError => "Middleware error",
                        WasccCode::ActorToActorCallNotExist => {
                            "Attempted actor-to-actor call to non-existent target"
                        }
                        WasccCode::InvocationError => "Invocation failure",
                        WasccCode::BadDispatch => "bad dispatch",
                        WasccCode::KeyValueError => "Key/value store error",
                        WasccCode::MessagingError => "Messaging error",
                        WasccCode::EnvVarError => "Environment variable error",
                        WasccCode::DiscardMessageError => "discard message",
                        WasccCode::ProviderOperationRejected => {
                            "provider operation rejected"
                        }
                    }
                }
            }
            pub fn new_wascc_error_code(code: WasccCode) -> WasccErrorCode {
                ErrorCodeTranslator::new(code as u16, WasccTranslator {})
            }
        }
        pub struct ErrorCode {
            code: u16,
            description: String,
            details: Option<String>,
            inner: Box<Option<ErrorCode>>,
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for ErrorCode {}
        #[automatically_derived]
        impl ::core::clone::Clone for ErrorCode {
            #[inline]
            fn clone(&self) -> ErrorCode {
                ErrorCode {
                    code: ::core::clone::Clone::clone(&self.code),
                    description: ::core::clone::Clone::clone(&self.description),
                    details: ::core::clone::Clone::clone(&self.details),
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ErrorCode {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "code" => _serde::__private::Ok(__Field::__field0),
                                "description" => _serde::__private::Ok(__Field::__field1),
                                "details" => _serde::__private::Ok(__Field::__field2),
                                "inner" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"code" => _serde::__private::Ok(__Field::__field0),
                                b"description" => _serde::__private::Ok(__Field::__field1),
                                b"details" => _serde::__private::Ok(__Field::__field2),
                                b"inner" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ErrorCode>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ErrorCode;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ErrorCode",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                u16,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ErrorCode with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ErrorCode with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ErrorCode with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Box<Option<ErrorCode>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ErrorCode with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ErrorCode {
                                code: __field0,
                                description: __field1,
                                details: __field2,
                                inner: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Box<Option<ErrorCode>>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("code"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u16>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "details",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("inner"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Box<Option<ErrorCode>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("code") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("description") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("details") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("inner") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ErrorCode {
                                code: __field0,
                                description: __field1,
                                details: __field2,
                                inner: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "code",
                        "description",
                        "details",
                        "inner",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ErrorCode",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ErrorCode>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ErrorCode {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ErrorCode",
                        false as usize + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "code",
                        &self.code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "details",
                        &self.details,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "inner",
                        &self.inner,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl PartialEq for ErrorCode {
            fn eq(&self, other: &Self) -> bool {
                self.code_chain().eq(&other.code_chain())
            }
        }
        impl ErrorCode {
            pub fn new(code: u16, description: String, details: Option<String>) -> Self {
                ErrorCode {
                    code,
                    description,
                    details,
                    inner: Default::default(),
                }
            }
            pub fn new_slim(code: u16, description: String) -> Self {
                ErrorCode {
                    code,
                    description,
                    details: Default::default(),
                    inner: Default::default(),
                }
            }
            pub fn new_nested(
                code: u16,
                description: String,
                details: Option<String>,
                inner: Option<ErrorCode>,
            ) -> Self {
                ErrorCode {
                    code,
                    description,
                    details,
                    inner: Box::new(inner),
                }
            }
            pub fn code(&self) -> u16 {
                self.code
            }
            pub fn inner(&self) -> &Option<ErrorCode> {
                self.inner.as_ref()
            }
            pub fn details(&self) -> Option<&String> {
                self.details.as_ref()
            }
            pub fn contains(&self, code: u16) -> bool {
                if self.code == code {
                    true
                } else {
                    if let Some(inner) = self.inner.as_ref() {
                        return inner.contains(code);
                    }
                    false
                }
            }
            fn error_chain_message(&self) -> String {
                let mut rtn = self
                    .error_message(&self.description, self.details.as_ref());
                if let Some(e) = self.inner.as_ref() {
                    rtn = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "\r\t"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&rtn),
                                    ::core::fmt::ArgumentV1::new_display(
                                        &e.error_chain_message(),
                                    ),
                                ],
                            ),
                        );
                        res
                    };
                }
                rtn
            }
            fn error_message(&self, desc: &str, details: Option<&String>) -> String {
                if let Some(details) = details {
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", ": ", ", details: "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&self.print_code()),
                                    ::core::fmt::ArgumentV1::new_display(&desc),
                                    ::core::fmt::ArgumentV1::new_display(&details),
                                ],
                            ),
                        );
                        res
                    }
                } else {
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&self.print_code()),
                                    ::core::fmt::ArgumentV1::new_display(&desc),
                                ],
                            ),
                        );
                        res
                    }
                }
            }
            fn print_code(&self) -> String {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["TEC-"],
                            &[::core::fmt::ArgumentV1::new_display(&self.code)],
                        ),
                    );
                    res
                }
            }
            fn code_chain(&self) -> Vec<u16> {
                let mut chain = Vec::new();
                let mut cursor = Some(self);
                while let Some(e) = cursor {
                    chain.push(e.code);
                    cursor = (*e.inner).as_ref();
                }
                chain
            }
        }
        impl Debug for ErrorCode {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &self.error_chain_message(),
                            ),
                        ],
                    ),
                )
            }
        }
        impl Display for ErrorCode {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &self.error_chain_message(),
                            ),
                        ],
                    ),
                )
            }
        }
    }
    pub mod translator {
        use crate::error::code::ErrorCode;
        use crate::error::TeaError;
        use std::marker::PhantomData;
        pub trait Translate<C>
        where
            C: From<u16>,
        {
            fn translate(&self, code: C) -> &'static str;
        }
        pub struct ErrorCodeTranslator<T, C>
        where
            T: Translate<C>,
            C: From<u16>,
        {
            pub code: u16,
            translator: T,
            phantom_data: PhantomData<C>,
        }
        impl<T, C> ErrorCodeTranslator<T, C>
        where
            T: Translate<C>,
            C: From<u16>,
        {
            pub fn new(code: u16, translator: T) -> Self {
                ErrorCodeTranslator {
                    code,
                    translator,
                    phantom_data: PhantomData::default(),
                }
            }
            pub fn to_error_code(
                &self,
                details: Option<String>,
                inner: Option<ErrorCode>,
            ) -> TeaError {
                TeaError::EncodedError(
                    ErrorCode::new_nested(self.code, self.to_string(), details, inner),
                )
            }
            pub fn error_from_nested(&self, e: TeaError) -> TeaError {
                match e.parse_error_code() {
                    Some(code) => self.to_error_code(None, Some(code)),
                    None => {
                        self
                            .to_error_code(
                                Some({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[::core::fmt::ArgumentV1::new_debug(&e)],
                                        ),
                                    );
                                    res
                                }),
                                None,
                            )
                    }
                }
            }
            pub fn error_from_nested_and_details(
                &self,
                e: TeaError,
                details: String,
            ) -> TeaError {
                match e.parse_error_code() {
                    Some(code) => self.to_error_code(Some(details), Some(code)),
                    None => {
                        self
                            .to_error_code(
                                Some({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["error is ", ", details: "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_debug(&e),
                                                ::core::fmt::ArgumentV1::new_display(&details),
                                            ],
                                        ),
                                    );
                                    res
                                }),
                                None,
                            )
                    }
                }
            }
        }
        impl<T, C> ToString for ErrorCodeTranslator<T, C>
        where
            T: Translate<C>,
            C: From<u16>,
        {
            fn to_string(&self) -> String {
                self.translator.translate(C::from(self.code)).to_string()
            }
        }
        impl<T, C> From<ErrorCodeTranslator<T, C>> for ErrorCode
        where
            T: Translate<C>,
            C: From<u16>,
        {
            fn from(val: ErrorCodeTranslator<T, C>) -> Self {
                ErrorCode::new_slim(val.code, val.to_string())
            }
        }
    }
    pub use code::{
        client::{new_client_error_code, ClientCode},
        common::{new_common_error_code, CommonCode},
        layer1::{new_layer1_error_code, Layer1Code},
        service::{new_service_error_code, ServiceCode},
        vmh::{new_vmh_error_code, VmhCode},
        wascc::{new_wascc_error_code, WasccCode},
        ErrorCode,
    };
    pub enum TeaError {
        #[error("Tea common error, details: `{0}`")]
        CommonError(String),
        #[error(transparent)]
        EncodedError(#[from] ErrorCode),
        #[error(transparent)]
        NewError(crate::errorx::Error<()>),
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for TeaError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                TeaError::CommonError { .. } => std::option::Option::None,
                TeaError::EncodedError { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                TeaError::NewError { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for TeaError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                TeaError::CommonError(_0) => {
                    __formatter
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Tea common error, details: `", "`"],
                                &[::core::fmt::ArgumentV1::new_display(&_0.as_display())],
                            ),
                        )
                }
                TeaError::EncodedError(_0) => std::fmt::Display::fmt(_0, __formatter),
                TeaError::NewError(_0) => std::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<ErrorCode> for TeaError {
        #[allow(deprecated)]
        fn from(source: ErrorCode) -> Self {
            TeaError::EncodedError {
                0: source,
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TeaError {
        #[inline]
        fn clone(&self) -> TeaError {
            match self {
                TeaError::CommonError(__self_0) => {
                    TeaError::CommonError(::core::clone::Clone::clone(__self_0))
                }
                TeaError::EncodedError(__self_0) => {
                    TeaError::EncodedError(::core::clone::Clone::clone(__self_0))
                }
                TeaError::NewError(__self_0) => {
                    TeaError::NewError(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TeaError {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "CommonError" => _serde::__private::Ok(__Field::__field0),
                            "EncodedError" => _serde::__private::Ok(__Field::__field1),
                            "NewError" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"CommonError" => _serde::__private::Ok(__Field::__field0),
                            b"EncodedError" => _serde::__private::Ok(__Field::__field1),
                            b"NewError" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TeaError>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TeaError;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum TeaError",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        String,
                                    >(__variant),
                                    TeaError::CommonError,
                                )
                            }
                            (__Field::__field1, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        ErrorCode,
                                    >(__variant),
                                    TeaError::EncodedError,
                                )
                            }
                            (__Field::__field2, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        crate::errorx::Error<()>,
                                    >(__variant),
                                    TeaError::NewError,
                                )
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "CommonError",
                    "EncodedError",
                    "NewError",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TeaError",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TeaError>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TeaError {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    TeaError::CommonError(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TeaError",
                            0u32,
                            "CommonError",
                            __field0,
                        )
                    }
                    TeaError::EncodedError(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TeaError",
                            1u32,
                            "EncodedError",
                            __field0,
                        )
                    }
                    TeaError::NewError(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TeaError",
                            2u32,
                            "NewError",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    pub type TeaResult<T> = std::result::Result<T, TeaError>;
    pub type Result<T, E = TeaError> = std::result::Result<T, E>;
    pub fn option_none_error<S: AsRef<str>>(msg: S) -> TeaError {
        code::common::new_common_error_code(CommonCode::OptionIsNone)
            .to_error_code(Some(msg.as_ref().to_string()), None)
    }
    pub fn discard_message_error<S: AsRef<str>>(msg: S) -> TeaError {
        code::wascc::new_wascc_error_code(WasccCode::DiscardMessageError)
            .to_error_code(Some(msg.as_ref().to_string()), None)
    }
    pub fn tea_err<T, E: Into<TeaError>>(e: Result<T, E>) -> TeaResult<T> {
        e.map_err::<TeaError, _>(|e| e.into())
    }
    impl TeaError {
        pub fn parse_error_code(&self) -> Option<ErrorCode> {
            match self {
                TeaError::EncodedError(e) => Some(e.clone()),
                TeaError::CommonError(s) => {
                    {
                        let lvl = ::log::Level::Warn;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["common error emit: "],
                                    &[::core::fmt::ArgumentV1::new_display(&s)],
                                ),
                                lvl,
                                &(
                                    "tea_codec::error",
                                    "tea_codec::error",
                                    "src/error.rs",
                                    59u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    Some(new_common_error_code(CommonCode::CommonError).into())
                }
                TeaError::NewError(x) => {
                    Some(
                        ErrorCode::new_nested(
                            CommonCode::NewError as _,
                            x.summary().map(Into::into).unwrap_or_default(),
                            x.detail().map(Into::into),
                            x
                                .inner()
                                .iter()
                                .filter_map(|x| {
                                    let error: TeaError = (*x).clone().into();
                                    error.parse_error_code()
                                })
                                .collect::<Vec<_>>()
                                .pop(),
                        ),
                    )
                }
            }
        }
    }
    impl From<prost::EncodeError> for TeaError {
        fn from(e: prost::EncodeError) -> Self {
            new_common_error_code(CommonCode::ProstEncodeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<prost::DecodeError> for TeaError {
        fn from(e: prost::DecodeError) -> Self {
            new_common_error_code(CommonCode::ProstDecodeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<String> for TeaError {
        fn from(s: String) -> Self {
            TeaError::CommonError(s)
        }
    }
    impl From<&str> for TeaError {
        fn from(s: &str) -> Self {
            TeaError::CommonError(s.to_string())
        }
    }
    impl From<TryFromIntError> for TeaError {
        fn from(e: TryFromIntError) -> Self {
            new_common_error_code(CommonCode::TryFromConvertError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<ParseIntError> for TeaError {
        fn from(e: ParseIntError) -> Self {
            new_common_error_code(CommonCode::ParseStringError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<SystemTimeError> for TeaError {
        fn from(e: SystemTimeError) -> Self {
            new_common_error_code(CommonCode::SystemTimeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<FromUtf8Error> for TeaError {
        fn from(e: FromUtf8Error) -> Self {
            new_common_error_code(CommonCode::UTF8EncodingError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<Utf8Error> for TeaError {
        fn from(e: Utf8Error) -> Self {
            new_common_error_code(CommonCode::UTF8EncodingError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<serde_json::Error> for TeaError {
        fn from(e: Error) -> Self {
            new_common_error_code(CommonCode::SerdeGeneralError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<std::io::Error> for TeaError {
        fn from(e: std::io::Error) -> Self {
            new_common_error_code(CommonCode::StdIoError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<log::SetLoggerError> for TeaError {
        fn from(e: log::SetLoggerError) -> Self {
            new_common_error_code(CommonCode::LogGeneralError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<ParseLevelError> for TeaError {
        fn from(e: ParseLevelError) -> Self {
            new_common_error_code(CommonCode::LogGeneralError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<bincode::Error> for TeaError {
        fn from(e: bincode::Error) -> Self {
            new_common_error_code(CommonCode::BincodeSerializeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<TryFromSliceError> for TeaError {
        fn from(e: TryFromSliceError) -> Self {
            new_common_error_code(CommonCode::TryFromConvertError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<base64::DecodeError> for TeaError {
        fn from(e: base64::DecodeError) -> Self {
            new_common_error_code(CommonCode::Base64DecodeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl From<hex::FromHexError> for TeaError {
        fn from(e: hex::FromHexError) -> Self {
            new_common_error_code(CommonCode::HexDecodeError)
                .to_error_code(
                    Some({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }),
                    None,
                )
        }
    }
    impl<S> From<crate::errorx::Error<S>> for TeaError {
        fn from(x: crate::errorx::Error<S>) -> Self {
            match x.back_cast() {
                Ok(e) => e,
                Err(x) => TeaError::NewError(x.into_scope()),
            }
        }
    }
    impl Debug for TeaError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TeaError::EncodedError(code) => {
                    f
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&code)],
                            ),
                        )
                }
                TeaError::CommonError(msg) => {
                    f
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Unknown error: "],
                                &[::core::fmt::ArgumentV1::new_display(&msg)],
                            ),
                        )
                }
                TeaError::NewError(err) => Debug::fmt(err, f),
            }
        }
    }
}
pub mod errorx {
    mod aggregate {
        use std::ops::Add;
        use smallvec::{smallvec, SmallVec};
        use super::{serde::SerializedData, Error, Global};
        pub(crate) struct Aggregate(pub(crate) SmallVec<[Error; 2]>);
        #[automatically_derived]
        impl ::core::fmt::Debug for Aggregate {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Aggregate",
                    &&self.0,
                )
            }
        }
        impl<LS, RS> Add<Error<RS>> for Error<LS> {
            type Output = Self;
            fn add(self, rhs: Error<RS>) -> Self::Output {
                let lhs = match self.back_cast::<Aggregate>() {
                    Ok(mut agg) => {
                        agg.0.push(rhs.into());
                        return Error::<Global>::from(agg).into();
                    }
                    Err(lhs) => lhs,
                };
                let lhs = if lhs.name() == Some("Aggregate".into()) {
                    match lhs.back_cast::<SerializedData>() {
                        Ok(mut agg) => {
                            match &mut agg.inner {
                                Some(inner) => inner.push(rhs.into()),
                                None => {
                                    agg
                                        .inner = Some({
                                        let count = 0usize + 1usize;
                                        #[allow(unused_mut)]
                                        let mut vec = ::smallvec::SmallVec::new();
                                        if count <= vec.inline_size() {
                                            vec.push(rhs.into());
                                            vec
                                        } else {
                                            ::smallvec::SmallVec::from_vec(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([rhs.into()]),
                                                ),
                                            )
                                        }
                                    });
                                }
                            }
                            return Error::<Global>::from(agg).into();
                        }
                        Err(lhs) => lhs,
                    }
                } else {
                    lhs
                };
                Error::<
                    Global,
                >::from(
                        Aggregate({
                            let count = 0usize + 1usize + 1usize;
                            #[allow(unused_mut)]
                            let mut vec = ::smallvec::SmallVec::new();
                            if count <= vec.inline_size() {
                                vec.push(lhs.into());
                                vec.push(rhs.into());
                                vec
                            } else {
                                ::smallvec::SmallVec::from_vec(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([lhs.into(), rhs.into()]),
                                    ),
                                )
                            }
                        }),
                    )
                    .into()
            }
        }
    }
    mod global {
        use std::{
            array::TryFromSliceError, borrow::Cow,
            char::{ParseCharError, TryFromCharError},
            num::{ParseFloatError, ParseIntError, TryFromIntError},
            rc::Rc, str::{ParseBoolError, Utf8Error},
            string::FromUtf8Error, sync::Arc, time::SystemTimeError,
        };
        use hex::FromHexError;
        use log::{ParseLevelError, SetLoggerError};
        use smallvec::SmallVec;
        use thiserror::Error;
        use super::{aggregate::Aggregate, Descriptor};
        pub enum Global {
            Aggregate,
            CannotBeNone,
            Unknown,
            JsonSerde,
            BincodeSerde,
            Utf8,
            StdIo,
            ProstEncode,
            ProstDecode,
            TryFrom,
            Parse,
            SystemTime,
            Log,
            Base64Decode,
            HexDecode,
            ChannelReceive,
            ChannelSend,
            TeaError,
        }
        impl ::core::marker::StructuralPartialEq for Global {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Global {
            #[inline]
            fn eq(&self, other: &Global) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        impl ::core::marker::StructuralEq for Global {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Global {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Global {
            #[inline]
            fn clone(&self) -> Global {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Global {}
        impl Global {
            pub const fn name_const(&self) -> &'static str {
                const AGGREGATE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Aggregate".as_bytes();
                    if let b"Global" = N1 {
                        "Aggregate"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const CANNOT_BE_NONE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "CannotBeNone".as_bytes();
                    if let b"Global" = N1 {
                        "CannotBeNone"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const UNKNOWN: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Unknown".as_bytes();
                    if let b"Global" = N1 {
                        "Unknown"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const JSON_SERDE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "JsonSerde".as_bytes();
                    if let b"Global" = N1 {
                        "JsonSerde"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const BINCODE_SERDE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "BincodeSerde".as_bytes();
                    if let b"Global" = N1 {
                        "BincodeSerde"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const UTF_8: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Utf8".as_bytes();
                    if let b"Global" = N1 {
                        "Utf8"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const STD_IO: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "StdIo".as_bytes();
                    if let b"Global" = N1 {
                        "StdIo"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const PROST_ENCODE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "ProstEncode".as_bytes();
                    if let b"Global" = N1 {
                        "ProstEncode"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const PROST_DECODE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "ProstDecode".as_bytes();
                    if let b"Global" = N1 {
                        "ProstDecode"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const TRY_FROM: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "TryFrom".as_bytes();
                    if let b"Global" = N1 {
                        "TryFrom"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const PARSE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Parse".as_bytes();
                    if let b"Global" = N1 {
                        "Parse"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const SYSTEM_TIME: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "SystemTime".as_bytes();
                    if let b"Global" = N1 {
                        "SystemTime"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const LOG: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Log".as_bytes();
                    if let b"Global" = N1 {
                        "Log"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const BASE_64_DECODE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "Base64Decode".as_bytes();
                    if let b"Global" = N1 {
                        "Base64Decode"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const HEX_DECODE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "HexDecode".as_bytes();
                    if let b"Global" = N1 {
                        "HexDecode"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const CHANNEL_RECEIVE: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "ChannelReceive".as_bytes();
                    if let b"Global" = N1 {
                        "ChannelReceive"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const CHANNEL_SEND: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "ChannelSend".as_bytes();
                    if let b"Global" = N1 {
                        "ChannelSend"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                const TEA_ERROR: &'static str = {
                    const N1: &[u8] = <Global as ::tea_codec::errorx::Scope>::FULLNAME
                        .as_bytes();
                    const N2: &[u8] = "TeaError".as_bytes();
                    if let b"Global" = N1 {
                        "TeaError"
                    } else {
                        const LEN: usize = N1.len() + N2.len() + 1;
                        const fn combine() -> [u8; LEN] {
                            let mut result = [0u8; LEN];
                            let mut i = 0;
                            while i < N1.len() {
                                result[i] = N1[i];
                                i += 1;
                            }
                            result[i] = b'.';
                            i = 0;
                            while i < N2.len() {
                                result[N1.len() + 1 + i] = N2[i];
                                i += 1;
                            }
                            result
                        }
                        unsafe { ::std::str::from_utf8_unchecked(&combine()) }
                    }
                };
                match self {
                    Global::Aggregate => AGGREGATE,
                    Global::CannotBeNone => CANNOT_BE_NONE,
                    Global::Unknown => UNKNOWN,
                    Global::JsonSerde => JSON_SERDE,
                    Global::BincodeSerde => BINCODE_SERDE,
                    Global::Utf8 => UTF_8,
                    Global::StdIo => STD_IO,
                    Global::ProstEncode => PROST_ENCODE,
                    Global::ProstDecode => PROST_DECODE,
                    Global::TryFrom => TRY_FROM,
                    Global::Parse => PARSE,
                    Global::SystemTime => SYSTEM_TIME,
                    Global::Log => LOG,
                    Global::Base64Decode => BASE_64_DECODE,
                    Global::HexDecode => HEX_DECODE,
                    Global::ChannelReceive => CHANNEL_RECEIVE,
                    Global::ChannelSend => CHANNEL_SEND,
                    Global::TeaError => TEA_ERROR,
                    _ => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(&["Bad scope value"], &[]),
                        )
                    }
                }
            }
        }
        impl ::std::ops::Deref for Global {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                self.name_const()
            }
        }
        impl ::std::convert::AsRef<str> for Global {
            fn as_ref(&self) -> &str {
                self
            }
        }
        impl ::std::borrow::Borrow<str> for Global {
            fn borrow(&self) -> &str {
                self
            }
        }
        impl ::std::fmt::Display for Global {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&**self, f)
            }
        }
        impl ::std::fmt::Debug for Global {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&**self, f)
            }
        }
        impl ::std::cmp::PartialEq<&str> for Global {
            fn eq(&self, other: &&str) -> bool {
                self.as_ref() == *other
            }
        }
        impl ::std::cmp::PartialEq<str> for Global {
            fn eq(&self, other: &str) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<str> for &Global {
            fn eq(&self, other: &str) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<&::std::string::String> for Global {
            fn eq(&self, other: &&::std::string::String) -> bool {
                self.as_ref() == other.as_str()
            }
        }
        impl ::std::cmp::PartialEq<::std::string::String> for Global {
            fn eq(&self, other: &::std::string::String) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<::std::string::String> for &Global {
            fn eq(&self, other: &::std::string::String) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<&::std::borrow::Cow<'_, str>> for Global {
            fn eq(&self, other: &&::std::borrow::Cow<'_, str>) -> bool {
                self.as_ref() == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<::std::borrow::Cow<'_, str>> for Global {
            fn eq(&self, other: &::std::borrow::Cow<'_, str>) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<::std::borrow::Cow<'_, str>> for &Global {
            fn eq(&self, other: &::std::borrow::Cow<'_, str>) -> bool {
                self.as_ref() == other
            }
        }
        impl ::std::cmp::PartialEq<Global> for &str {
            fn eq(&self, other: &Global) -> bool {
                *self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<Global> for str {
            fn eq(&self, other: &Global) -> bool {
                self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<&Global> for str {
            fn eq(&self, other: &&Global) -> bool {
                self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<Global> for &::std::string::String {
            fn eq(&self, other: &Global) -> bool {
                self.as_str() == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<Global> for ::std::string::String {
            fn eq(&self, other: &Global) -> bool {
                self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<&Global> for ::std::string::String {
            fn eq(&self, other: &&Global) -> bool {
                self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<Global> for &::std::borrow::Cow<'_, str> {
            fn eq(&self, other: &Global) -> bool {
                self.as_ref() == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<Global> for ::std::borrow::Cow<'_, str> {
            fn eq(&self, other: &Global) -> bool {
                self == other.as_ref()
            }
        }
        impl ::std::cmp::PartialEq<&Global> for ::std::borrow::Cow<'_, str> {
            fn eq(&self, other: &&Global) -> bool {
                self == other.as_ref()
            }
        }
        impl std::convert::From<Global> for ::std::borrow::Cow<'static, str> {
            fn from(s: Global) -> Self {
                s.name_const().into()
            }
        }
        impl std::convert::From<Global> for &'static str {
            fn from(s: Global) -> Self {
                s.name_const()
            }
        }
        impl std::convert::From<Global> for String {
            fn from(s: Global) -> Self {
                ::std::string::ToString::to_string(s.name_const())
            }
        }
        impl ::tea_codec::errorx::Scope for Global {
            type Parent = ::tea_codec::errorx::Global;
            type Descriptor<T> = Self;
            const NAME: &'static str = "Global";
            const FULLNAME: &'static str = "Global";
        }
        impl<T> ::tea_codec::errorx::Descriptor<T> for Global {
            default fn name(v: &T) -> Option<::std::borrow::Cow<str>> {
                None
            }
            default fn summary(v: &T) -> Option<::std::borrow::Cow<str>> {
                None
            }
            default fn detail(v: &T) -> Option<::std::borrow::Cow<str>> {
                None
            }
            default fn inner(
                v: &T,
            ) -> Option<
                ::tea_codec::errorx::SmallVec<[&::tea_codec::errorx::Error; 1]>,
            > {
                None
            }
            default fn type_id(v: &T) -> Option<::std::any::TypeId> {
                None
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Aggregate> for Global {
            fn name<'a>(_: &'a Aggregate) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Aggregate).into())
            }
            fn summary<'a>(v: &'a Aggregate) -> Option<std::borrow::Cow<'a, str>> {
                Some(("Multiple errors occurred").into())
            }
            fn detail<'a>(v: &'a Aggregate) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&v)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn inner<'a>(
                v: &'a Aggregate,
            ) -> Option<
                ::tea_codec::errorx::SmallVec<[&'a ::tea_codec::errorx::Error; 1]>,
            > {
                Some((v.0.iter().collect::<SmallVec<_>>()).into())
            }
            fn type_id<'a>(_: &'a Aggregate) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Aggregate>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<CannotBeNone> for Global {
            fn name<'a>(_: &'a CannotBeNone) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::CannotBeNone).into())
            }
            fn summary<'a>(
                __value__: &'a CannotBeNone,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a CannotBeNone,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a CannotBeNone) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<CannotBeNone>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<String> for Global {
            fn name<'a>(_: &'a String) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a String) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a String) -> Option<std::borrow::Cow<'a, str>> {
                Some((s).into())
            }
            fn type_id<'a>(_: &'a String) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<String>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<&str> for Global {
            fn name<'a>(_: &'a &str) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a &str) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a &str) -> Option<std::borrow::Cow<'a, str>> {
                Some((*s).into())
            }
            fn type_id<'a>(_: &'a &str) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<&str>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Box<str>> for Global {
            fn name<'a>(_: &'a Box<str>) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a Box<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a Box<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some((**s).into())
            }
            fn type_id<'a>(_: &'a Box<str>) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Box<str>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Rc<str>> for Global {
            fn name<'a>(_: &'a Rc<str>) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a Rc<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a Rc<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some((**s).into())
            }
            fn type_id<'a>(_: &'a Rc<str>) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Rc<str>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Arc<str>> for Global {
            fn name<'a>(_: &'a Arc<str>) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a Arc<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a Arc<str>) -> Option<std::borrow::Cow<'a, str>> {
                Some((**s).into())
            }
            fn type_id<'a>(_: &'a Arc<str>) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Arc<str>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Cow<'_, str>> for Global {
            fn name<'a>(_: &'a Cow<'_, str>) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(s: &'a Cow<'_, str>) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    ({
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["A string is thrown: \"", "\""],
                                &[::core::fmt::ArgumentV1::new_display(&s)],
                            ),
                        );
                        res
                    })
                        .into(),
                )
            }
            fn detail<'a>(s: &'a Cow<'_, str>) -> Option<std::borrow::Cow<'a, str>> {
                Some((**s).into())
            }
            fn type_id<'a>(_: &'a Cow<'_, str>) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Cow<'_, str>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Box<dyn std::error::Error + '_>>
        for Global {
            fn name<'a>(
                _: &'a Box<dyn std::error::Error + '_>,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(
                e: &'a Box<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                e: &'a Box<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a Box<dyn std::error::Error + '_>,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Box<dyn std::error::Error + '_>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Rc<dyn std::error::Error + '_>> for Global {
            fn name<'a>(
                _: &'a Rc<dyn std::error::Error + '_>,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(
                e: &'a Rc<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                e: &'a Rc<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a Rc<dyn std::error::Error + '_>,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Rc<dyn std::error::Error + '_>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Arc<dyn std::error::Error + '_>>
        for Global {
            fn name<'a>(
                _: &'a Arc<dyn std::error::Error + '_>,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Unknown).into())
            }
            fn summary<'a>(
                e: &'a Arc<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                e: &'a Arc<dyn std::error::Error + '_>,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&e)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a Arc<dyn std::error::Error + '_>,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Arc<dyn std::error::Error + '_>>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<serde_json::Error> for Global {
            fn name<'a>(
                _: &'a serde_json::Error,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::JsonSerde).into())
            }
            fn summary<'a>(
                __value__: &'a serde_json::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a serde_json::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a serde_json::Error) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<serde_json::Error>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<bincode::Error> for Global {
            fn name<'a>(_: &'a bincode::Error) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::BincodeSerde).into())
            }
            fn summary<'a>(
                __value__: &'a bincode::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a bincode::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a bincode::Error) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<bincode::Error>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<Utf8Error> for Global {
            fn name<'a>(_: &'a Utf8Error) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Utf8).into())
            }
            fn summary<'a>(
                __value__: &'a Utf8Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a Utf8Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a Utf8Error) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<Utf8Error>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<FromUtf8Error> for Global {
            fn name<'a>(_: &'a FromUtf8Error) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Utf8).into())
            }
            fn summary<'a>(
                __value__: &'a FromUtf8Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a FromUtf8Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a FromUtf8Error) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<FromUtf8Error>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<std::io::Error> for Global {
            fn name<'a>(_: &'a std::io::Error) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::StdIo).into())
            }
            fn summary<'a>(
                __value__: &'a std::io::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a std::io::Error,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a std::io::Error) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<std::io::Error>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<prost::EncodeError> for Global {
            fn name<'a>(
                _: &'a prost::EncodeError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ProstEncode).into())
            }
            fn summary<'a>(
                __value__: &'a prost::EncodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a prost::EncodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a prost::EncodeError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<prost::EncodeError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<prost::DecodeError> for Global {
            fn name<'a>(
                _: &'a prost::DecodeError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ProstDecode).into())
            }
            fn summary<'a>(
                __value__: &'a prost::DecodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a prost::DecodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a prost::DecodeError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<prost::DecodeError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<TryFromIntError> for Global {
            fn name<'a>(_: &'a TryFromIntError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::TryFrom).into())
            }
            fn summary<'a>(
                __value__: &'a TryFromIntError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a TryFromIntError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a TryFromIntError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<TryFromIntError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<TryFromCharError> for Global {
            fn name<'a>(_: &'a TryFromCharError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::TryFrom).into())
            }
            fn summary<'a>(
                __value__: &'a TryFromCharError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a TryFromCharError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a TryFromCharError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<TryFromCharError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<TryFromSliceError> for Global {
            fn name<'a>(
                _: &'a TryFromSliceError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::TryFrom).into())
            }
            fn summary<'a>(
                __value__: &'a TryFromSliceError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a TryFromSliceError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a TryFromSliceError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<TryFromSliceError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<ParseBoolError> for Global {
            fn name<'a>(_: &'a ParseBoolError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Parse).into())
            }
            fn summary<'a>(
                __value__: &'a ParseBoolError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a ParseBoolError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a ParseBoolError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<ParseBoolError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<ParseIntError> for Global {
            fn name<'a>(_: &'a ParseIntError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Parse).into())
            }
            fn summary<'a>(
                __value__: &'a ParseIntError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a ParseIntError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a ParseIntError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<ParseIntError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<ParseCharError> for Global {
            fn name<'a>(_: &'a ParseCharError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Parse).into())
            }
            fn summary<'a>(
                __value__: &'a ParseCharError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a ParseCharError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a ParseCharError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<ParseCharError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<ParseFloatError> for Global {
            fn name<'a>(_: &'a ParseFloatError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Parse).into())
            }
            fn summary<'a>(
                __value__: &'a ParseFloatError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a ParseFloatError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a ParseFloatError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<ParseFloatError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<num_traits::ParseFloatError> for Global {
            fn name<'a>(
                _: &'a num_traits::ParseFloatError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Parse).into())
            }
            fn summary<'a>(
                __value__: &'a num_traits::ParseFloatError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a num_traits::ParseFloatError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a num_traits::ParseFloatError,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<num_traits::ParseFloatError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<SystemTimeError> for Global {
            fn name<'a>(_: &'a SystemTimeError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::SystemTime).into())
            }
            fn summary<'a>(
                __value__: &'a SystemTimeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a SystemTimeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a SystemTimeError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<SystemTimeError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<ParseLevelError> for Global {
            fn name<'a>(_: &'a ParseLevelError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Log).into())
            }
            fn summary<'a>(
                __value__: &'a ParseLevelError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a ParseLevelError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a ParseLevelError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<ParseLevelError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<SetLoggerError> for Global {
            fn name<'a>(_: &'a SetLoggerError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Log).into())
            }
            fn summary<'a>(
                __value__: &'a SetLoggerError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a SetLoggerError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a SetLoggerError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<SetLoggerError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<base64::DecodeError> for Global {
            fn name<'a>(
                _: &'a base64::DecodeError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::Base64Decode).into())
            }
            fn summary<'a>(
                __value__: &'a base64::DecodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a base64::DecodeError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a base64::DecodeError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<base64::DecodeError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<FromHexError> for Global {
            fn name<'a>(_: &'a FromHexError) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::HexDecode).into())
            }
            fn summary<'a>(
                __value__: &'a FromHexError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a FromHexError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(_: &'a FromHexError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<FromHexError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<std::sync::mpsc::RecvError> for Global {
            fn name<'a>(
                _: &'a std::sync::mpsc::RecvError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ChannelReceive).into())
            }
            fn summary<'a>(
                __value__: &'a std::sync::mpsc::RecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a std::sync::mpsc::RecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a std::sync::mpsc::RecvError,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<std::sync::mpsc::RecvError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<crossbeam_channel::RecvError> for Global {
            fn name<'a>(
                _: &'a crossbeam_channel::RecvError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ChannelReceive).into())
            }
            fn summary<'a>(
                __value__: &'a crossbeam_channel::RecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a crossbeam_channel::RecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a crossbeam_channel::RecvError,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<crossbeam_channel::RecvError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<futures::channel::mpsc::TryRecvError>
        for Global {
            fn name<'a>(
                _: &'a futures::channel::mpsc::TryRecvError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ChannelReceive).into())
            }
            fn summary<'a>(
                __value__: &'a futures::channel::mpsc::TryRecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a futures::channel::mpsc::TryRecvError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a futures::channel::mpsc::TryRecvError,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<futures::channel::mpsc::TryRecvError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<futures::channel::oneshot::Canceled>
        for Global {
            fn name<'a>(
                _: &'a futures::channel::oneshot::Canceled,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ChannelReceive).into())
            }
            fn summary<'a>(
                __value__: &'a futures::channel::oneshot::Canceled,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a futures::channel::oneshot::Canceled,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a futures::channel::oneshot::Canceled,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<futures::channel::oneshot::Canceled>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<futures::channel::mpsc::SendError>
        for Global {
            fn name<'a>(
                _: &'a futures::channel::mpsc::SendError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::ChannelSend).into())
            }
            fn summary<'a>(
                __value__: &'a futures::channel::mpsc::SendError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                __value__: &'a futures::channel::mpsc::SendError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&__value__)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn type_id<'a>(
                _: &'a futures::channel::mpsc::SendError,
            ) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<futures::channel::mpsc::SendError>())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl ::tea_codec::errorx::Descriptor<crate::error::TeaError> for Global {
            fn name<'a>(
                _: &'a crate::error::TeaError,
            ) -> Option<::std::borrow::Cow<'a, str>> {
                Some((&*Global::TeaError).into())
            }
            fn summary<'a>(
                v: &'a crate::error::TeaError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&v)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn detail<'a>(
                v: &'a crate::error::TeaError,
            ) -> Option<std::borrow::Cow<'a, str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&v)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
            fn inner<'a>(
                v: &'a crate::error::TeaError,
            ) -> Option<
                ::tea_codec::errorx::SmallVec<[&'a ::tea_codec::errorx::Error; 1]>,
            > {
                Some((temp_get_tea_error_inner(v)).into())
            }
            fn type_id<'a>(_: &'a crate::error::TeaError) -> Option<::std::any::TypeId> {
                Some(::std::any::TypeId::of::<crate::error::TeaError>())
            }
        }
        pub type Error<S = Global> = ::tea_codec::errorx::Error<S>;
        pub type Result<T, E = Error> = std::result::Result<T, E>;
        fn temp_get_tea_error_inner(
            v: &crate::error::TeaError,
        ) -> SmallVec<[&Error<()>; 1]> {
            match v {
                crate::error::TeaError::NewError(e) => e.inner(),
                _ => Default::default(),
            }
        }
        impl<T> Descriptor<std::sync::mpsc::SendError<T>> for Global {
            fn name(_: &std::sync::mpsc::SendError<T>) -> Option<Cow<str>> {
                Some("ChannelSend".into())
            }
            fn summary(v: &std::sync::mpsc::SendError<T>) -> Option<Cow<str>> {
                Some(v.to_string().into())
            }
            fn detail(v: &std::sync::mpsc::SendError<T>) -> Option<Cow<str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&v)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
        }
        impl<T> Descriptor<crossbeam_channel::SendError<T>> for Global {
            fn name(_: &crossbeam_channel::SendError<T>) -> Option<Cow<str>> {
                Some("ChannelSend".into())
            }
            fn summary(v: &crossbeam_channel::SendError<T>) -> Option<Cow<str>> {
                Some(v.to_string().into())
            }
            fn detail(v: &crossbeam_channel::SendError<T>) -> Option<Cow<str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&v)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
        }
        impl<T> Descriptor<futures::channel::mpsc::TrySendError<T>> for Global {
            fn name(_: &futures::channel::mpsc::TrySendError<T>) -> Option<Cow<str>> {
                Some("ChannelSend".into())
            }
            fn summary(v: &futures::channel::mpsc::TrySendError<T>) -> Option<Cow<str>> {
                Some(v.to_string().into())
            }
            fn detail(v: &futures::channel::mpsc::TrySendError<T>) -> Option<Cow<str>> {
                Some(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_debug(&v)],
                            ),
                        );
                        res
                    }
                        .into(),
                )
            }
        }
        #[error("Value \"{0}\" cannot be none")]
        pub struct CannotBeNone(pub String);
        #[allow(unused_qualifications)]
        impl std::error::Error for CannotBeNone {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for CannotBeNone {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self(_0) = self;
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Value \"", "\" cannot be none"],
                            &[::core::fmt::ArgumentV1::new_display(&_0.as_display())],
                        ),
                    )
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CannotBeNone {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "CannotBeNone",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for CannotBeNone {
            #[inline]
            fn default() -> CannotBeNone {
                CannotBeNone(::core::default::Default::default())
            }
        }
        impl ::core::marker::StructuralPartialEq for CannotBeNone {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CannotBeNone {
            #[inline]
            fn eq(&self, other: &CannotBeNone) -> bool {
                self.0 == other.0
            }
        }
        impl ::core::marker::StructuralEq for CannotBeNone {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CannotBeNone {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CannotBeNone {
            #[inline]
            fn clone(&self) -> CannotBeNone {
                CannotBeNone(::core::clone::Clone::clone(&self.0))
            }
        }
    }
    mod scope {
        use std::{any::TypeId, borrow::Cow, marker::PhantomData};
        use super::{Error, SmallVec};
        pub trait Scope: Send + Sync + 'static {
            type Parent: Scope;
            type Descriptor<T>: Descriptor<T>;
            const NAME: &'static str;
            const FULLNAME: &'static str;
        }
        pub trait Descriptor<T> {
            fn name(v: &T) -> Option<Cow<str>>;
            fn summary(v: &T) -> Option<Cow<str>>;
            fn detail(v: &T) -> Option<Cow<str>>;
            fn inner(v: &T) -> Option<SmallVec<[&Error; 1]>>;
            fn type_id(v: &T) -> Option<TypeId>;
        }
        pub(crate) trait Descriptee: Send + Sync {
            fn name(&self) -> Option<Cow<str>>;
            fn summary(&self) -> Option<Cow<str>>;
            fn detail(&self) -> Option<Cow<str>>;
            fn inner(&self) -> Option<SmallVec<[&Error; 1]>>;
            fn type_id(&self) -> Option<TypeId>;
        }
        #[repr(transparent)]
        pub(crate) struct Dispatcher<T, S> {
            pub(crate) data: T,
            _p: PhantomData<S>,
        }
        impl<T, S> Dispatcher<T, S>
        where
            S: Scope,
        {
            pub fn new(data: T) -> Self {
                Self { data, _p: PhantomData }
            }
        }
        impl<T, S> Descriptee for Dispatcher<T, S>
        where
            T: Send + Sync,
            S: Scope,
        {
            default fn name(&self) -> Option<Cow<str>> {
                <<S as Scope>::Descriptor<T> as Descriptor<T>>::name(&self.data)
            }
            default fn summary(&self) -> Option<Cow<str>> {
                <<S as Scope>::Descriptor<T> as Descriptor<T>>::summary(&self.data)
            }
            default fn detail(&self) -> Option<Cow<str>> {
                <<S as Scope>::Descriptor<T> as Descriptor<T>>::detail(&self.data)
            }
            default fn inner(&self) -> Option<SmallVec<[&Error; 1]>> {
                <<S as Scope>::Descriptor<T> as Descriptor<T>>::inner(&self.data)
            }
            default fn type_id(&self) -> Option<TypeId> {
                <<S as Scope>::Descriptor<T> as Descriptor<T>>::type_id(&self.data)
            }
        }
    }
    mod serde {
        use std::{any::TypeId, borrow::Cow};
        use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
        use smallvec::SmallVec;
        use super::{Descriptor, Error, Global};
        pub(crate) struct SerializedData {
            name: Option<String>,
            summary: Option<String>,
            detail: Option<String>,
            pub(crate) inner: Option<SmallVec<[Error; 1]>>,
        }
        impl Descriptor<SerializedData> for Global {
            fn name(v: &SerializedData) -> Option<Cow<str>> {
                v.name.as_deref().map(Cow::Borrowed)
            }
            fn summary(v: &SerializedData) -> Option<Cow<str>> {
                v.summary.as_deref().map(Cow::Borrowed)
            }
            fn detail(v: &SerializedData) -> Option<Cow<str>> {
                v.detail.as_deref().map(Cow::Borrowed)
            }
            fn inner(v: &SerializedData) -> Option<smallvec::SmallVec<[&Error; 1]>> {
                v.inner.as_ref().map(|x| x.iter().collect())
            }
            fn type_id(_: &SerializedData) -> Option<std::any::TypeId> {
                Some(TypeId::of::<SerializedData>())
            }
        }
        impl<X> Serialize for Error<X> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(4))?;
                if let Some(name) = self.name() {
                    map.serialize_entry("name", name.as_ref())?;
                }
                if let Some(summary) = self.summary() {
                    map.serialize_entry("summary", summary.as_ref())?;
                }
                if let Some(detail) = self.detail() {
                    map.serialize_entry("detail", detail.as_ref())?;
                }
                let inner = self.inner();
                if !inner.is_empty() {
                    map.serialize_entry("inner", inner.as_ref())?;
                }
                map.end()
            }
        }
        impl<'a, X> Deserialize<'a> for Error<X> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'a>,
            {
                struct ErrorVisitor;
                impl<'de> Visitor<'de> for ErrorVisitor {
                    type Value = Error<Global>;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        formatter.write_str("Error")
                    }
                    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut name = None;
                        let mut summary = None;
                        let mut detail = None;
                        let mut inner = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                "name" => name = Some(map.next_value()?),
                                "summary" => summary = Some(map.next_value()?),
                                "detail" => detail = Some(map.next_value()?),
                                "inner" => inner = Some(map.next_value()?),
                                unknown => {
                                    return Err(
                                        serde::de::Error::unknown_field(
                                            unknown,
                                            &["name", "summary", "detail", "inner"],
                                        ),
                                    );
                                }
                            }
                        }
                        let data: Error<Global> = SerializedData {
                            name,
                            summary,
                            detail,
                            inner,
                        }
                            .into();
                        Ok(data)
                    }
                }
                deserializer.deserialize_map(ErrorVisitor).map(|x| x.into_scope())
            }
        }
        impl<S> Clone for Error<S> {
            fn clone(&self) -> Self {
                let result: Error<Global> = SerializedData {
                    name: self.name().map(Into::<_>::into),
                    summary: self.summary().map(Into::<_>::into),
                    detail: self.detail().map(Into::<_>::into),
                    inner: self
                        .data
                        .source
                        .inner()
                        .map(|inner| inner.iter().map(|x| (*x).clone()).collect()),
                }
                    .into();
                result.into_scope()
            }
        }
    }
    mod sync_error {
        use std::{
            fmt::{Debug, Display, Formatter},
            sync::Mutex,
        };
        use super::{Error, Scope};
        pub struct SyncError<T>(
            Mutex<T>,
        )
        where
            T: ?Sized + Send + 'static;
        impl<T> Display for SyncError<T>
        where
            T: ?Sized + Display + Send + 'static,
        {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let item = self.0.lock().unwrap();
                Display::fmt(&*item, f)
            }
        }
        impl<T> Debug for SyncError<T>
        where
            T: ?Sized + Debug + Send + 'static,
        {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let item = self.0.lock().unwrap();
                Debug::fmt(&*item, f)
            }
        }
        impl<T> std::error::Error for SyncError<T>
        where
            T: ?Sized + std::error::Error + Send + 'static,
        {}
        pub trait SyncErrorExt {
            fn sync_into<S>(self) -> Error<S>
            where
                S: Scope;
        }
        impl<T> SyncErrorExt for T
        where
            T: Send + 'static,
        {
            fn sync_into<S>(self) -> Error<S>
            where
                S: Scope,
            {
                SyncError(Mutex::new(self)).into()
            }
        }
        pub trait SyncResultExt {
            type Value;
            fn sync_err_into<S>(self) -> Result<Self::Value, Error<S>>
            where
                S: Scope;
        }
        impl<T, E> SyncResultExt for Result<T, E>
        where
            E: Send + 'static,
        {
            type Value = T;
            fn sync_err_into<S>(self) -> Result<T, Error<S>>
            where
                S: Scope,
            {
                self.map_err(|e| SyncError(Mutex::new(e)).into())
            }
        }
    }
    pub use global::{CannotBeNone, Global};
    pub use scope::*;
    pub use sync_error::*;
    pub use tea_error_macros::define_scope;
    pub use smallvec::SmallVec;
    use std::{
        any::TypeId, borrow::Cow, boxed::ThinBox, fmt::{Debug, Display, Formatter},
        marker::PhantomData, mem::{self, ManuallyDrop},
    };
    trait Irrelative {
        type Type;
    }
    impl<T> Irrelative for T
    where
        T: ?Sized,
    {
        type Type = !;
    }
    pub struct Error<S = ()> {
        data: ThinBox<ErrorData<dyn Descriptee>>,
        _p: PhantomData<S>,
    }
    struct ErrorData<T>
    where
        T: ?Sized,
    {
        source: T,
    }
    struct NotErrorWrapper<T>(<T as Irrelative>::Type);
    auto trait NotWrappedError {}
    impl<S> !NotWrappedError for NotErrorWrapper<Error<S>> {}
    impl<S> NotWrappedError for Error<S> {}
    trait NotError {}
    impl<T> NotError for T
    where
        NotErrorWrapper<T>: NotWrappedError,
    {}
    impl<T, S> From<T> for Error<S>
    where
        T: NotError + Send + Sync + 'static,
        S: Scope,
    {
        default fn from(data: T) -> Self {
            Self {
                data: ThinBox::new_unsize(ErrorData {
                    source: Dispatcher::<_, S>::new(data),
                }),
                _p: PhantomData,
            }
        }
    }
    mod temp {
        use crate::{error::TeaError, errorx::{Dispatcher, Error, ErrorData}};
        use std::{boxed::ThinBox, marker::PhantomData};
        impl<S> From<TeaError> for Error<S>
        where
            S: crate::errorx::Scope,
        {
            fn from(e: TeaError) -> Self {
                match e {
                    TeaError::NewError(n) => n.into_scope(),
                    data => {
                        Self {
                            data: ThinBox::new_unsize(ErrorData {
                                source: Dispatcher::<_, S>::new(data),
                            }),
                            _p: PhantomData,
                        }
                    }
                }
            }
        }
    }
    struct Equality<X, Y>(<X as Irrelative>::Type, <Y as Irrelative>::Type);
    auto trait NotEqual {}
    impl<X> !NotEqual for Equality<X, X> {}
    impl<X, Y> From<Error<X>> for Error<Y>
    where
        Equality<X, Y>: NotEqual,
    {
        fn from(source: Error<X>) -> Self {
            Self {
                data: source.data,
                _p: PhantomData,
            }
        }
    }
    impl<'a, X, Y> From<&'a Error<X>> for &'a Error<Y>
    where
        Equality<X, Y>: NotEqual,
    {
        fn from(scope: &'a Error<X>) -> Self {
            scope.as_scope()
        }
    }
    impl<S> Error<S> {
        pub fn name(&self) -> Option<Cow<str>> {
            self.data.source.name()
        }
        pub fn summary(&self) -> Option<Cow<str>> {
            self.data.source.summary()
        }
        pub fn detail(&self) -> Option<Cow<str>> {
            self.data.source.detail()
        }
        pub fn inner(&self) -> SmallVec<[&Error; 1]> {
            self.data.source.inner().unwrap_or_default()
        }
        pub fn into_scope<T>(self) -> Error<T> {
            Error {
                data: self.data,
                _p: PhantomData,
            }
        }
        pub fn as_scope<T>(&self) -> &Error<T> {
            unsafe { mem::transmute(self) }
        }
        pub fn back_cast<T>(self) -> Result<T, Self>
        where
            T: Send + Sync + 'static,
        {
            if self.data.source.type_id() == Some(TypeId::of::<T>()) {
                let mut data = self.data;
                unsafe {
                    let result = (&mut (*(&mut data.source as *mut _
                        as *mut Dispatcher<T, S>))
                        .data as *mut T)
                        .read();
                    mem::transmute::<
                        _,
                        ThinBox<ErrorData<ManuallyDrop<dyn Descriptee>>>,
                    >(data);
                    Ok(result)
                }
            } else {
                Err(self)
            }
        }
        pub fn is_name_of<T>(&self) -> bool
        where
            T: Send + Sync + Default + 'static,
            S: Scope,
        {
            Dispatcher::<T, S>::new(Default::default()).name() == self.name()
        }
        pub fn name_of<T>() -> Option<String>
        where
            T: Send + Sync + Default + 'static,
            S: Scope,
        {
            Dispatcher::<T, S>::new(Default::default()).name().map(Into::into)
        }
    }
    impl<S> Display for Error<S> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            if let Some(value) = self.summary() {
                f.write_str(value.as_ref())?;
            }
            Ok(())
        }
    }
    impl<S> Debug for Error<S> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            let mut dbg = f.debug_struct("Error");
            if let Some(value) = self.name() {
                dbg.field("name", &value.as_ref());
            }
            if let Some(value) = self.summary() {
                dbg.field("summary", &value.as_ref());
            }
            if let Some(value) = self.detail() {
                dbg.field("detail", &value.as_ref());
            }
            let inner = self.inner();
            if !inner.is_empty() {
                dbg.field("inner", &inner.as_slice());
            }
            dbg.finish()
        }
    }
    impl<S> std::error::Error for Error<S> {}
    pub trait ResultXExt {
        fn assume_error_into_backcast<E>(self) -> Option<E>
        where
            E: Send + Sync + 'static;
    }
    impl<T, S> ResultXExt for Result<T, Error<S>> {
        fn assume_error_into_backcast<E>(self) -> Option<E>
        where
            E: Send + Sync + 'static,
        {
            if let Err(e) = self {
                if let Ok(e) = e.back_cast() {
                    return Some(e);
                }
            }
            None
        }
    }
    impl<S> PartialEq for Error<S> {
        fn eq(&self, other: &Self) -> bool {
            self.name() == other.name() && self.summary() == other.summary()
                && self.detail() == other.detail() && self.inner() == other.inner()
        }
    }
    impl<S> Eq for Error<S> {}
}
