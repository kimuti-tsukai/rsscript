use syn::buffer::Cursor;

pub fn keyword(input: syn::parse::ParseStream, token: &str) -> syn::Result<proc_macro2::Span> {
    input.step(|cursor| {
        if let Some((ident, rest)) = cursor.ident() {
            if ident == token {
                return Ok((ident.span(), rest));
            }
        }
        Err(cursor.error(format!("expected `{}`", token)))
    })
}

pub fn peek_keyword(cursor: Cursor, token: &str) -> bool {
    if let Some((ident, _rest)) = cursor.ident() {
        ident == token
    } else {
        false
    }
}

pub trait IsNext {
    fn is_next(input: syn::parse::ParseStream) -> bool;
}

#[macro_export]
macro_rules! enum_impl {
    (
        $vis:vis enum $enum_name:ident {
            $($variant:ident($type:ty),)+
        }
    ) => {
        $vis enum $enum_name {
            $(
                $variant($type)
            ),+
        }

        impl Parse for $enum_name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                use syn::parse::discouraged::Speculative;

                $(
                    let fork = input.fork();
                    if let Ok(result) = fork.parse::<$type>() {
                        input.advance_to(&fork);
                        return Ok(Self::$variant(result));
                    }
                )+

                return Err(input.error(format!("No matching variant for {}", stringify!($enum_name))))
            }
        }
    };
}
