use std::{
	array::TryFromSliceError,
	borrow::Cow,
	char::{ParseCharError, TryFromCharError},
	num::{ParseFloatError, ParseIntError, TryFromIntError},
	rc::Rc,
	str::{ParseBoolError, Utf8Error},
	string::FromUtf8Error,
	sync::{mpsc::RecvError, Arc, PoisonError},
	time::SystemTimeError,
};

use hex::FromHexError;
use log::{ParseLevelError, SetLoggerError};
use smallvec::SmallVec;

use super::{aggregate::Aggregate, Descriptor};

crate::define_scope! {
	Global {
		Aggregate as v => Aggregate, "Multiple errors occurred", format!("{:?}", v), v.0.iter().collect::<SmallVec<_>>();
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
		RecvError => ChannelReceive, @Display, @Debug;
	}
}

impl<T> Descriptor<PoisonError<T>> for Global {
	fn name(_: &PoisonError<T>) -> Option<Cow<str>> {
		Some("Poisoned".into())
	}

	fn summary(v: &PoisonError<T>) -> Option<Cow<str>> {
		Some(v.to_string().into())
	}

	fn detail(v: &PoisonError<T>) -> Option<Cow<str>> {
		Some(format!("{:?}", v).into())
	}
}
