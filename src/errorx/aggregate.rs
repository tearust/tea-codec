use std::ops::Add;

use smallvec::{smallvec, SmallVec};

use super::{serde::SerializedData, Error, Global};

#[derive(Debug)]
pub(crate) struct Aggregate(pub(crate) SmallVec<[Error; 2]>);

impl<LS, RS> Add<Error<RS>> for Error<LS> {
	type Output = Self;
	fn add(self, rhs: Error<RS>) -> Self::Output {
		let lhs = match self.back_cast::<Aggregate>() {
			Ok(mut agg) => {
				agg.0.push(rhs.into());
				return Error::<Global>::from(agg).into();
			}
			Err(lhs) => lhs,
		};

		let lhs = if lhs.name() == Some("Aggregate".into()) {
			match lhs.back_cast::<SerializedData>() {
				Ok(mut agg) => {
					match &mut agg.inner {
						Some(inner) => inner.push(rhs.into()),
						None => agg.inner = Some(smallvec![rhs.into()]),
					}
					return Error::<Global>::from(agg).into();
				}
				Err(lhs) => lhs,
			}
		} else {
			lhs
		};

		Error::<Global>::from(Aggregate(smallvec![lhs.into(), rhs.into()])).into()
	}
}
