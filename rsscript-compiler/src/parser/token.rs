use proc_macro2::Span;
use syn::parse::Parse;

use crate::parser::parsing;

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
        $crate::parser::token::Rust
    };
    [function] => {
        $crate::parser::token::Function
    };
    [export] => {
        $crate::parser::token::Export
    };
    [private] => {
        $crate::parser::token::Private
    };
    [public] => {
        $crate::parser::token::Public
    };
    [protected] => {
        $crate::parser::token::Protected
    };
    [class] => {
        $crate::parser::token::Class
    };
    [extends] => {
        $crate::parser::token::Extends
    };
    [switch] => {
        $crate::parser::token::Switch
    };
    [case] => {
        $crate::parser::token::Case
    };
    [interface] => {
        $crate::parser::token::Interface
    };
    [import] => {
        $crate::parser::token::Import
    };
    [from] => {
        $crate::parser::token::From
    };
    [$token:tt] => {
        syn::Token![$token]
    };
}

#[macro_export(local_inner_macros)]
macro_rules! define_keyword {
    ($($keyword:tt, $struct:ident);+ $(;)?) => {
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
    interface, Interface;
    import, Import;
    from, From;
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
