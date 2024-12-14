use syn::{parse::Parse, punctuated::Punctuated, Ident, Lifetime, Type, TypeParamBound};

use crate::{token::IdentPeeker, Token};

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
