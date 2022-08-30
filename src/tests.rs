use crate::errorx::define_scope;

define_scope! {
	Test {
		I32;
		i32 as v => @test::I32, @Debug, v.to_string();
		bool => Bool, @Serde;
		HasInner as x => HasInner, @Debug, @Debug, [&x.0];
	}
}

#[derive(Debug)]
struct HasInner(Error<()>);

#[test]
fn test() {
	assert_eq!(test::I32, "Test.I32");
	let ex = foo().unwrap_err();
	let s = serde_json::to_string(&ex.clone()).unwrap();
	let e: Error = serde_json::from_str(&s).unwrap();
	assert_eq!(e.name(), Some("Test.HasInner".into()));
	assert_eq!(e.inner()[0].name(), Some("Test.I32".into()));
	assert_eq!(e.inner()[0].detail(), Some("123".into()));
	let e = ex.back_cast::<HasInner>().unwrap();
	assert_eq!(e.0.back_cast::<i32>().unwrap(), 123);
	let sum = bar().unwrap_err() + bar().unwrap_err() + bar().unwrap_err();
	assert_eq!(sum.inner().len(), 3);
	let sum = serde_json::from_str::<Error>(
		serde_json::to_string(&(bar().unwrap_err() + bar().unwrap_err()))
			.unwrap()
			.as_str(),
	)
	.unwrap()
		+ bar().unwrap_err();
	assert_eq!(sum.inner().len(), 3);
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
