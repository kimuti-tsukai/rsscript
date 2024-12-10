use syn::{
    braced, parenthesized,
    parse::Parse,
    token::{Brace, Paren},
    Ident, Pat,
};

use crate::{
    enum_impl, expr::ExprParen, item::DeclarationKeyword, token::IdentPeeker, Expr, Item, Token,
};

pub struct Stmt {
    pub stmt: StmtValue,
    pub punct: Option<Token![;]>,
}

impl Parse for Stmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            stmt: input.parse()?,
            punct: input.parse()?,
        })
    }
}

enum_impl! {
    pub enum StmtValue {
        Block(Block),
        If(StmtIf),
        Switch(StmtSwitch),
        For(StmtFor),
        While(StmtWhile),
        DoWhile(StmtDoWhile),
        Item(Box<Item>),
        Expr(Expr),
    }
}

pub struct StmtIf {
    pub if_token: Token![if],
    pub cond: ExprParen,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<(Token![else], Box<Stmt>)>,
}

impl Parse for StmtIf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let if_token = input.parse()?;
        let cond = input.parse()?;
        let then_branch = input.parse()?;

        let else_branch = if input.peek(Token![else]) {
            let else_token = input.parse()?;
            let branch = input.parse()?;

            Some((else_token, branch))
        } else {
            None
        };

        Ok(Self {
            if_token,
            cond,
            then_branch,
            else_branch,
        })
    }
}

pub struct CaseDecl {
    pub case_token: Token![case],
    pub pat: Pat,
    pub colon_token: Token![:],
}

impl Parse for CaseDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            case_token: input.parse()?,
            pat: Pat::parse_multi(input)?,
            colon_token: input.parse()?,
        })
    }
}

pub struct DefaultDecl {
    pub default_token: Token![default],
    pub colon_token: Token![:],
}

impl Parse for DefaultDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            default_token: input.parse()?,
            colon_token: input.parse()?,
        })
    }
}

enum_impl! {
    pub enum ArmDecl {
        Case(CaseDecl),
        Default(DefaultDecl),
    }
}

pub struct Arm {
    pub decl: ArmDecl,
    pub stmts: Vec<Stmt>,
}

impl Parse for Arm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let decl = input.parse()?;
        let mut stmts = Vec::new();

        while !input.is_empty() && !input.ipeek::<Token![case]>() && !input.peek(Token![default]) {
            let next = input.parse()?;
            stmts.push(next);
        }

        Ok(Self { decl, stmts })
    }
}

pub struct StmtSwitch {
    pub switch_token: Token![switch],
    pub expr: Expr,
    pub brace_token: Brace,
    pub arms: Vec<Arm>,
}

impl Parse for StmtSwitch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            switch_token: input.parse()?,
            expr: input.parse()?,
            brace_token: braced!(content in input),
            arms: {
                let mut arms = Vec::new();
                while !content.is_empty() {
                    let next = input.parse()?;
                    arms.push(next);
                }
                arms
            },
        })
    }
}

pub struct StmtWhile {
    pub while_token: Token![while],
    pub cond: ExprParen,
    pub body: Box<Stmt>,
    pub else_block: Option<(Token![else], Box<Stmt>)>,
}

impl Parse for StmtWhile {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            while_token: input.parse()?,
            cond: input.parse()?,
            body: input.parse()?,
            else_block: if input.peek(Token![else]) {
                let else_token = input.parse()?;
                let block = input.parse()?;
                Some((else_token, block))
            } else {
                None
            },
        })
    }
}

pub struct StmtDoWhile {
    pub do_token: Token![do],
    pub body: Box<Stmt>,
    pub while_token: Token![while],
    pub cond: ExprParen,
    pub else_block: Option<(Token![else], Box<Stmt>)>,
}

impl Parse for StmtDoWhile {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            do_token: input.parse()?,
            body: input.parse()?,
            while_token: input.parse()?,
            cond: input.parse()?,
            else_block: if input.peek(Token![else]) {
                let else_token = input.parse()?;
                let block = input.parse()?;
                Some((else_token, block))
            } else {
                None
            },
        })
    }
}

pub struct StmtFor {
    pub for_token: Token![for],
    pub header: ForArgs,
    pub body: Box<Stmt>,
    pub else_block: Option<(Token![else], Box<Stmt>)>,
}

impl Parse for StmtFor {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let for_token = input.parse()?;
        let header = input.parse()?;
        let body = input.parse()?;

        let else_block = if input.peek(Token![else]) {
            let else_token = input.parse()?;
            let block = input.parse()?;
            Some((else_token, block))
        } else {
            None
        };

        Ok(Self {
            for_token,
            header,
            body,
            else_block,
        })
    }
}

enum_impl! {
    pub enum ForArgs {
        In(ForInHeader),
        Header(ForHeader),
    }
}

pub struct ForInHeader {
    pub paren_token: Paren,
    pub keyword: DeclarationKeyword,
    pub pat: Pat,
    pub in_token: Token![in],
    pub iterable: Ident,
}

impl Parse for ForInHeader {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            keyword: content.parse()?,
            pat: Pat::parse_multi(input)?,
            in_token: input.parse()?,
            iterable: input.parse()?,
        })
    }
}

pub struct ForHeader {
    pub paren_token: Paren,
    pub init: Option<Box<StmtValue>>,
    pub semi_token: Token![;],
    pub cond: Option<Expr>,
    pub semi_token2: Token![;],
    pub after: Option<Box<StmtValue>>,
}

impl Parse for ForHeader {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            init: if !content.peek(Token![;]) {
                Some(content.parse()?)
            } else {
                None
            },
            semi_token: content.parse()?,
            cond: if !content.peek(Token![;]) {
                Some(content.parse()?)
            } else {
                None
            },
            semi_token2: content.parse()?,
            after: if !content.is_empty() {
                Some(content.parse()?)
            } else {
                None
            },
        })
    }
}

pub struct Block {
    pub brace_token: Brace,
    pub stmts: Vec<Stmt>,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = braced!(content in input);
        let mut stmts = Vec::new();

        while !content.is_empty() {
            let next: Stmt = content.parse()?;
            stmts.push(next);
        }

        Ok(Self { brace_token, stmts })
    }
}
