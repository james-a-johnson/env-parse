#![feature(proc_macro_diagnostic)]

use std::env::var;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, Token, Type, Visibility, LitStr};

macro_rules! parse_type {
    ($ty:ty, $var:expr) => {{
        let val = $var.parse::<$ty>().unwrap();
        quote!{ #val }
    }}
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

    let env_var = match var(value.value()) {
        Ok(s) => s,
        Err(_) => {
            value.span().unwrap().error("Couldn't find variable in environment").emit();
            return TokenStream::new();
        },
    };
    let type_name = ty.span().source_text().unwrap();
    println!("{:?}", type_name);
    let const_value = match type_name.as_str() {
        "u8" => {
            parse_type!(u8, env_var)
        },
        "i8" => {
            parse_type!(i8, env_var)
        },
        "u16" => {
            parse_type!(u16, env_var)
        },
        "i16" => {
            parse_type!(i16, env_var)
        },
        "u32" => {
            parse_type!(u32, env_var)
        },
        "i32" => {
            parse_type!(i32, env_var)
        },
        "u64" => {
            parse_type!(u64, env_var)
        },
        "i64" => {
            parse_type!(i64, env_var)
        },
        "f32" => {
            parse_type!(f32, env_var)
        },
        "f64" => {
            parse_type!(f64, env_var)
        },
        _ => {
            ty.span().unwrap().error("Unsupported type").emit();
            return TokenStream::new();
        }
    };

    let expanded = quote! {
        #visibility const #name: #ty = #const_value;
    };

    TokenStream::from(expanded)
}