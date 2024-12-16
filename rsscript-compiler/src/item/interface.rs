use syn::{
    punctuated::Punctuated,
    token::{Brace, Paren},
    Ident, Type, TypeParamBound,
};

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
    pub paren_token: Paren,
    pub inputs: FnArgs,
    pub output: Option<TypeAnnotation>,
    pub default: Option<Block>,
    pub semicolon_token: Option<Token![;]>,
}

pub struct InterfaceItemType {
    pub type_token: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Option<(Token![extends], Punctuated<TypeParamBound, Token![+]>)>,
    pub default: Option<(Token![=], Type)>,
    pub semicolon_token: Option<Token![;]>,
}
