use std::{
	any::TypeId,
	borrow::Cow,
	fmt::{Display, Formatter, Write},
	marker::PhantomData,
};

use super::{Error, Global, SmallVec};

pub trait Scope: Send + Sync + 'static {
	type Parent: Scope;
	type Descriptor<T>: Descriptor<T>;
	const NAME: &'static str;
}

pub trait ScopeExt {
	fn full_name_fmt(f: &mut Formatter) -> Result<bool, std::fmt::Error>;
	fn full_name() -> String;
	fn error_full_name(name: &str, is_raw: bool) -> String;
}

impl<T> ScopeExt for T
where
	T: Scope,
{
	default fn full_name_fmt(f: &mut Formatter) -> Result<bool, std::fmt::Error> {
		if <T::Parent as ScopeExt>::full_name_fmt(f)? {
			f.write_char('.')?;
		}
		f.write_str(T::NAME)?;
		Ok(true)
	}
	fn full_name() -> String {
		struct Format<T>(PhantomData<T>)
		where
			T: ?Sized;

		impl<T> Display for Format<T>
		where
			T: Scope + ScopeExt + ?Sized,
		{
			fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
				if !T::full_name_fmt(f)? {
					f.write_str(T::NAME)?;
				}
				Ok(())
			}
		}

		Format::<Self>(PhantomData).to_string()
	}
	fn error_full_name(name: &str, is_raw: bool) -> String {
		struct Format<'a, T>(&'a str, PhantomData<T>)
		where
			T: ?Sized;

		impl<T> Display for Format<'_, T>
		where
			T: ScopeExt + ?Sized,
		{
			fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
				if T::full_name_fmt(f)? {
					f.write_char('.')?;
				}
				f.write_str(self.0)
			}
		}

		if is_raw {
			name.to_string()
		} else {
			Format::<Self>(name, PhantomData).to_string()
		}
	}
}

impl ScopeExt for Global {
	fn full_name_fmt(_: &mut Formatter) -> Result<bool, std::fmt::Error> {
		Ok(false)
	}
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
		<<S as Scope>::Descriptor<T> as Descriptor<T>>::name(&self.data).or_else(|| {
			<<<S as Scope>::Parent as Scope>::Descriptor<T> as Descriptor<T>>::name(&self.data)
		})
	}

	default fn summary(&self) -> Option<Cow<str>> {
		<<S as Scope>::Descriptor<T> as Descriptor<T>>::summary(&self.data).or_else(|| {
			<<<S as Scope>::Parent as Scope>::Descriptor<T> as Descriptor<T>>::summary(&self.data)
		})
	}

	default fn detail(&self) -> Option<Cow<str>> {
		<<S as Scope>::Descriptor<T> as Descriptor<T>>::detail(&self.data).or_else(|| {
			<<<S as Scope>::Parent as Scope>::Descriptor<T> as Descriptor<T>>::detail(&self.data)
		})
	}

	default fn inner(&self) -> Option<SmallVec<[&Error; 1]>> {
		<<S as Scope>::Descriptor<T> as Descriptor<T>>::inner(&self.data).or_else(|| {
			<<<S as Scope>::Parent as Scope>::Descriptor<T> as Descriptor<T>>::inner(&self.data)
		})
	}

	default fn type_id(&self) -> Option<TypeId> {
		<<S as Scope>::Descriptor<T> as Descriptor<T>>::type_id(&self.data).or_else(|| {
			<<<S as Scope>::Parent as Scope>::Descriptor<T> as Descriptor<T>>::type_id(&self.data)
		})
	}
}

impl<T> Descriptee for Dispatcher<T, Global>
where
	T: Send + Sync,
{
	fn name(&self) -> Option<Cow<str>> {
		<Global as Descriptor<T>>::name(&self.data)
	}

	fn summary(&self) -> Option<Cow<str>> {
		<Global as Descriptor<T>>::summary(&self.data)
	}

	fn detail(&self) -> Option<Cow<str>> {
		<Global as Descriptor<T>>::detail(&self.data)
	}

	fn type_id(&self) -> Option<TypeId> {
		<Global as Descriptor<T>>::type_id(&self.data)
	}
}
