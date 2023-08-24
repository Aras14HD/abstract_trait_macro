// TODO: where (unstable)
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, punctuated::Punctuated, token, Ident, Result, Token, TypeParam};

#[allow(dead_code)]
struct Input {
    name: Ident,
    semi_token: token::Semi,
    types: Punctuated<TypeParam, Token![;]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            semi_token: input.parse()?,
            types: input.parse_terminated(TypeParam::parse, Token![;])?,
        })
    }
}

#[proc_macro]
pub fn abst_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Input);
    let name = input.name;
    let types: Vec<Ident> = input.types.iter().map(|x| x.clone().ident).collect();
    let types_trait: Vec<TypeParam> = input.types.iter().map(|x| x.clone()).collect();
    let res = quote! {
        trait #name {
            #(type #types_trait;)*
        }
        impl<#(#types_trait),*> #name for (#(#types),*) {
            #(type #types = #types;)*
        }
    }
    .into();

    println!("{}", res);

    res
}
