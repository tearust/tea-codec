use crate::{define_scope, errorx::single};

define_scope! {
	Test {
		i32 as v => I32, @Debug, v.to_string();
		bool => Bool, @Serde;
		HasInner as x => HasInner, @Debug, @Debug, single(&x.0);
	}
}

#[derive(Debug)]
struct HasInner(Error<()>);

#[test]
fn test() {
	let ex = foo().unwrap_err();
	let s = serde_json::to_string(&ex.clone()).unwrap();
	let e: Error = serde_json::from_str(&s).unwrap();
	assert_eq!(e.name(), Some("Test.HasInner".into()));
	assert_eq!(e.inner()[0].name(), Some("Test.I32".into()));
	assert_eq!(e.inner()[0].detail(), Some("123".into()));
	let e = ex.back_cast::<HasInner>().unwrap();
	assert_eq!(e.0.back_cast::<i32>().unwrap(), 123);
}

fn foo() -> Result<()> {
	baz()?;
	Ok(())
}

fn bar() -> Result<(), Error> {
	Err(123.into())
}

fn baz() -> Result<()> {
	Err(HasInner(bar().unwrap_err().into()).into())
}
