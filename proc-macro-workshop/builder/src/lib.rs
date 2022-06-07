//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 07
// 回顾在04 中实现的这两个方法
// pub fn args(&mut self, args: Vec<String>) -> &mut Self {
//     match &self.args {
//         Some(v) => v.to_owned().extend(args),
//         None => self.args = Some(args),
//     };
//     self
// }
// pub fn env(&mut self, env: Vec<String>) -> &mut Self {
//     match &self.env {
//         Some(v) => v.to_owned().extend(env),
//         None => self.env = Some(env),
//     };
//     self
// }
// args 和env 调用是增加参数
// 而在上个06中我们实现了动态增加方法， 但是统一设置成了替换
// 07 就是要实现动态增加方法并且还是如上述形式去增加参数而不是替换全部的参数
// 通过对字段进行属性标记实现
//    #[builder(each = "arg")]
// 标记each的字段我们可以视为是vec对象， 否则报错

use std::error::Error;
use std::vec;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

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

fn get_each(attrs: Vec<syn::Attribute>) -> String {
    let mut res = "".to_string();
    let mut ident = "".to_string();
    for a in attrs {
        if a.path.segments.len() == 0 {
            return "".to_owned();
        }
        let s1 = a.path.segments.first().unwrap();
        if s1.ident.to_string() == "builder" {
            for t in a.tokens {
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
                                return res.replace("\"", "");
                            }
                            return "temp".to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return "".to_string();
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
                let each = get_each(f.attrs.clone());
                let is_v = is_vec(ty.clone());
                let is_opt = is_option(ty.clone());
                if is_v {
                    builder_lines.push(quote! {
                        #i: Vec::new(),
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
                        #i: None,
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
                            #i: Option<#ty>,
                        });
                        build_ok.push(quote! {
                            #i: self.#i.as_ref().unwrap().to_owned(),
                        })
                    }
                    methods.push(quote! {
                        pub fn #i(&mut self, #i: #ty) -> &mut Self {
                            self.#i = Some(#i);
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
                return Err(Box::from("missing fields".to_owned()));
            };
        });
    }
    build.push(quote! {
        Ok(#ident {
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
        pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
             #(#build)*
        }
        #( #methods )*
    }
    };
    token.into()
}
