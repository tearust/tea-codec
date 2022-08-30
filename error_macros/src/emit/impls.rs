use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn impls(scope: &Ident) -> TokenStream {
	quote! {
		impl ::std::ops::Deref for #scope {
			type Target = str;
			fn deref(&self) -> &Self::Target {
				self.name_const()
			}
		}

		impl ::std::convert::AsRef<str> for #scope {
			fn as_ref(&self) -> &str {
				self
			}
		}

		impl ::std::borrow::Borrow<str> for #scope {
			fn borrow(&self) -> &str {
				self
			}
		}

		impl ::std::fmt::Display for #scope {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&**self, f)
			}
		}

		impl ::std::fmt::Debug for #scope {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&**self, f)
			}
		}

		impl ::std::cmp::PartialEq<&str> for #scope {
			fn eq(&self, other: &&str) -> bool {
				self.as_ref() == *other
			}
		}

		impl ::std::cmp::PartialEq<str> for #scope {
			fn eq(&self, other: &str) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<str> for &#scope {
			fn eq(&self, other: &str) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<&::std::string::String> for #scope {
			fn eq(&self, other: &&::std::string::String) -> bool {
				self.as_ref() == other.as_str()
			}
		}

		impl ::std::cmp::PartialEq<::std::string::String> for #scope {
			fn eq(&self, other: &::std::string::String) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<::std::string::String> for &#scope {
			fn eq(&self, other: &::std::string::String) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<&::std::borrow::Cow<'_, str>> for #scope {
			fn eq(&self, other: &&::std::borrow::Cow<'_, str>) -> bool {
				self.as_ref() == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<::std::borrow::Cow<'_, str>> for #scope {
			fn eq(&self, other: &::std::borrow::Cow<'_, str>) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<::std::borrow::Cow<'_, str>> for &#scope {
			fn eq(&self, other: &::std::borrow::Cow<'_, str>) -> bool {
				self.as_ref() == other
			}
		}

		impl ::std::cmp::PartialEq<#scope> for &str {
			fn eq(&self, other: &#scope) -> bool {
				*self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<#scope> for str {
			fn eq(&self, other: &#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<&#scope> for str {
			fn eq(&self, other: &&#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<#scope> for &::std::string::String {
			fn eq(&self, other: &#scope) -> bool {
				self.as_str() == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<#scope> for ::std::string::String {
			fn eq(&self, other: &#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<&#scope> for ::std::string::String {
			fn eq(&self, other: &&#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<#scope> for &::std::borrow::Cow<'_, str> {
			fn eq(&self, other: &#scope) -> bool {
				self.as_ref() == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<#scope> for ::std::borrow::Cow<'_, str> {
			fn eq(&self, other: &#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl ::std::cmp::PartialEq<&#scope> for ::std::borrow::Cow<'_, str> {
			fn eq(&self, other: &&#scope) -> bool {
				self == other.as_ref()
			}
		}

		impl std::convert::From<#scope> for ::std::borrow::Cow<'static, str> {
			fn from(s: #scope) -> Self {
				s.name_const().into()
			}
		}

		impl std::convert::From<#scope> for &'static str {
			fn from(s: #scope) -> Self {
				s.name_const()
			}
		}

		impl std::convert::From<#scope> for String {
			fn from(s: #scope) -> Self {
				::std::string::ToString::to_string(s.name_const())
			}
		}
	}
}
