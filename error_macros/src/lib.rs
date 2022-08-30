mod ast;
mod emit;
use proc_macro::TokenStream;

use crate::emit::emit;

#[proc_macro]
pub fn define_scope_internal(input: TokenStream) -> TokenStream {
	let ast: ast::DefineScope = syn::parse(input).unwrap();
	emit(&ast).into()
}

#[proc_macro]
pub fn define_scope(input: TokenStream) -> TokenStream {
	let ast: ast::DefineScope = syn::parse(input).unwrap();
	if ast.name.to_string() == "Global" {
		panic!("The scope's name cannot be \"Global\".");
	}
	emit(&ast).into()
}
