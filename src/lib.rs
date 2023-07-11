#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, Token, Type, Visibility, LitStr};

macro_rules! parse_type {
    ($ty:ty, $var:item) => {
        let val = $var.parse::<$ty>();
        quote!{ val }
    }
}

/// Parses the following syntax:
///
///     env_parse! {
///         $VISIBILITY const $NAME: $TYPE = "...";
///     }
/// 
/// For example:
///     env_parse! {
///         pub const BUILD_ID: u64 = "12345";
///     }
struct EnvParse {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    value: LitStr,
}

impl Parse for EnvParse {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![const]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: LitStr = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(EnvParse { visibility, name, ty, value })
    }
}

#[proc_macro]
pub fn env_parse(input: TokenStream) -> TokenStream {
    let EnvParse {
        visibility,
        name,
        ty,
        value,
    } = parse_macro_input!(input as EnvParse);

    let assert_parseable = quote_spanned!{ty.span()=>
        struct _AssertParseable where #ty: std::str::FromStr {}
    };

    let type_name = format!("{:?}", ty);
    let const_value = match type_name.as_str() {
        "u8" => parse_type!(u32, value.value()),
        "i32" => parse_type!(i32, value.value()),
    };

    let const_value = quote!{
        #value::parse::<#ty>().unwrap()
    };

    let expanded = quote! {
        #assert_parseable
        #visibility const #name: #ty = #const_value;
    };

    TokenStream::from(expanded)
}