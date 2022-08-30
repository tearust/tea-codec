mod aggregate;
mod global;
mod name_lookup;
mod scope;
mod serde;
mod sync_error;

pub use global::{CannotBeNone, Global};
pub use name_lookup::*;
pub use scope::*;
pub use sync_error::*;
pub use tea_error_macros::define_scope;

pub use smallvec::SmallVec;

use std::{
	any::TypeId,
	borrow::Cow,
	boxed::ThinBox,
	fmt::{Debug, Display, Formatter},
	marker::PhantomData,
	mem::{self, ManuallyDrop},
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
impl<T> NotError for T where NotErrorWrapper<T>: NotWrappedError {}

impl<T, S> From<T> for Error<S>
where
	T: NotError + Send + Sync + 'static,
	S: Scope,
{
	fn from(data: T) -> Self {
		Self {
			data: ThinBox::new_unsize(ErrorData {
				source: Dispatcher::<_, S>::new(data),
			}),
			_p: PhantomData,
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
				let result = (&mut (*(&mut data.source as *mut _ as *mut Dispatcher<T, S>)).data
					as *mut T)
					.read();
				mem::transmute::<_, ThinBox<ErrorData<ManuallyDrop<dyn Descriptee>>>>(data);
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
		Dispatcher::<T, S>::new(Default::default())
			.name()
			.map(Into::into)
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
		self.name() == other.name()
			&& self.summary() == other.summary()
			&& self.detail() == other.detail()
			&& self.inner() == other.inner()
	}
}

impl<S> Eq for Error<S> {}
