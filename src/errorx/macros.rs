#[macro_export]
macro_rules! impl_error {
	($sc:ident, $c:expr, $t:ty, $sf:ident, $s:expr, $d:expr) => {
		impl $crate::errorx::ErrorInfo for $t {
			fn identity(&self) -> Option<$crate::errorx::ErrorIdentity> {
				Some($crate::errorx::ErrorIdentity {
					scope: $crate::errorx::ErrorScope::$sc as _,
					code: $c as _,
				})
			}

			fn summary(&self) -> Option<std::borrow::Cow<str>> {
				let $sf = self;
				Some($s.into())
			}

			fn detail(&self) -> Option<std::borrow::Cow<str>> {
				let $sf = self;
				Some($d.into())
			}
		}
	};

	($sc:ident, $c:ident, $t:ty, $sf:ident, $s:expr) => {
		impl_error!($sc, $c, $t, $sf, $s, format!("{:?}", $sf));
	};

	($sc:ident, $c:ident, $t:ty) => {
		impl_error!($sc, $c, $t, x, x.to_string());
	};

    {
		$(scope $sc:ident {
        	$($c:ident: $t:ty $(, $sf:ident, $s:expr $(, $d:expr)?)?;)*
		})*
    } => {
        $($(impl_error!($sc, $c, $t $(, $sf, $s $(, $d)?)?);)*)*
    }
}

#[macro_export]
macro_rules! impl_deref {
	() => {
		fn identity(&self) -> Option<ErrorIdentity> {
			(**self).identity()
		}

		fn summary(&self) -> Option<Cow<str>> {
			(**self).summary()
		}

		fn detail(&self) -> Option<Cow<str>> {
			(**self).detail()
		}

		fn inner(&self) -> Box<dyn Iterator<Item = &Error> + '_> {
			(**self).inner()
		}
	};

	($t:ident: $($c:ty),*) => {$(
		impl<$t> ErrorInfo for $c
		where
			$t: ErrorInfo,
		{
			impl_deref!();
		}
	)*};
}
