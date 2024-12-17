use syn::{parse::Parse, punctuated::Punctuated, token::Brace, Ident, Type, TypeParamBound};

use crate::{generics::Generics, restrinction::Visibility, stmt::Block, Token};

use super::{FnArgs, TypeAnnotation};

pub struct Interface {
    pub vis: Visibility,
    pub interface_token: Token![interface],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Option<(Token![extends], Punctuated<TypeParamBound, Token![+]>)>,
    pub brace: Brace,
    pub items: Vec<InterfaceItem>,
}

pub enum InterfaceItem {
    Fn(InterfaceItemFn),
    Type(InterfaceItemType),
}

pub struct InterfaceItemFn {
    pub function_token: Token![function],
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: FnArgs,
    pub output: Option<TypeAnnotation>,
    pub default: Option<Block>,
    pub semicolon_token: Option<Token![;]>,
}

impl Parse for InterfaceItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;
        let inputs = input.parse()?;
        let output = if input.peek(Token![=]) {
            Some(input.parse()?)
        } else {
            None
        };
        let (default, semicolon_token) = if input.peek(Brace) {
            (Some(input.parse()?), None)
        } else if input.peek(Token![;]) {
            (None, Some(input.parse()?))
        } else {
            (None, None)
        };

        Ok(Self {
            function_token,
            ident,
            generics,
            inputs,
            output,
            default,
            semicolon_token,
        })
    }
}

pub struct InterfaceItemType {
    pub type_token: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Option<(Token![extends], Punctuated<TypeParamBound, Token![+]>)>,
    pub default: Option<(Token![=], Type)>,
    pub semicolon_token: Option<Token![;]>,
}
