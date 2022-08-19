use crate::{
	define_scope,
	errorn::{Descriptor, InnerError, Scope},
};

define_scope! {
	scope TestScope {
		String: Test, "string error", serde;
	}
}

#[test]
fn test() {
	let e = foo().unwrap_err();
	assert_eq!(e.name(), Some("TestScope.Test".into()));
	assert_eq!(e.summary(), Some("string error".into()));
	assert_eq!(e.detail(), Some("\"bbbbb\"".into()));
	assert_eq!(e.inner_error()[0].detail(), Some("\"aaaa\"".into()));
}

fn foo() -> Result<()> {
	baz()?;
	Ok(())
}

fn bar() -> Result<(), String> {
	Err("aaaa".to_string())
}

fn baz() -> Result<()> {
	Err("bbbbb".to_string().with_inner_error(bar().unwrap_err()))
}
