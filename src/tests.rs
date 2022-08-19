use crate::define_scope;
use crate::errorx::InnerError;

define_scope! {
	Test {
		i32 as v => I32, @Debug, v.to_string();
		bool => Bool, @Serde;
	}
}

#[test]
fn test() {
	let e = foo().unwrap_err();
	assert_eq!(e.name(), Some("Unknown".into()));
	assert_eq!(e.summary(), Some("A string is thrown: \"456\"".into()));
	assert_eq!(e.detail(), Some("456".into()));
	assert_eq!(e.inner_error()[0].name(), Some("Test.I32".into()));
	assert_eq!(e.inner_error()[0].detail(), Some("123".into()));
}

fn foo() -> Result<()> {
	baz()?;
	Ok(())
}

fn bar() -> Result<(), i32> {
	Err(123)
}

fn baz() -> Result<()> {
	Err("456".with_inner_error(bar().unwrap_err()))
}
