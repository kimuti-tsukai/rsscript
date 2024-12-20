use syn::{braced, parse::Parse, punctuated::Punctuated, token::Brace, Ident};

use crate::{
    enum_impl,
    parser::{
        generics::Generics,
        item::{FnArgs, TypeAnnotation},
        stmt::Block,
        Expr,
    },
    Token,
};

pub struct ObjectIdent {
    pub sharp_token: Option<Token![#]>,
    pub ident: Ident,
}

impl Parse for ObjectIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            sharp_token: if input.peek(Token![#]) {
                Some(input.parse()?)
            } else {
                None
            },
            ident: input.parse()?,
        })
    }
}

pub struct ObjectMethod {
    pub ident: ObjectIdent,
    pub generics: Generics,
    pub inputs: FnArgs,
    pub outputs: Option<TypeAnnotation>,
    pub body: Block,
}

impl Parse for ObjectMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            generics: input.parse()?,
            inputs: input.parse()?,
            outputs: if input.peek(Token![:]) {
                Some(input.parse()?)
            } else {
                None
            },
            body: input.parse()?,
        })
    }
}

pub struct ObjectField {
    pub ident: ObjectIdent,
    pub init: Option<(Token![:], Expr)>,
}

impl Parse for ObjectField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            init: if input.peek(Token![:]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

enum_impl! {
    pub enum ObjectStmt {
        ObjectMethod(ObjectMethod),
        Field(ObjectField),
    }
}

pub struct ExprObject {
    pub brace_token: Brace,
    pub fields: Punctuated<ObjectStmt, Token![,]>,
}

impl Parse for ExprObject {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            fields: Punctuated::parse_terminated(&content)?,
        })
    }
}
