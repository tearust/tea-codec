use crate::impl_error;
use std::{
	array::TryFromSliceError,
	borrow::Cow,
	char::{ParseCharError, TryFromCharError},
	num::{ParseFloatError, ParseIntError, TryFromIntError},
	rc::Rc,
	str::{ParseBoolError, Utf8Error},
	string::{FromUtf8Error, ParseError},
	sync::Arc,
	time::SystemTimeError,
};

use hex::FromHexError;
use log::{ParseLevelError, SetLoggerError};
use CommonCode::*;

#[repr(u16)]
pub enum CommonCode {
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
}

impl_error! {
	scope Common {
		Unknown: anyhow::Error;
		Unknown: String, x, x, x;
		Unknown: &'static str, x, *x, *x;
		Unknown: Box<str>, x, **x, **x;
		Unknown: Rc<str>, x, **x, **x;
		Unknown: Arc<str>, x, **x, **x;
		Unknown: Cow<'static, str>, x, **x, **x;
		Unknown: Box<dyn std::error::Error>;
		Unknown: Rc<dyn std::error::Error>;
		Unknown: Arc<dyn std::error::Error>;
		JsonSerde: serde_json::Error;
		BincodeSerde: bincode::Error;
		Utf8: Utf8Error;
		Utf8: FromUtf8Error;
		StdIo: std::io::Error;
		ProstEncode: prost::EncodeError;
		ProstDecode: prost::DecodeError;
		TryFrom: TryFromIntError;
		TryFrom: TryFromCharError;
		TryFrom: TryFromSliceError;
		Parse: ParseBoolError;
		Parse: ParseIntError;
		Parse: ParseCharError;
		Parse: ParseFloatError;
		Parse: ParseError;
		SystemTime: SystemTimeError;
		Log: ParseLevelError;
		Log: SetLoggerError;
		Base64Decode: base64::DecodeError;
		HexDecode: FromHexError;
	}
}
