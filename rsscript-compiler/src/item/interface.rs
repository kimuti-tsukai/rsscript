use syn::{
    braced, parse::Parse, punctuated::Punctuated, token::Brace, Ident, Type, TypeParamBound,
};

use crate::{
    generics::Generics,
    restrinction::Visibility,
    stmt::Block,
    token::IdentPeeker,
    Token,
};

use super::{FnArgs, TypeAnnotation};

pub struct ItemInterface {
    pub vis: Visibility,
    pub interface_token: Token![interface],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Option<(Token![extends], Punctuated<TypeParamBound, Token![+]>)>,
    pub brace_token: Brace,
    pub items: Vec<InterfaceItem>,
}

impl Parse for ItemInterface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let interface_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;
        let bounds = if input.ipeek::<Token![extends]>() {
            let extends_token = input.parse()?;
            let mut bounds = Punctuated::new();

            loop {
                let next = input.parse()?;
                bounds.push_value(next);

                if input.peek(Brace) {
                    break;
                }

                let punct = input.parse()?;
                bounds.push_punct(punct);

                if input.peek(Brace) {
                    break;
                }
            }

            Some((extends_token, bounds))
        } else {
            None
        };

        let content;
        let brace_token = braced!(content in input);
        let mut items = Vec::new();

        while !content.is_empty() {
            let item = input.parse()?;
            items.push(item);
        }

        Ok(Self {
            vis,
            interface_token,
            ident,
            generics,
            bounds,
            brace_token,
            items,
        })
    }
}

pub enum InterfaceItem {
    Function(InterfaceItemFn),
    Type(InterfaceItemType),
}

impl Parse for InterfaceItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.ipeek::<Token![function]>() {
            Ok(Self::Function(input.parse()?))
        } else if input.peek(Token![type]) {
            Ok(Self::Type(input.parse()?))
        } else {
            Err(input.error("Interface value have to be function or trait"))
        }
    }
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

impl Parse for InterfaceItemType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;
        let bounds = if input.ipeek::<Token![extends]>() {
            let extends_token = input.parse()?;
            let mut bounds = Punctuated::new();

            let check_next =
                || input.peek(Token![;]) || input.ipeek::<Token![function]>();

            loop {
                let next = input.parse()?;
                bounds.push_punct(next);

                if check_next() {
                    break;
                }

                let punct = input.parse()?;
                bounds.push_punct(punct);

                if check_next() {
                    break;
                }
            }

            Some((extends_token, bounds))
        } else {
            None
        };

        let default = if input.peek(Token![=]) {
            let eq_token = input.parse()?;
            let ty = input.parse()?;

            Some((eq_token, ty))
        } else {
            None
        };

        let semicolon_token = input.parse()?;

        Ok(Self {
            type_token,
            ident,
            generics,
            bounds,
            default,
            semicolon_token,
        })
    }
}
