use syn::parse::Parse;

use crate::{token::IdentPeeker, Token};

pub struct Visibility {
    pub export_token: Option<Token![export]>,
}

impl Parse for Visibility {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            export_token: if input.ipeek::<Token![export]>() {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

pub enum ClassVisibility {
    Private(Token![private]),
    Public(Token![public]),
    Protected(Token![protected]),
    Inherited,
}

impl Parse for ClassVisibility {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.ipeek::<Token![private]>() {
            Ok(Self::Private(input.parse()?))
        } else if input.ipeek::<Token![public]>() {
            Ok(Self::Public(input.parse()?))
        } else if input.ipeek::<Token![protected]>() {
            Ok(Self::Protected(input.parse()?))
        } else {
            Ok(Self::Inherited)
        }
    }
}
