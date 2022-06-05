//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 02 中使用字符串追加的方法比较不方便， 所以本节之后采用syn和quote 库
// 再次感谢大佬 dtolnay
// [syn](https://github.com/dtolnay/syn) 是一个将 ident数组 (TokenStream.标记流) 转换成ast 语法树的解析库
// [quote](https://github.com/dtolnay/quote) 可以用来辅助将rust 语法树 转换成标记流(TokenStream).
// 插值语法 #var     变量 var 要实现特征 quote::ToTokens

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
    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }
    impl CommandBuilder {
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
