use class::ItemClass;
use interface::ItemInterface;
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, token::Paren, FnArg, Ident, Pat, Type,
};

use crate::{
    parser::{
        expr::Expr, generics::Generics, restrinction::Visibility, stmt::Block, token::IdentPeeker,
    },
    Token,
};

pub mod class;

pub mod interface;

pub enum Item {
    Local(Local),
    Function(ItemFunction),
    Class(ItemClass),
    Interface(ItemInterface),
    TypeAlias(ItemTypeAlias),
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![const]) || input.peek(Token![let]) {
            Ok(Self::Local(input.parse()?))
        } else if input.ipeek::<Token![function]>()
            || input.peek(Token![async]) && input.ipeekn::<Token![function]>(2)
            || input.ipeek::<Token![export]>() && input.ipeekn::<Token![function]>(2)
            || input.ipeek::<Token![export]>()
                && input.peek2(Token![async])
                && input.ipeekn::<Token![function]>(3)
        {
            Ok(Self::Function(input.parse()?))
        } else if input.ipeek::<Token![class]>()
            || input.ipeek::<Token![export]>() && input.ipeekn::<Token![class]>(2)
        {
            Ok(Self::Class(input.parse()?))
        } else if input.ipeek::<Token![interface]>()
            || input.ipeek::<Token![export]>() && input.ipeekn::<Token![interface]>(2)
        {
            Ok(Self::Interface(input.parse()?))
        } else if input.peek(Token![type])
            || input.ipeek::<Token![export]>() && input.peek2(Token![type])
        {
            Ok(Self::TypeAlias(input.parse()?))
        } else {
            Err(input.error("Not excepted token"))
        }
    }
}

pub enum DeclarationKeyword {
    Let,
    Const,
}

impl Parse for DeclarationKeyword {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![let]) {
            let _: Token![let] = input.parse()?;
            Ok(Self::Let)
        } else if input.peek(Token![const]) {
            let _: Token![const] = input.parse()?;
            Ok(Self::Const)
        } else {
            Err(input.parse::<Token![let]>().unwrap_err())
        }
    }
}

pub struct TypeAnnotation {
    pub colon_token: Token![:],
    pub ty: syn::Type,
}

impl Parse for TypeAnnotation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

pub struct Local {
    pub let_token: DeclarationKeyword,
    pub pat: Pat,
    pub init: Option<InitVar>,
}

impl Parse for Local {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            let_token: input.parse()?,
            pat: Pat::parse_multi(input)?,
            init: input.parse::<InitVar>().ok(),
        })
    }
}

pub struct InitVar {
    pub eq_token: Token![=],
    pub expr: Box<Expr>,
}

impl Parse for InitVar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            eq_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

pub struct FnArgs {
    pub paren_token: Paren,
    pub args: Punctuated<FnArg, Token![,]>,
}

impl Parse for FnArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let mut args = Punctuated::new();

        while !content.is_empty() {
            let next = content.parse()?;
            args.push_value(next);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            args.push_punct(punct);
        }

        Ok(Self { paren_token, args })
    }
}

pub struct ItemFunction {
    pub export_token: Visibility,
    pub async_token: Option<Token![async]>,
    pub function_token: Token![function],
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: FnArgs,
    pub output: Option<TypeAnnotation>,
    pub body: Block,
}

impl Parse for ItemFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            export_token: input.parse()?,
            async_token: input.parse()?,
            function_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            inputs: input.parse()?,
            output: if input.peek(Token![:]) {
                Some(input.parse()?)
            } else {
                None
            },
            body: input.parse()?,
        })
    }
}

pub struct ItemTypeAlias {
    pub vis: Visibility,
    pub type_token: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub eq_token: Token![=],
    pub alias: Type,
}

impl Parse for ItemTypeAlias {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            vis: input.parse()?,
            type_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            eq_token: input.parse()?,
            alias: input.parse()?,
        })
    }
}
