use std::borrow::Cow;

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use smallvec::SmallVec;

use super::{Descriptor, Error, Global};

struct SerializedData {
	name: Option<String>,
	summary: Option<String>,
	detail: Option<String>,
	inner: Option<SmallVec<[Error; 1]>>,
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

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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
							return Err(serde::de::Error::unknown_field(
								unknown,
								&["name", "summary", "detail", "inner"],
							))
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
		deserializer
			.deserialize_map(ErrorVisitor)
			.map(|x| x.into_scope())
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
