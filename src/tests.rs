use crate::errorx::{Catch, Result};

fn fail1() -> Result<()> {
	Err("error message")?;
	Ok(())
}

fn fail2() -> Result<()> {
	let _: i32 = "abc".parse()?;
	Ok(())
}

fn fail3() -> Result<()> {
	let _: (i32, i32) = serde_json::from_str("aaa")?;
	Ok(())
}

fn catch_json_error() -> Result<bool> {
	let x: serde_json::Error = fail3().catch()?.unwrap_err();
	Ok(x.to_string() == "expected value at line 1 column 1")
}

#[test]
fn test_error() {
	assert_eq!(
		fail1().unwrap_err().summary().as_deref(),
		Some("error message")
	);

	assert_eq!(format!("{:?}", fail2().unwrap_err()), "Error { scope: 0, code: 8, summary: \"invalid digit found in string\", detail: \"ParseIntError { kind: InvalidDigit }\", inner: None }");

	assert!(catch_json_error().unwrap());
}
