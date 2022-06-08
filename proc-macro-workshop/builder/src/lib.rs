//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 09
// 考虑到常用函数被别名替换，所有使用prelude 预导入包时尽量使用包绝对引用路径而不是alias

use std::error::Error;
use std::vec;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

fn mk_compile_error<T: std::fmt::Display>(span: Span, msg: T) -> TokenStream {
    syn::Error::new(span, msg).to_compile_error().into()
}

fn is_type(ty: syn::Type, n: &str) -> bool {
    match ty {
        syn::Type::Path(t) => {
            if t.path.segments.first().unwrap().ident.to_string() == n {
                return true;
            }
        }
        _ => {}
    }
    false
}

fn is_option(ty: syn::Type) -> bool {
    is_type(ty, "Option")
}

fn is_vec(ty: syn::Type) -> bool {
    is_type(ty, "Vec")
}

fn get_each(field: syn::Field) -> Result<String, TokenStream> {
    let mut res = "".to_string();
    let mut ident = "".to_string();
    for a in field.attrs.iter() {
        if a.path.segments.len() == 0 {
            return Ok("".to_owned());
        }
        let s1 = a.path.segments.first().unwrap();
        if s1.ident.to_string() == "builder" {
            for t in a.tokens.clone() {
                match t {
                    proc_macro2::TokenTree::Group(g) => {
                        let s = g.stream();
                        for i in s {
                            match i {
                                proc_macro2::TokenTree::Ident(temp) => ident = temp.to_string(),
                                proc_macro2::TokenTree::Literal(temp) => res = temp.to_string(),
                                _ => {}
                            }
                        }
                        if ident == "each".to_string() {
                            if res != "".to_string() {
                                return Ok(res.replace("\"", ""));
                            }
                        } else {
                            return Err(mk_compile_error(
                                g.span(),
                                "expected `builder(each = \"...\")`",
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return Ok("".to_owned());
}

fn get_sub_type(ty: syn::Type, n: &str) -> Result<syn::Type, Box<dyn Error>> {
    match ty {
        syn::Type::Path(t) => {
            let i1 = t.path.segments.first().unwrap();
            match &i1.arguments {
                syn::PathArguments::AngleBracketed(ia) => match ia.args.first().unwrap() {
                    syn::GenericArgument::Type(it) => {
                        if i1.ident.to_string() == n {
                            return Ok(it.to_owned());
                        }
                        return get_sub_type(it.to_owned(), n);
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        _ => {}
    }
    Err(Box::from("invalid type".to_owned()))
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let mut methods = Vec::new();
    let mut builder_lines = Vec::new();
    let mut command_builder_items = vec![];
    let mut build_if = vec![];
    let mut build_ok = vec![];

    match input.data {
        syn::Data::Struct(v) => {
            for f in v.fields {
                let i = f.ident.clone();
                let mut ty = f.ty.clone();
                let each = get_each(f.clone());
                if let Err(err) = each {
                    return err;
                }
                let each = each.unwrap();
                let is_v = is_vec(ty.clone());
                let is_opt = is_option(ty.clone());
                if is_v {
                    builder_lines.push(quote! {
                        #i: ::std::vec::Vec::new(),
                    });
                    command_builder_items.push(quote! {
                        #i: #ty,
                    });
                    if each != "".to_string() {
                        let each = format_ident!("{}", each);
                        let ty = get_sub_type(ty.clone(), "Vec").unwrap();
                        methods.push(quote! {
                        pub fn #each(&mut self, #each: #ty) -> &mut Self{
                            self.#i.push(#each);
                            self
                        }
                        })
                    }
                    if each != i.clone().unwrap().to_string() {
                        methods.push(quote! {
                            pub fn #i(&mut self, #i: #ty) -> &mut Self {
                                self.#i = #i;
                                self
                            }
                        });
                    }
                    build_ok.push(quote! {
                        #i: self.#i.clone(),
                    })
                } else {
                    // 非vec对象皆转换为option对象
                    builder_lines.push(quote! {
                        #i: ::std::option::Option::None,
                    });
                    if is_opt {
                        command_builder_items.push(quote! {
                            #i: #ty,
                        });
                        ty = get_sub_type(ty.clone(), "Option").unwrap();
                        build_ok.push(quote! {
                            #i: self.#i.clone(),
                        })
                    } else {
                        if build_if.len() == 0 {
                            build_if.push(quote! {
                                None == self.#i
                            });
                        } else {
                            build_if.push(quote! {
                                || None == self.#i
                            });
                        }
                        command_builder_items.push(quote! {
                            #i: ::std::option::Option<#ty>,
                        });
                        build_ok.push(quote! {
                            #i: self.#i.as_ref().unwrap().to_owned(),
                        })
                    }
                    methods.push(quote! {
                        pub fn #i(&mut self, #i: #ty) -> &mut Self {
                            self.#i = ::std::option::Option::Some(#i);
                            self
                        }
                    });
                }
            }
        }
        _ => panic!("invalid data"),
    };
    let mut build = vec![];
    if build_if.len() != 0 {
        build.push(quote! {
            if #( #build_if )* {
                return ::std::result::Result::Err(::std::boxed::Box::from("missing fields".to_owned()));
            };
        });
    }
    build.push(quote! {
        ::std::result::Result::Ok(#ident {
            #( #build_ok )*
        })
    });

    let token = quote! {
    impl #ident {
        fn builder() -> CommandBuilder {
            CommandBuilder {
                #( #builder_lines )*
            }
        }
    }
    struct CommandBuilder {
        #( #command_builder_items )*
    }
    impl CommandBuilder {
        pub fn build(&mut self) -> ::std::result::Result<#ident, ::std::boxed::Box<dyn std::error::Error>> {
             #(#build)*
        }
        #( #methods )*
    }
    };
    token.into()
}
