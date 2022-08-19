mod global;
mod macros;
mod scope;
mod serde;
use std::{borrow::Cow, boxed::ThinBox, marker::PhantomData, mem};

pub use global::*;
pub use macros::*;
pub use scope::*;
use smallvec::SmallVec;

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
	T: NotError + 'static,
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
	T: NotError + 'static,
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
