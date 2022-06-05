//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 03 主要是为 CommandBuilder 实现build方法
// 涉及到特征对象 Box<dyn Error>> 中 Error 特征未导入

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    // 将rust code转换成 TokenStream
    //
    // 编写 CommandBuilder 结构体 及其方法
    let command_builder = quote! {
    use std::error::Error;
    struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }
    impl CommandBuilder {
        pub fn build(&mut self) -> Result<#ident, Box<dyn Error>> {
            if None == self.executable
                || None == self.args
                || None == self.env
                || None == self.current_dir
            {
                return Err(Box::from("missing fields".to_owned()));
            }
            Ok(#ident{
                executable: self.executable.as_ref().unwrap().to_owned(),
                args: self.args.as_ref().unwrap().to_vec(),
                env: self.env.as_ref().unwrap().to_vec(),
                current_dir: self.current_dir.as_ref().unwrap().to_owned(),
            })
        }
        pub fn executable(&mut self, executable: String) -> &mut Self {
            self.executable = Some(executable);
            self
        }
        pub fn args(&mut self, args: Vec<String>) -> &mut Self {
            match &self.args {
                Some(v) => v.to_owned().extend(args),
                None => self.args = Some(args),
            };
            self
        }
        pub fn env(&mut self, env: Vec<String>) -> &mut Self {
            match &self.env {
                Some(v) => v.to_owned().extend(env),
                None => self.env = Some(env),
            };
            self
        }
        pub fn current_dir(&mut self, d: String) -> &mut Self {
            self.current_dir = Some(d);
            self
        }
    }
        };

    // 派生builder方法 并且添加 CommandBuilder 语法树
    let token = quote! {
    #command_builder
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
    };
    token.into()
}
