use std::{
	any::{Any, TypeId},
	borrow::Cow,
	fmt::{Debug, Display},
	marker::PhantomData,
	mem::{self, ManuallyDrop},
	ops::{Deref, DerefMut},
	ptr::NonNull,
	rc::Rc,
	sync::Arc,
};

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use smallvec::SmallVec;

mod macros;
pub use macros::*;

#[allow(unused_variables)]
mod common;
pub use common::*;

use crate::impl_deref;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error {
	ptr: NonNull<usize>,
	_p: PhantomData<dyn ErrorInfo>,
}

unsafe impl Send for Error where Error: Send {}
unsafe impl Sync for Error where Error: Sync {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ErrorIdentity {
	pub scope: u16,
	pub code: u16,
}

pub trait ErrorInfo: Any + ErrorInfoDrop {
	fn identity(&self) -> Option<ErrorIdentity> {
		None
	}

	fn summary(&self) -> Option<Cow<str>> {
		None
	}

	fn inner(&self) -> Box<dyn Iterator<Item = &Error> + '_> {
		Box::new([].iter())
	}

	fn detail(&self) -> Option<Cow<str>> {
		None
	}
}

#[doc(hidden)]
pub trait ErrorInfoDrop {
	#[doc(hidden)]
	unsafe fn drop_error(&mut self);
}

impl<T> ErrorInfoDrop for T
where
	T: ErrorInfo,
{
	unsafe fn drop_error(&mut self) {
		mem::drop(Box::from_raw(
			(self as *mut Self as *mut usize).offset(-1) as *mut (usize, Self)
		))
	}
}

pub enum SerializeImpl<'a> {
	Borrowed(&'a (dyn erased_serde::Serialize + 'a)),
	Boxed(Box<dyn erased_serde::Serialize + 'a>),
}

impl Error {
	pub fn new<T>(mut value: T) -> Self
	where
		T: ErrorInfo,
	{
		Self {
			ptr: unsafe {
				NonNull::new_unchecked(Box::into_raw(Box::new((
					mem::transmute::<_, &mut (usize, usize)>(&mut (
						&mut value as *mut T as *mut dyn ErrorInfo,
						0usize,
					))
					.1,
					value,
				))) as _)
			},
			_p: PhantomData,
		}
	}

	pub fn scope(&self) -> Option<u16> {
		self.identity().map(|ErrorIdentity { scope, .. }| scope)
	}

	pub fn code(&self) -> Option<u16> {
		self.identity().map(|ErrorIdentity { code, .. }| code)
	}

	unsafe fn get_fat_ptr(&self) -> *mut (dyn ErrorInfo) {
		mem::transmute((self.ptr.as_ptr().offset(1), *self.ptr.as_ref()))
	}
}

impl Drop for Error {
	fn drop(&mut self) {
		unsafe {
			self.drop_error();
		}
	}
}

impl Deref for Error {
	type Target = dyn ErrorInfo;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.get_fat_ptr() }
	}
}

impl DerefMut for Error {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.get_fat_ptr() }
	}
}

impl<T> From<T> for Error
where
	T: ErrorInfo,
{
	fn from(v: T) -> Self {
		Self::new(v)
	}
}

impl<T> From<Error> for Result<T, Error>
where
	T: ErrorInfo,
{
	fn from(error: Error) -> Result<T, Error> {
		unsafe {
			let ptr = error.get_fat_ptr();
			if (*ptr).type_id() == TypeId::of::<T>() {
				let result = (ptr as *mut T).read();
				mem::drop(Box::from_raw(
					error.ptr.as_ptr() as *mut ManuallyDrop<(usize, T)>
				));
				mem::forget(error);
				Ok(result)
			} else {
				Err(error)
			}
		}
	}
}

