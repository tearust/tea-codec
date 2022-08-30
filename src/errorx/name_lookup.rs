use std::{
	borrow::{Borrow, Cow},
	cell::UnsafeCell,
	fmt::{Debug, Display},
	marker::PhantomData,
	ops::Deref,
};

use super::{Scope, ScopeExt};

pub struct ErrorName<S> {
	input: &'static str,
	output: UnsafeCell<Option<String>>,
	_p: PhantomData<S>,
}

unsafe impl<S> Sync for ErrorName<S> {}

impl<S> ErrorName<S>
where
	S: ScopeExt,
{
	pub const fn new(v: &'static str) -> Self {
		Self {
			input: v,
			output: UnsafeCell::new(None),
			_p: PhantomData,
		}
	}

	fn generate(&self) -> String {
		S::error_full_name(self.input, false)
	}
}

impl<S> Deref for ErrorName<S>
where
	S: Scope,
{
	type Target = str;
	fn deref(&self) -> &Self::Target {
		loop {
			if let Some(s) = unsafe { &*self.output.get() }.as_ref() {
				return s;
			}

			unsafe {
				*self.output.get() = Some(self.generate());
			}
		}
	}
}

impl<S> AsRef<str> for ErrorName<S>
where
	S: Scope,
{
	fn as_ref(&self) -> &str {
		self
	}
}

impl<S> Borrow<str> for ErrorName<S>
where
	S: Scope,
{
	fn borrow(&self) -> &str {
		self
	}
}

impl<S> Display for ErrorName<S>
where
	S: Scope,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		Display::fmt(&**self, f)
	}
}

impl<S> Debug for ErrorName<S>
where
	S: Scope,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		Display::fmt(&**self, f)
	}
}

impl<S> PartialEq for ErrorName<S>
where
	S: Scope,
{
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<S> PartialEq<str> for ErrorName<S>
where
	S: Scope,
{
	fn eq(&self, other: &str) -> bool {
		self.as_ref() == other
	}
}

impl<S> Eq for ErrorName<S> where S: Scope {}

impl<'a, S> From<&'a ErrorName<S>> for Cow<'a, str>
where
	S: Scope,
{
	fn from(s: &'a ErrorName<S>) -> Self {
		(&**s).into()
	}
}

impl<'a, S> From<&'a ErrorName<S>> for &'a str
where
	S: Scope,
{
	fn from(s: &'a ErrorName<S>) -> Self {
		s
	}
}
