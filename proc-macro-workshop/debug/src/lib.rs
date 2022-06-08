//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-08 14:55
// Distributed under terms of the MIT license.
// 01
// 先导入syn 和 quote 包
// 然后写一个自己舒服的派生结构

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn mk_compile_error<T: std::fmt::Display>(span: Span, msg: T) -> TokenStream {
    syn::Error::new(span, msg).to_compile_error().into()
}

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let src_ident = input.ident;

    quote! {}.into()
}
