use syn::{
    bracketed, parenthesized,
    parse::Parse,
    punctuated::Punctuated,
    token::{Bracket, Paren},
    AngleBracketedGenericArguments, BinOp, Ident, Member, Pat,
};

use crate::{
    enum_impl, generics::Generics, item::{FnArgs, TypeAnnotation}, stmt::{Block, Stmt}, Token
};

enum_impl!(
    pub enum Expr {
        Assign(ExprAssign),
        Array(ExprArray),
        Await(ExprAwait),
        Binary(ExprBinary),
        Field(ExprField),
        Infer(ExprInfer),
        Lit(ExprLit),
        MethodCall(ExprMethodCall),
        Function(ExprFunction),
        ArrowFunction(ExprArrowFunction),
        Call(ExprCall),
        If(ExprIf),
        Rust(RustCode),
        Paren(ExprParen),
    }
);

pub struct ExprAssign {
    pub left: Box<Expr>,
    pub eq_token: Token![=],
    pub right: Box<Expr>,
}

pub struct ExprArrayExpand {
    pub colons: Token![...],
    pub ident: Ident,
}

impl Parse for ExprArrayExpand {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            colons: input.parse()?,
            ident: input.parse()?,
        })
    }
}

impl Parse for ExprAssign {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            left: input.parse()?,
            eq_token: input.parse()?,
            right: input.parse()?,
        })
    }
}

pub struct ExprArray {
    pub bracket_token: Bracket,
    pub elems: Punctuated<Expr, Token![,]>,
}

impl Parse for ExprArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let mut elems = Punctuated::new();

        while !content.is_empty() {
            let first: Expr = content.parse()?;
            elems.push_value(first);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(ExprArray {
            bracket_token,
            elems,
        })
    }
}

pub struct ExprAwait {
    pub await_token: Token![await],
    pub base: Box<Expr>,
}

impl Parse for ExprAwait {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            await_token: input.parse()?,
            base: input.parse()?,
        })
    }
}

pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl Parse for ExprBinary {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            left: input.parse()?,
            op: input.parse()?,
            right: input.parse()?,
        })
    }
}

pub struct ExprField {
    pub base: Box<Expr>,
    pub dot_token: Token![.],
    pub member: Member,
}

impl Parse for ExprField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            base: input.parse()?,
            dot_token: input.parse()?,
            member: input.parse()?,
        })
    }
}

pub struct ExprInfer {
    pub underscore_token: Token![_],
}

impl Parse for ExprInfer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            underscore_token: input.parse()?,
        })
    }
}

pub struct ExprLit {
    pub lit: syn::Lit,
}

impl Parse for ExprLit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lit: input.parse()?,
        })
    }
}

pub struct ExprMethodCall {
    pub receiver: Box<Expr>,
    pub dot_token: Token![.],
    pub method: Ident,
    pub turbofish: Option<AngleBracketedGenericArguments>,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Token![,]>,
}

impl Parse for ExprMethodCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let receiver: Box<Expr> = input.parse()?;
        let dot_token: Token![.] = input.parse()?;
        let method: Ident = input.parse()?;
        let turbofish: Option<AngleBracketedGenericArguments> =
            if input.peek(Token![::]) && input.peek3(Token![<]) {
                Some(input.parse()?)
            } else {
                None
            };
        let content;
        let paren_token = parenthesized!(content in input);
        let mut args = Punctuated::new();

        while !content.is_empty() {
            let first: Expr = content.parse()?;
            args.push_value(first);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            args.push_punct(punct);
        }

        Ok(Self {
            receiver,
            dot_token,
            method,
            turbofish,
            paren_token,
            args,
        })
    }
}

pub struct ExprFunction {
    pub async_token: Option<Token![async]>,
    pub function_token: Token![function],
    pub generics: Generics,
    pub inputs: FnArgs,
    pub output: Option<TypeAnnotation>,
    pub body: Block,
}

impl Parse for ExprFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            async_token: input.parse()?,
            function_token: input.parse()?,
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

pub struct ArrowFunctionArgs {
    pub paren_token: Paren,
    pub inputs: Punctuated<Pat, Token![,]>,
}

impl Parse for ArrowFunctionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let mut inputs = Punctuated::new();

        while !content.is_empty() {
            let new = Pat::parse_multi(&content)?;
            inputs.push_value(new);

            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            inputs.push_punct(punct);
        }

        Ok(Self {
            paren_token,
            inputs,
        })
    }
}

pub struct ExprArrowFunction {
    pub inputs: ArrowFunctionArgs,
    pub output: Option<TypeAnnotation>,
    pub arrow_token: Token![=>],
    pub body: Box<Stmt>,
}

impl Parse for ExprArrowFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            inputs: input.parse()?,
            output: if input.peek(Token![:]) {
                Some(input.parse()?)
            } else {
                None
            },
            arrow_token: input.parse()?,
            body: input.parse()?,
        })
    }
}

pub struct ExprCall {
    pub func: Box<Expr>,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Token![,]>,
}

impl Parse for ExprCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let mut args = Punctuated::new();

        while !content.is_empty() {
            let new = input.parse()?;
            args.push_value(new);

            if content.is_empty() {
                break;
            }

            let punct = input.parse()?;
            args.push_punct(punct);
        }

        Ok(Self {
            func,
            paren_token,
            args,
        })
    }
}

pub struct ExprIf {
    pub cond: Box<Expr>,
    pub question_token: Token![?],
    pub then_branch: Box<Expr>,
    pub colon_token: Token![:],
    pub else_branch: Box<Expr>,
}

impl Parse for ExprIf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            cond: input.parse()?,
            question_token: input.parse()?,
            then_branch: input.parse()?,
            colon_token: input.parse()?,
            else_branch: input.parse()?,
        })
    }
}

pub struct RustCode {
    pub rust_token: Token![rust],
    pub body: syn::Block,
}

impl Parse for RustCode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            rust_token: input.parse()?,
            body: input.parse()?,
        })
    }
}

pub struct ExprParen {
    pub paren_token: Paren,
    pub expr: Box<Expr>,
}

impl Parse for ExprParen {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}
