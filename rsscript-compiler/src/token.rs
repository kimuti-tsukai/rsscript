use proc_macro2::Span;
use syn::{ext::IdentExt, parse::Parse, Ident};

use crate::parsing;

pub trait TypePeek: Parse {
    fn cpeek(input: syn::parse::ParseStream) -> bool {
        input.fork().parse::<Self>().is_ok()
    }
}

impl<T: Parse> TypePeek for T {}

pub trait TypePeeker {
    fn cpeek<T: TypePeek>(&self) -> bool;
}

impl TypePeeker for syn::parse::ParseBuffer<'_> {
    fn cpeek<T: TypePeek>(&self) -> bool {
        T::cpeek(self)
    }
}

pub trait IdentPeek {
    fn ipeek(input: syn::parse::ParseStream) -> bool;
}

pub trait IdentPeeker {
    fn ipeek<T: IdentPeek>(&self) -> bool;
}

impl IdentPeeker for syn::parse::ParseBuffer<'_> {
    fn ipeek<T: IdentPeek>(&self) -> bool {
        T::ipeek(self)
    }
}

#[macro_export]
macro_rules! Token {
    [rust] => {
        $crate::token::Rust
    };
    [function] => {
        $crate::token::Function
    };
    [export] => {
        $crate::token::Export
    };
    [private] => {
        $crate::token::Private
    };
    [public] => {
        $crate::token::Public
    };
    [protected] => {
        $crate::token::Protected
    };
    [class] => {
        $crate::token::Class
    };
    [extends] => {
        $crate::token::Extends
    };
    [switch] => {
        $crate::token::Switch
    };
    [case] => {
        $crate::token::Case
    };
    [$token:tt] => {
        syn::Token![$token]
    };
}

#[macro_export(local_inner_macros)]
macro_rules! define_keyword {
    ($($keyword:ident, $struct:ident);+) => {
        define_keyword!(@inner [$] $($keyword, $struct);+);
    };
    (@inner [$dollar:tt] $($keyword:ident, $struct:ident);+) => {
        $(
            pub struct $struct {
                pub span: Span,
            }

            impl Parse for $struct {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    Ok(Self {
                        span: parsing::keyword(input, std::stringify!($keyword))?,
                    })
                }
            }

            impl IdentPeek for $struct {
                fn ipeek(input: syn::parse::ParseStream) -> bool {
                    input.peek(Ident::peek_any) && {
                        let fork = input.fork();
                        let ident = Ident::parse_any(&fork).unwrap();
                        ident == std::stringify!($keyword)
                    }
                }
            }
        )+
    };
}

define_keyword!(
    rust, Rust;
    function, Function;
    export, Export;
    private, Private;
    public, Public;
    protected, Protected;
    class, Class;
    extends, Extends;
    switch, Switch;
    case, Case
);
