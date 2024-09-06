#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
use proc_macro::TokenStream;
use std::ops::Add;
use syn::{Data};

#[proc_macro_derive(TypeName)]
pub fn type_name(input: TokenStream) -> TokenStream {
    // Parse token stream into input AST
    let ast = syn::parse(input).unwrap();
    impl_typename(&ast).into()
}

fn impl_typename(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let data = &ast.data;

    let mut fields = Option::None;

    match data {
        Data::Struct(s) => {
            let field: Vec<_> = s.fields.iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();
            fields = Some(field);
        }
        Data::Enum(_) => {}
        Data::Union(_) => {}
    }
    let result_string = match fields {
        None => "no fields".to_string(),
        Some(fields) => fields.iter().fold(String::new(), |f, u| f.add(u.as_ref()).add(" "))
    };
    quote! {
        impl TypeName for #name {
            fn typename() -> String {
                stringify!(#name).to_string()
            }
            fn attributes(&self) -> String {
                #result_string.to_string()
            }
        }
    }
}
