mod ast;
mod emit;
use proc_macro::TokenStream;

use crate::emit::{emit, emit_all};

#[proc_macro]
pub fn define_scope_internal(input: TokenStream) -> TokenStream {
	let ast: ast::DefineScope = syn::parse(input).unwrap();
	emit::<true>(&ast).into()
}

#[proc_macro]
pub fn define_scope(input: TokenStream) -> TokenStream {
	let ast: ast::DefineScopes = syn::parse(input).unwrap();
	emit_all(&ast.0).into()
}
