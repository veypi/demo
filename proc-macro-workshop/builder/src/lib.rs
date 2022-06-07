//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 06 主要是 CommandBuilder 字段中出现了option 字段， 需要代码分析字段并生成相应方法
// 难点就是很多处方法需要根据字段是不是option的进行分类处理

use std::vec;

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let mut fields: Vec<syn::Field> = vec![];
    let mut option_fields: Vec<syn::Field> = vec![];
    match input.data {
        syn::Data::Struct(v) => {
            for i in v.fields {
                fields.push(i.clone());
                match i.ty {
                    syn::Type::Path(t) => {
                        if t.path.segments.first().unwrap().ident.to_string() == "Option" {
                            option_fields.push(fields.pop().unwrap());
                        }
                    }
                    _ => {}
                };
            }
        }
        _ => panic!("asd"),
    };
    // 将rust code转换成 TokenStream
    //
    // 编写 CommandBuilder 结构体 及其方法

    // 根据 两种类型字段生成 CommandBuilder 结构体
    let mut q = quote!();
    q.append_all(fields.iter().map(|v| {
        let ident = &v.ident;
        let ty = &v.ty;
        quote!(
        #ident: Option<#ty>,
        )
    }));
    q.append_all(option_fields.iter().map(|v| v));

    let cmd_buidler = quote! {
        struct CommandBuilder {
            #q
        }
    };

    // 构建build 方法
    // 第一段需要根据fileds判断是否有必填的字段, 如缺失字段则返回错误
    // 第二段返回Command 对象， 但是赋值需要动态判断
    let mut build_if = quote! {};
    let mut build_ok = quote! {};
    let mut build_method = quote! {};
    build_ok.append_all(fields.iter().map(|v| {
        let i = &v.ident;
        quote! {
            #i: self.#i.as_ref().unwrap().to_owned(),
        }
    }));
    build_ok.append_all(option_fields.iter().map(|v| {
        let i = &v.ident;
        quote! {
            #i: self.#i.clone(),
        }
    }));
    if fields.len() > 0 {
        let mut if1 = false;
        build_if.append_all(fields.iter().map(|v| {
            let i = &v.ident;
            if if1 {
                quote! {|| None == self.#i}
            } else {
                if1 = true;
                quote! {None == self.#i}
            }
        }));
        build_method = quote! {
            if #build_if {
             return Err(Box::from("missing fields".to_owned()));
            }
        };
    }
    let mut methods = quote! {
    pub fn build(&mut self) -> Result<#ident, Box<dyn Error>> {
        #build_method
        Ok(#ident{
            #build_ok
        })
    }
    };
    // 添加属性设置方法
    methods.append_all(fields.iter().map(|v| {
        let i = &v.ident;
        let ty = &v.ty;
        quote! {
            pub fn #i(&mut self, #i: #ty) -> &mut Self {
                self.#i = Some(#i);
                self
            }
        }
    }));
    methods.append_all(option_fields.iter().map(|v| {
        let i = &v.ident;
        let ty = match &v.ty {
            syn::Type::Path(t) => {
                let i1 = t.path.segments.first().unwrap();
                if i1.ident.to_string() == "Option" {
                    match &i1.arguments {
                        syn::PathArguments::AngleBracketed(ia) => ia.args.first().unwrap(),
                        _ => panic!("invalid data"),
                    }
                } else {
                    panic!("invalid data")
                }
            }
            _ => panic!("invalid data"),
        };
        quote! {
            pub fn #i(&mut self, #i: #ty) -> &mut Self {
                self.#i = Some(#i);
                self
            }
        }
    }));
    // 派生builder方法 并且添加 CommandBuilder 语法树
    let token = quote! {
    use std::error::Error;
    impl #ident {
        fn builder() -> CommandBuilder {
            CommandBuilder {
                executable: None,
                args: None,
                env: None,
                current_dir: None,
            }
        }
    }

    #cmd_buidler
    impl CommandBuilder {
        #methods
    }
    };
    token.into()
}
