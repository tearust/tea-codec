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
		Self {
			data,
			_p: PhantomData,
		}
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
