use syn::{
	braced, bracketed,
	parse::{Parse, ParseStream},
	token, Expr, Ident, Result, Token, Type,
};

pub struct DefineScopes(pub Vec<DefineScope>);

impl Parse for DefineScopes {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut result = Vec::new();

		while !input.is_empty() {
			result.push(input.parse()?);
		}

		Ok(Self(result))
	}
}

pub struct DefineScope {
	pub name: Ident,
	pub parents: Vec<(Type, bool)>,
	pub definitions: Vec<Definition>,
}

impl Parse for DefineScope {
	fn parse(input: ParseStream) -> Result<Self> {
		let name = input.parse()?;
		let mut parents = Vec::new();
		if input.parse::<Option<Token![:]>>()?.is_some() {
			loop {
				let is_pub = input.parse::<Option<Token![pub]>>()?.is_some();
				let r#type = input.parse()?;
				parents.push((r#type, is_pub));

				if input.parse::<Option<Token![,]>>()?.is_none() {
					break;
				}
			}
		}
		let body;
		braced!(body in input);
		let mut definitions = Vec::new();
		while !body.is_empty() {
			definitions.push(body.parse()?);
		}
		Ok(Self {
			name,
			parents,
			definitions,
		})
	}
}

pub enum Definition {
	Abstract(AbstractDefinition),
	Typed(Box<TypedDefinition>),
}

impl Parse for Definition {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek2(Token![;]) {
			Self::Abstract(input.parse()?)
		} else {
			Self::Typed(input.parse()?)
		})
	}
}

pub struct AbstractDefinition(pub Ident);

impl Parse for AbstractDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		let id = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self(id))
	}
}

pub struct TypedDefinition {
	pub r#type: Type,
	pub value: Option<Ident>,
	pub name: Name,
	pub summary: Option<StringExpr>,
	pub detail: Option<StringExpr>,
	pub inner: Option<Inner>,
}

impl Parse for TypedDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		let r#type = input.parse()?;
		let value = if input.parse::<Option<Token![as]>>()?.is_some() {
			input.parse::<Ignorable<_>>()?.0
		} else {
			None
		};
		input.parse::<Token![=>]>()?;
		let name = input.parse()?;
		let summary;
		let detail;
		let inner;
		if input.parse::<Option<Token![,]>>()?.is_some() {
			summary = input.parse::<Ignorable<_>>()?.0;
			if input.parse::<Option<Token![,]>>()?.is_some() {
				detail = input.parse::<Ignorable<_>>()?.0;
				if input.parse::<Option<Token![,]>>()?.is_some() {
					inner = input.parse::<Ignorable<_>>()?.0
				} else {
					inner = None;
				}
			} else {
				detail = None;
				inner = None;
			}
		} else {
			summary = None;
			detail = None;
			inner = None;
		}
		input.parse::<Token![;]>()?;
		Ok(Self {
			r#type,
			value,
			name,
			summary,
			detail,
			inner,
		})
	}
}

pub enum Name {
	Define(Ident),
	Use(Box<Expr>),
}

impl Parse for Name {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.parse::<Option<Token![@]>>()?.is_some() {
			Self::Use(input.parse()?)
		} else {
			Self::Define(input.parse()?)
		})
	}
}

struct Ignorable<T>(Option<T>);

impl<T> Parse for Ignorable<T>
where
	T: Parse,
{
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.parse::<Option<Token![_]>>()?.is_some() {
			Self(None)
		} else {
			Self(Some(input.parse()?))
		})
	}
}

pub enum StringExpr {
	Expr(Box<Expr>),
	Use(StringAuto),
}

impl Parse for StringExpr {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.parse::<Option<Token![@]>>()?.is_some() {
			Self::Use(input.parse()?)
		} else {
			Self::Expr(input.parse()?)
		})
	}
}

pub enum StringAuto {
	Display,
	Debug,
	Serde,
}

mod string_auto_kw {
	use syn::custom_keyword;
	custom_keyword!(Display);
	custom_keyword!(Debug);
	custom_keyword!(Serde);
}

impl Parse for StringAuto {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(
			if input.parse::<Option<string_auto_kw::Display>>()?.is_some() {
				Self::Display
			} else if input.parse::<Option<string_auto_kw::Debug>>()?.is_some() {
				Self::Debug
			} else {
				input.parse::<string_auto_kw::Serde>()?;
				Self::Serde
			},
		)
	}
}

pub enum Inner {
	Values(Vec<Expr>),
	Raw(Box<Expr>),
}

impl Parse for Inner {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(if input.peek(token::Bracket) {
			let values;
			bracketed!(values in input);
			let values = values
				.parse_terminated::<_, Token![,]>(Expr::parse)?
				.into_iter()
				.collect();
			Self::Values(values)
		} else {
			Self::Raw(input.parse()?)
		})
	}
}
