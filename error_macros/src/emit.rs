use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::ast::{DefineScope, Definition, Name, StringAuto, StringExpr, TypedDefinition};

pub fn emit(source: &DefineScope) -> TokenStream {
	let DefineScope {
		name,
		parent,
		definitions,
	} = source;

	let parent = if let Some(parent) = parent {
		quote! { #parent }
	} else {
		quote! { ::tea_codec::errorx::Global }
	};

	let name_string = name.to_string();

	let names = emit_names(name, definitions);

	let definitions = definitions
		.iter()
		.filter_map(|x| match x {
			Definition::Abstract(_) => None,
			Definition::Typed(def) => Some(def),
		})
		.map(|def| emit_definition(name, def));

	quote! {
		pub struct #name;

		impl ::tea_codec::errorx::Scope for #name {
			type Parent = #parent;
			type Descriptor<T> = Self;
			const NAME: &'static str = #name_string;
		}

		#names

		impl<T> ::tea_codec::errorx::Descriptor<T> for #name {
			default fn name(_: &T) -> Option<::std::borrow::Cow<str>> {
				None
			}

			default fn summary(_: &T) -> Option<::std::borrow::Cow<str>> {
				None
			}

			default fn detail(_: &T) -> Option<::std::borrow::Cow<str>> {
				None
			}

			default fn inner(_: &T) -> Option<::tea_codec::errorx::SmallVec<[&::tea_codec::errorx::Error; 1]>> {
				None
			}

			default fn type_id(_: &T) -> Option<::std::any::TypeId> {
				None
			}
		}

		#(#definitions)*

		pub type Error<S = #name> = ::tea_codec::errorx::Error<S>;
		pub type Result<T, E = Error> = std::result::Result<T, E>;
	}
}

fn emit_names<'a>(
	scope: &'a Ident,
	def: impl IntoIterator<Item = &'a Definition> + 'a,
) -> TokenStream {
	let names = def
		.into_iter()
		.filter_map(|def| match def {
			Definition::Abstract(def) => Some(&def.0),
			Definition::Typed(def) => match &def.name {
				Name::Define(id) => Some(id),
				Name::Use(_) => None,
			},
		})
		.unique()
		.map(|id| {
			let value = id.to_string();
			let back_id = Ident::new(format!("_{id}").as_str() ,id.span());
			quote! {
				static #back_id: ::tea_codec::errorx::ErrorName<super::#scope> = ::tea_codec::errorx::ErrorName::new(#value);
				pub static #id: &'static ::tea_codec::errorx::ErrorName<super::#scope> = &#back_id;
			}
		});

	let mod_name = convert_scope_mod_case(scope);

	quote! {
		#[allow(non_upper_case_globals)]
		pub mod #mod_name {
			#(#names)*
		}
	}
}

fn convert_scope_mod_case(id: &Ident) -> Ident {
	Ident::new(id.to_string().to_case(Case::Snake).as_str(), id.span())
}

fn emit_definition(scope: &Ident, def: &TypedDefinition) -> TokenStream {
	let TypedDefinition {
		r#type,
		value,
		name,
		summary,
		detail,
		inner,
	} = def;

	let value = value
		.as_ref()
		.map(|v| quote! {#v})
		.unwrap_or_else(|| quote! {__value__});

	let name = match name {
		Name::Define(name) => {
			let scope = convert_scope_mod_case(scope);
			quote! { (&*#scope::#name).into() }
		}
		Name::Use(s) => quote! { (#s).into() },
	};

	let summary = summary
		.as_ref()
		.map(|summary| {
			let summary = emit_string_expr(&value, summary);
			quote! {
				fn summary<'a>(#value: &'a #r#type) -> Option<std::borrow::Cow<'a, str>> {
					#summary
				}
			}
		})
		.unwrap_or_default();

	let detail = detail
		.as_ref()
		.map(|detail| {
			let detail = emit_string_expr(&value, detail);
			quote! {
				fn detail<'a>(#value: &'a #r#type) -> Option<std::borrow::Cow<'a, str>> {
					#detail
				}
			}
		})
		.unwrap_or_default();

	let inner = inner
		.as_ref()
		.map(|inner| {
			let inner = match inner {
				crate::ast::Inner::Values(values) => {
					let len = values.len();
					let values = values
						.iter()
						.map(|e| quote! { inner.push((#e).into()); });
					quote! {{
						let mut inner = ::tea_codec::errorx::SmallVec::<[&::tea_codec::errorx::Error; 1]>::new();
						inner.reserve_exact(#len);
						#(#values)*
						inner
					}}
				}
				crate::ast::Inner::Raw(value) => quote! { (#value).into() },
			};

			quote! {
				fn inner<'a>(#value: &'a #r#type) -> Option<::tea_codec::errorx::SmallVec<[&'a ::tea_codec::errorx::Error; 1]>> {
					Some(#inner)
				}
			}
		})
		.unwrap_or_default();

	quote! {
		#[allow(unused_variables)]
		#[allow(unused_parens)]
		impl ::tea_codec::errorx::Descriptor<#r#type> for #scope {
			fn name<'a>(_: &'a #r#type) -> Option<::std::borrow::Cow<'a, str>> {
				Some(#name)
			}

			#summary
			#detail
			#inner

			fn type_id<'a>(_: &'a #r#type) -> Option<::std::any::TypeId> {
				Some(::std::any::TypeId::of::<#r#type>())
			}
		}
	}
}

fn emit_string_expr(value: &TokenStream, expr: &StringExpr) -> TokenStream {
	match expr {
		StringExpr::Expr(expr) => quote! { Some((#expr).into()) },
		StringExpr::Use(auto) => match auto {
			StringAuto::Display => quote! { Some(format!("{}", #value).into()) },
			StringAuto::Debug => quote! { Some(format!("{:?}", #value).into()) },
			StringAuto::Serde => quote! { ::serde_json::to_string(#value).ok().map(Into::into) },
		},
	}
}
