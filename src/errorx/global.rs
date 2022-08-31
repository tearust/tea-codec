use std::{
	array::TryFromSliceError,
	borrow::Cow,
	char::{ParseCharError, TryFromCharError},
	num::{ParseFloatError, ParseIntError, TryFromIntError},
	rc::Rc,
	str::{ParseBoolError, Utf8Error},
	string::FromUtf8Error,
	sync::Arc,
	time::SystemTimeError,
};

use hex::FromHexError;
use log::{ParseLevelError, SetLoggerError};
use smallvec::SmallVec;
use thiserror::Error;

use super::{aggregate::Aggregate, Descriptor};

tea_error_macros::define_scope_internal! {
	Global {
		Aggregate as v => Aggregate, "Multiple errors occurred", format!("{:?}", v), v.0.iter().collect::<SmallVec<_>>();
		CannotBeNone => CannotBeNone, @Display, @Debug;
		String as s => Unknown, format!("A string is thrown: \"{s}\""), s;
		&str as s => Unknown, format!("A string is thrown: \"{s}\""), *s;
		Box<str> as s => Unknown, format!("A string is thrown: \"{s}\""), **s;
		Rc<str> as s => Unknown, format!("A string is thrown: \"{s}\""), **s;
		Arc<str> as s => Unknown, format!("A string is thrown: \"{s}\""), **s;
		Cow<'_, str> as s => Unknown, format!("A string is thrown: \"{s}\""), **s;
		Box<dyn std::error::Error + '_> as e => Unknown, @Display, @Debug;
		Rc<dyn std::error::Error + '_> as e => Unknown, @Display, @Debug;
		Arc<dyn std::error::Error + '_> as e => Unknown, @Display, @Debug;
		serde_json::Error => JsonSerde, @Display, @Debug;
		bincode::Error => BincodeSerde, @Display, @Debug;
		Utf8Error => Utf8, @Display, @Debug;
		FromUtf8Error => Utf8, @Display, @Debug;
		std::io::Error => StdIo, @Display, @Debug;
		prost::EncodeError => ProstEncode, @Display, @Debug;
		prost::DecodeError => ProstDecode, @Display, @Debug;
		TryFromIntError => TryFrom, @Display, @Debug;
		TryFromCharError => TryFrom, @Display, @Debug;
		TryFromSliceError => TryFrom, @Display, @Debug;
		ParseBoolError => Parse, @Display, @Debug;
		ParseIntError => Parse, @Display, @Debug;
		ParseCharError => Parse, @Display, @Debug;
		ParseFloatError => Parse, @Display, @Debug;
		num_traits::ParseFloatError => Parse, @Display, @Debug;
		SystemTimeError => SystemTime, @Display, @Debug;
		ParseLevelError => Log, @Display, @Debug;
		SetLoggerError => Log, @Display, @Debug;
		base64::DecodeError => Base64Decode, @Display, @Debug;
		FromHexError => HexDecode, @Display, @Debug;
		std::sync::mpsc::RecvError => ChannelReceive, @Display, @Debug;
		crossbeam_channel::RecvError => ChannelReceive, @Display, @Debug;
		futures::channel::mpsc::TryRecvError => ChannelReceive, @Display, @Debug;
		futures::channel::oneshot::Canceled => ChannelReceive, @Display, @Debug;
		futures::channel::mpsc::SendError => ChannelSend, @Display, @Debug;
		crate::error::TeaError as v => TeaError, @Display, @Debug, temp_get_tea_error_inner(v);
	}
}

fn temp_get_tea_error_inner(v: &crate::error::TeaError) -> SmallVec<[&Error<()>; 1]> {
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
		Some(format!("{:?}", v).into())
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
		Some(format!("{:?}", v).into())
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
		Some(format!("{:?}", v).into())
	}
}

#[derive(Error, Debug, Default, PartialEq, Eq, Clone)]
#[error("Value \"{0}\" cannot be none")]
pub struct CannotBeNone(pub String);
