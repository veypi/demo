//
// lib.rs
// Copyright (C) 2022 veypi <veypi@qq.com>
// 2022-06-05 17:33
// Distributed under terms of the MIT license.
// 先使用最原始的追加字符串的方法派生代码

use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut name = "".to_string();
    for i in input {
        // TokenStream 是个包含四种类型的枚举
        match i {
            proc_macro::TokenTree::Ident(t) => {
                eprintln!("{:#?}", t.to_string());
                name = t.to_string();
            }
            proc_macro::TokenTree::Group(t) => eprintln!("{:#?}", t.to_string()),
            proc_macro::TokenTree::Literal(t) => eprintln!("{:#?}", t),
            proc_macro::TokenTree::Punct(t) => eprintln!("{:#?}", t),
        };
    }
    // 仅替换了类型名字
    TokenStream::from_str(&format!(
        "
pub struct CommandBuilder {{
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}}
impl {} {{
    pub fn builder() -> CommandBuilder {{
        CommandBuilder {{
            executable: None,
            args: None,
            env: None,
            current_dir: None,
        }}
    }}
}}
        ",
        name
    ))
    .unwrap()
}
