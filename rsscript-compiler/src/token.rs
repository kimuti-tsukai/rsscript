use proc_macro2::Span;
use syn::parse::Parse;

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
    fn ipeek(input: syn::buffer::Cursor) -> bool;
}

pub trait IdentPeeker {
    fn ipeek<T: IdentPeek>(&self) -> bool;

    fn ipeekn<T: IdentPeek>(&self, n: usize) -> bool;
}

impl IdentPeeker for syn::parse::ParseBuffer<'_> {
    fn ipeek<T: IdentPeek>(&self) -> bool {
        T::ipeek(self.cursor())
    }

    fn ipeekn<T: IdentPeek>(&self, n: usize) -> bool {
        let mut cursor = self.cursor();
        for _ in 0..n - 1 {
            if let Some((_, c)) = cursor.token_tree() {
                cursor = c;
            } else {
                return false;
            }
        }

        T::ipeek(cursor)
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
    [interface] => {
        $crate::token::Interface
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
                fn ipeek(input: syn::buffer::Cursor) -> bool {
                    parsing::peek_keyword(input, std::stringify!($keyword))
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
    case, Case;
    interface, Interface
);

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use quote::quote;

    #[test]
    fn ipeek_test() {
        let token_stream = quote! {
            rust impl case
        };

        #[derive(Debug)]
        struct GenParseStream;

        impl Parse for GenParseStream {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                assert!(input.ipeek::<Token![rust]>());
                assert!(input.peek2(Token![impl]));
                assert!(input.ipeekn::<Token![case]>(3));
                let _: Token![rust] = input.parse()?;
                let _: Token![impl] = input.parse()?;
                let _: Token![case] = input.parse()?;
                Ok(GenParseStream)
            }
        }

        syn::parse2::<GenParseStream>(token_stream).unwrap();
    }
}
