use std::{
	fmt::{Debug, Display, Formatter},
	sync::Mutex,
};

use super::{Error, Scope};

pub struct SyncError<T>(Mutex<T>)
where
	T: ?Sized + Send + 'static;

impl<T> Display for SyncError<T>
where
	T: ?Sized + Display + Send + 'static,
{
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let item = self.0.lock().unwrap();
		Display::fmt(&*item, f)
	}
}

impl<T> Debug for SyncError<T>
where
	T: ?Sized + Debug + Send + 'static,
{
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let item = self.0.lock().unwrap();
		Debug::fmt(&*item, f)
	}
}

impl<T> std::error::Error for SyncError<T> where T: ?Sized + std::error::Error + Send + 'static {}

trait SyncErrorExt {
	fn sync_into<S>(self) -> Error<S>
	where
		S: Scope;
}

impl<T> SyncErrorExt for T
where
	T: Send + 'static,
{
	fn sync_into<S>(self) -> Error<S>
	where
		S: Scope,
	{
		SyncError(Mutex::new(self)).into()
	}
}

trait SyncResultExt {
	type Value;
	fn sync_err_into<S>(self) -> Result<Self::Value, Error<S>>
	where
		S: Scope;
}

impl<T, E> SyncResultExt for Result<T, E>
where
	E: Send + 'static,
{
	type Value = T;
	fn sync_err_into<S>(self) -> Result<T, Error<S>>
	where
		S: Scope,
	{
		self.map_err(|e| SyncError(Mutex::new(e)).into())
	}
}
