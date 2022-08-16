use std::borrow::Cow;

use serde::Serialize;

use crate::errorx::{Error, ErrorIdentity, ErrorInfo};

#[derive(Debug, PartialEq, Eq, Serialize)]
struct Foo(u16, u16, String);

impl ErrorInfo for Foo {
	fn identity(&self) -> Option<ErrorIdentity> {
		Some(ErrorIdentity {
			scope: self.0,
			code: self.1,
		})
	}

	fn summary(&self) -> Option<Cow<str>> {
		Some(self.2.as_str().into())
	}

	fn detail(&self) -> Option<Cow<str>> {
		None
	}
}

#[test]
fn test_error() {
	let foo = Foo(1, 2, "abc".to_string());
	let error: Error = foo.into();
	let bar: Result<Foo, _> = error.into();
	let bar = bar.unwrap();
	assert_eq!(bar, Foo(1, 2, "abc".to_string()));
}
