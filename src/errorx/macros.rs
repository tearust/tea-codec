#[doc(hidden)]
#[macro_export]
macro_rules! __private_parent_scope {
	($parent:ty) => {
		$parent
	};
	() => {
		$crate::errorx::Global
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_define_scope_string {
	($v:expr, Serde) => {
		serde_json::to_string($v).ok().map(Into::into)
	};
	($v:expr, Debug) => {
		Some(format!("{:?}", $v).into())
	};
	($v:expr, Display) => {
		Some(format!("{}", $v).into())
	};
	($v:expr, $e:expr) => {
		Some($e.into())
	};
	($v:expr, _) => {
		None
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_define_scope_impl {
	($scope:ident, $t:ty as $v:ident => $name:expr $(,$(@$summary_k:ident)?$($summary:expr)? $(,$(@$detail_k:ident)?$($detail:expr)?)?)?) => {
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl $crate::errorx::Descriptor<$t> for $scope {
            fn name<'a>(_: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                Some(stringify!($name).into())
            }

            $(fn summary<'a>($v: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                $crate::__private_define_scope_string!($v, $($summary_k)?$($summary)?)
            }

            $(fn detail<'a>($v: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                $crate::__private_define_scope_string!($v, $($detail_k)?$($detail)?)
            })?)?
        }
    };

	($scope:ident, $t:ty => $name:expr $(,$(@$summary_k:ident)?$($summary:expr)? $(,$(@$detail_k:ident)?$($detail:expr)?)?)?) => {
        #[allow(unused_variables)]
        #[allow(unused_parens)]
        impl $crate::errorx::Descriptor<$t> for $scope {
            fn name<'a>(_: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                Some(stringify!($name).into())
            }

            $(fn summary<'a>(v: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                $crate::__private_define_scope_string!(v, $($summary_k)?$($summary)?)
            }

            $(fn detail<'a>(v: &'a $t) -> Option<std::borrow::Cow<'a, str>> {
                $crate::__private_define_scope_string!(v, $($detail_k)?$($detail)?)
            })?)?
        }
    };
}

#[macro_export]
macro_rules! define_scope {
    {$(
        $scope:ident $(: $parent:ty)? {$(
            $t:ty $(as $v:ident)? => $name:ident $(, $(@$summary_k:ident)?$($summary:expr)? $(,$(@$detail_k:ident)?$($detail:expr)?)?)?;
        )*}
    )*} => {$(
        pub struct $scope;

        impl $crate::errorx::Scope for $scope {
            type Parent = $crate::__private_parent_scope!($($parent)?);
            type Descriptor<T> = Self;
            const NAME: &'static str = stringify!($scope);
        }

        impl<T> $crate::errorx::Descriptor<T> for $scope {
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

        $($crate::__private_define_scope_impl!($scope, $t $(as $v)? => $name $(,$(@$summary_k)?$($summary)? $(,$(@$detail_k)?$($detail)?)?)?);)*

		#[allow(dead_code)]
		pub type Error = $crate::errorx::Error<$scope>;
		#[allow(dead_code)]
		pub type Result<T, E = $crate::errorx::Error<$scope>> = std::result::Result<T, E>;
    )*};
}
