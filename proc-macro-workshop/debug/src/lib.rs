//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-08 14:55
// Distributed under terms of the MIT license.
// 03
// 为字段添加 debug 参数
// 为结构体 派生fmt::Debug 特征
// 在main.rs 派生了内置debug方法 对比参考写一个自己的

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn mk_compile_error<T: std::fmt::Display>(span: Span, msg: T) -> TokenStream {
    syn::Error::new(span, msg).to_compile_error().into()
}
fn get_debug(field: syn::Field) -> Result<String, TokenStream> {
    for a in field.attrs.iter() {
        if a.path.segments.len() == 0 {
            return Ok("".to_owned());
        }
        let s1 = a.path.segments.first().unwrap();
        if s1.ident.to_string() == "debug" {
            for t in a.tokens.clone() {
                match t {
                    proc_macro2::TokenTree::Literal(g) => {
                        let res = g.to_string();
                        if res != "".to_string() {
                            return Ok(res.replace("\"", ""));
                        } else {
                            return Err(mk_compile_error(g.span(), "expected debug= \"...\""));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return Ok("".to_owned());
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let src_ident = input.ident;
    let src_name = src_ident.to_string();
    let mut self_matches = Vec::new();
    let mut format_lines = Vec::new();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        for f in named {
            let i = f.ident.clone().unwrap();
            let i_name = i.to_string();
            self_matches.push(quote! {
                #i: ref #i,
            });
            let debug_str = get_debug(f.clone());
            if let Err(e) = debug_str {
                return e;
            }
            let debug_str = debug_str.unwrap();
            if debug_str != "".to_string() {
                format_lines.push(quote! {
                    let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, #i_name, &format_args!(#debug_str, &&(*#i)));
                });
            } else {
                format_lines.push(quote! {
                    let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, #i_name, &&(*#i));
                });
            }
        }
    } else {
        return mk_compile_error(
            src_ident.span(),
            "This Builder can just be applied to structs with named fields",
        );
    }

    quote! {
        impl ::core::fmt::Debug for #src_ident {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self{
                    #( #self_matches )*
                }=> {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, #src_name);
                #( #format_lines )*
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
        }
    }
    .into()
}
