mod global;
mod macros;
mod scope;
mod serde;

pub use global::Global;
pub use scope::*;

use smallvec::SmallVec;
use std::{
	borrow::Cow,
	boxed::ThinBox,
	fmt::{Debug, Display, Formatter},
	marker::PhantomData,
	mem,
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
	inner: SmallVec<[Error; 1]>,
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
				inner: SmallVec::new(),
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

	pub fn inner_error(&self) -> &[Error] {
		self.data.inner.as_slice()
	}

	pub fn into_scope<T>(self) -> Error<T> {
		Error {
			data: self.data,
			_p: PhantomData,
		}
	}

	pub fn with_inner_error<T>(mut self, inner: T) -> Self
	where
		T: Into<Self>,
	{
		self.data.inner.push(inner.into().into());
		self
	}

	pub fn as_scope<T>(&self) -> &Error<T> {
		unsafe { mem::transmute(self) }
	}
}

pub trait InnerError {
	fn with_inner_error<I, O>(self, e: I) -> Error<O>
	where
		I: Into<Error<O>>,
		O: Scope;
}

impl<T> InnerError for T
where
	T: NotError + Send + Sync + 'static,
{
	fn with_inner_error<I, O>(self, e: I) -> Error<O>
	where
		I: Into<Error<O>>,
		O: Scope,
	{
		let mut result: Error<O> = self.into();
		result.data.inner.push(e.into().into());
		result
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
		if !self.data.inner.is_empty() {
			dbg.field("inner_error", &self.data.inner.as_slice());
		}
		dbg.finish()
	}
}

impl<S> std::error::Error for Error<S> {}
