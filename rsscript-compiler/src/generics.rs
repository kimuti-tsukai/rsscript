use syn::{parse::Parse, punctuated::Punctuated, Ident, Lifetime, Type, TypeParamBound};

use crate::Token;

pub struct TypeParam {
    pub ident: Ident,
    pub extends_token: Option<Token![extends]>,
    pub bounds: Punctuated<TypeParamBound, Token![+]>,
    pub eq_token: Option<Token![=]>,
    pub default: Option<Type>,
}

pub struct LifetimeParam {
    pub lifetime: Lifetime,
    pub extends_token: Option<Token![extends]>,
    pub bounds: Punctuated<Lifetime, Token![+]>,
}
