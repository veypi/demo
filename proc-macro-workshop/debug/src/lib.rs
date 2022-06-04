use proc_macro::TokenStream;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    println!("123\n");
    println!("{}", input);
    let _ = input;

    unimplemented!()
}
