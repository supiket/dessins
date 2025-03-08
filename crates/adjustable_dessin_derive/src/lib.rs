use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DefaultAdjustableDessin)]
pub fn derive_controllable_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl AdjustableDessin for #name { }
    };

    TokenStream::from(expanded)
}
