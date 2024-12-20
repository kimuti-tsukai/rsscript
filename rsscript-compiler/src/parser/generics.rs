use syn::{
    parse::Parse, punctuated::Punctuated, ConstParam, Ident, Lifetime, Type, TypeParamBound,
};

use crate::{parser::token::IdentPeeker, Token};

pub struct Generics {
    pub lt_token: Token![<],
    pub params: Punctuated<GenericsParam, Token![,]>,
    pub gt_token: Token![>],
}

impl Parse for Generics {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lt_token = input.parse()?;
        let mut params = Punctuated::new();

        while !input.peek(Token![>]) {
            let next = input.parse()?;
            params.push_value(next);

            if input.peek(Token![>]) {
                break;
            }

            let punct = input.parse()?;
            params.push_punct(punct);
        }

        let gt_token = input.parse()?;

        Ok(Self {
            lt_token,
            params,
            gt_token,
        })
    }
}

pub enum GenericsParam {
    Type(TypeParam),
    Lifetime(LifetimeParam),
    Const(ConstParam),
}

impl Parse for GenericsParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            Ok(Self::Type(input.parse()?))
        } else if input.peek(Lifetime) {
            Ok(Self::Lifetime(input.parse()?))
        } else if input.peek(Token![const]) {
            Ok(Self::Const(input.parse()?))
        } else {
            Err(input.error("Not excepted token"))
        }
    }
}

pub struct TypeParam {
    pub ident: Ident,
    pub extends_token: Option<Token![extends]>,
    pub bounds: Punctuated<TypeParamBound, Token![+]>,
    pub eq_token: Option<Token![=]>,
    pub default: Option<Type>,
}

impl Parse for TypeParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let extends_token = if input.ipeek::<Token![extends]>() {
            Some(input.parse()?)
        } else {
            None
        };
        let mut bounds = Punctuated::new();

        while !input.peek(Token![,]) && !input.peek(Token![>]) && !input.peek(Token![=]) {
            let next = input.parse()?;
            bounds.push_value(next);
            if input.peek(Token![,]) || input.peek(Token![>]) || input.peek(Token![=]) {
                break;
            }

            let punct = input.parse()?;
            bounds.push_punct(punct);
        }

        let (eq_token, default) = if input.peek(Token![=]) {
            (Some(input.parse()?), Some(input.parse()?))
        } else {
            (None, None)
        };

        Ok(Self {
            ident,
            extends_token,
            bounds,
            eq_token,
            default,
        })
    }
}

pub struct LifetimeParam {
    pub lifetime: Lifetime,
    pub extends_token: Option<Token![extends]>,
    pub bounds: Punctuated<Lifetime, Token![+]>,
}

impl Parse for LifetimeParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lifetime = input.parse()?;
        let extends_token = if input.ipeek::<Token![extends]>() {
            Some(input.parse()?)
        } else {
            None
        };
        let mut bounds = Punctuated::new();

        while !input.peek(Token![,]) && !input.peek(Token![>]) {
            let next = input.parse()?;
            bounds.push_value(next);
            if input.peek(Token![,]) || input.peek(Token![>]) {
                break;
            }

            let punct = input.parse()?;
            bounds.push_punct(punct);
        }

        Ok(Self {
            lifetime,
            extends_token,
            bounds,
        })
    }
}