impl Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut f = f.debug_struct("Error");
		if let Some(ErrorIdentity { scope, code }) = self.identity() {
			f.field("scope", &scope).field("code", &code);
		}
		if let Some(summary) = self.summary() {
			f.field("summary", &summary.as_ref());
		}
		if let Some(detail) = self.detail() {
			f.field("detail", &detail.as_ref());
		}
		let inner = self.inner().collect::<SmallVec<[_; 1]>>();
		if inner.len() > 1 {
			f.field("inner", &inner.as_slice());
		} else {
			f.field("inner", &inner.first());
		}

		f.finish()
	}
}

impl std::error::Error for Error {}

impl Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut map = serializer.serialize_map(Some(5))?;
		if let Some(ErrorIdentity { scope, code }) = self.identity() {
			map.serialize_key("scope")?;
			map.serialize_value(&scope)?;
			map.serialize_key("code")?;
			map.serialize_value(&code)?;
		}
		if let Some(summary) = (**self).summary() {
			map.serialize_key("summary")?;
			map.serialize_value(summary.as_ref())?;
		}
		if let Some(detail) = (**self).detail() {
			map.serialize_key("detail")?;
			map.serialize_value(detail.as_ref())?;
		}
		map.serialize_key("inner")?;
		map.serialize_value(self.inner().collect::<Vec<_>>().as_slice())?;
		map.end()
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(
			serde_json::to_string(self)
				.map_err(|_| std::fmt::Error)?
				.as_str(),
		)
	}
}

#[derive(Debug)]
struct SerializedError {
	identity: Option<ErrorIdentity>,
	summary: Option<String>,
	detail: Option<String>,
	inner: SmallVec<[Error; 1]>,
}

impl ErrorInfo for SerializedError {
	fn identity(&self) -> Option<ErrorIdentity> {
		self.identity
	}

	fn summary(&self) -> Option<Cow<str>> {
		self.summary.as_deref().map(|x| x.into())
	}

	fn detail(&self) -> Option<Cow<str>> {
		self.detail.as_deref().map(|x| x.into())
	}

	fn inner(&self) -> Box<dyn Iterator<Item = &Error> + '_> {
		Box::new(self.inner.iter())
	}
}

impl<'a> Deserialize<'a> for Error {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'a>,
	{
		struct ErrorVisitor;
		impl<'de> Visitor<'de> for ErrorVisitor {
			type Value = Error;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("Error")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
			where
				V: serde::de::MapAccess<'de>,
			{
				let mut scope = None;
				let mut code = None;
				let mut summary = None;
				let mut detail = None;
				let mut inner = None;

				while let Some(key) = map.next_key()? {
					match key {
						"scope" => scope = Some(map.next_value()?),
						"code" => code = Some(map.next_value()?),
						"summary" => summary = Some(map.next_value()?),
						"detail" => detail = Some(map.next_value()?),
						"inner" => inner = Some(map.next_value()?),
						unknown => {
							return Err(serde::de::Error::unknown_field(
								unknown,
								&["scope", "code", "summary", "detail", "inner"],
							))
						}
					}
				}

				Ok(SerializedError {
					identity: match (scope, code) {
						(Some(scope), Some(code)) => Some(ErrorIdentity { scope, code }),
						(None, None) => None,
						(None, _) => return Err(serde::de::Error::missing_field("scope")),
						(_, None) => return Err(serde::de::Error::missing_field("code")),
					},
					summary: if let Some(summary) = summary {
						summary
					} else {
						return Err(serde::de::Error::missing_field("summary"));
					},
					detail,
					inner: if let Some(inner) = inner {
						inner
					} else {
						return Err(serde::de::Error::missing_field("detail"));
					},
				}
				.into())
			}
		}
		deserializer.deserialize_map(ErrorVisitor)
	}
}

impl_deref!(T: &'static T, Box<T>, Rc<T>, Arc<T>);

impl<T> ErrorInfo for Cow<'static, T>
where
	T: ErrorInfo + Clone,
{
	impl_deref!();
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ErrorScope {
	Common,
	Client,
	Layer1,
	Service,
	Vmh,
	Wascc,
}
