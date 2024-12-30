use proc_macro2::TokenStream;

pub trait ToRustCode {
    fn to_rust_code(&self, tokens: &mut TokenStream);

    fn to_rust_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_rust_code(&mut tokens);
        tokens
    }

    fn into_rust_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        let mut tokens = TokenStream::new();
        self.to_rust_code(&mut tokens);
        tokens
    }
}

impl<T: ToRustCode> ToRustCode for Option<T> {
    fn to_rust_code(&self, tokens: &mut TokenStream) {
        if let Some(inner) = self {
            inner.to_rust_code(tokens);
        }
    }
}
