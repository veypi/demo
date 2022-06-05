use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", input);
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    eprintln!("{:#?}", st);
    TokenStream::new()
}
