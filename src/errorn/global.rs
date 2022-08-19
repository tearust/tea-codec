use std::borrow::Cow;

use super::{Descriptor, Scope};

pub struct Global;

impl Scope for Global {
	type Parent = Self;
	type Descriptor<T> = Self;
	const NAME: &'static str = "global";
}

impl<T> Descriptor<T> for Global {
	default fn name(_: &T) -> Option<Cow<str>> {
		None
	}

	default fn summary(_: &T) -> Option<Cow<str>> {
		None
	}

	default fn detail(_: &T) -> Option<Cow<str>> {
		None
	}
}
