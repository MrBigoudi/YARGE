use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(FileResource)]
pub fn derive_file_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl ::yarge::FileResource for #name {}
    }
    .into()
}

#[proc_macro_derive(RonFileResource)]
pub fn derive_ron_file_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl ::yarge::FileResource for #name {}
        impl ::yarge::RonFileResource for #name {}
    }
    .into()
}
