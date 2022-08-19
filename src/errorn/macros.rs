#[doc(hidden)]
#[macro_export]
macro_rules! parent_scope {
	($parent:ty) => {
		$parent
	};
	() => {
		$crate::errorn::Global
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! scope_serde {
	($v:expr, serde) => {
		serde_json::to_string($v).ok().map(Into::into)
	};
	($v:expr,) => {
		None
	};
}

#[macro_export]
macro_rules! define_scope {
    {$(
        scope $name:ident $(: $parent:ty)? {$(
            $t:ty: $id:ident, $s:expr, $($serde:ident)?;
        )*}
    )*} => {$(
        pub struct $name;

        impl Scope for $name {
            type Parent = $crate::parent_scope!($($parent)?);
            type Descriptor<T> = Self;
            const NAME: &'static str = stringify!($name);
        }

        impl<T> Descriptor<T> for TestScope {
            default fn name(_: &T) -> Option<std::borrow::Cow<str>> {
                None
            }

            default fn summary(_: &T) -> Option<std::borrow::Cow<str>> {
                None
            }

            default fn detail(_: &T) -> Option<std::borrow::Cow<str>> {
                None
            }
        }

        $(impl Descriptor<$t> for TestScope {
            fn name<'a>(_: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                Some(stringify!($id).into())
            }

            fn summary<'a>(_: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                Some($s.into())
            }

            fn detail<'a>(v: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                $crate::scope_serde!(v, $($serde)?)
            }
        })*

		#[allow(dead_code)]
		pub type Error = $crate::errorn::Error<$name>;
		#[allow(dead_code)]
		pub type Result<T, E = $crate::errorn::Error<$name>> = std::result::Result<T, E>;
    )*};
}
