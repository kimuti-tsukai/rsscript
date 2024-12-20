use syn::{
    braced, parenthesized,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Paren},
    Ident, Path, TypeParamBound,
};

use crate::{
    enum_impl,
    expr::object::{ObjectIdent, ObjectMethod, ObjectStmt},
    generics::Generics,
    item::InitVar,
    restrinction::{ClassVisibility, Visibility},
    token::IdentPeeker,
    Token,
};

use super::ItemTypeAlias;

pub struct ClassField {
    pub ident: ObjectIdent,
    pub init: Option<InitVar>,
}

impl Parse for ClassField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            init: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

pub struct ClassMethod {
    pub static_token: Option<Token![static]>,
    pub method: ObjectMethod,
}

impl Parse for ClassMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            static_token: input.parse()?,
            method: input.parse()?,
        })
    }
}

pub struct ClassTypeAlias {
    pub alias: ItemTypeAlias,
}

impl Parse for ClassTypeAlias {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            alias: input.parse()?,
        })
    }
}

pub struct ImplRestriction {
    pub impl_token: Token![impl],
    pub paren_token: Paren,
    pub restrict: Path,
}

impl Parse for ImplRestriction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            impl_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            restrict: content.parse()?,
        })
    }
}

enum_impl! {
    pub enum ClassStmtValue {
        Field(ClassField),
        Method(ClassMethod),
        TypeAlias(ClassTypeAlias),
    }
}

pub struct ClassStmt {
    pub restrict: Option<ImplRestriction>,
    pub vis: ClassVisibility,
    pub stmt: ObjectStmt,
    pub semi_colon_token: Option<Token![;]>,
}

impl Parse for ClassStmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            restrict: if input.peek(Token![impl]) {
                Some(input.parse()?)
            } else {
                None
            },
            vis: input.parse()?,
            stmt: input.parse()?,
            semi_colon_token: input.parse()?,
        })
    }
}

pub struct ItemClass {
    pub vis: Visibility,
    pub class_token: Token![class],
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Option<(Token![extends], Punctuated<TypeParamBound, Token![,]>)>,
    pub brace: Brace,
    pub stmts: Punctuated<ObjectStmt, Option<Token![;]>>,
}

impl Parse for ItemClass {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let class_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;

        let bounds = if input.ipeek::<Token![extends]>() {
            let extends_token = input.parse()?;
            let mut bounds = Punctuated::new();

            while !input.peek(Brace) {
                let next = input.parse()?;
                bounds.push_value(next);
                if input.peek(Brace) {
                    break;
                }
                let punct = input.parse()?;
                bounds.push_punct(punct);
            }

            Some((extends_token, bounds))
        } else {
            None
        };

        let content;
        let brace = braced!(content in input);
        let mut stmts = Punctuated::new();

        while !content.is_empty() {
            let next = input.parse()?;
            stmts.push(next);

            if content.is_empty() {
                break;
            }

            let punct = if content.peek(Token![;]) {
                let punct = input.parse()?;
                Some(punct)
            } else {
                None
            };

            stmts.push_punct(punct);
        }

        Ok(Self {
            vis,
            class_token,
            ident,
            generics,
            bounds,
            brace,
            stmts,
        })
    }
}
