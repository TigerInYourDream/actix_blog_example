extern crate proc_macro;

use quote::*;
use syn::*;
use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;

#[proc_macro_derive(GenMessage)]
pub fn derive(input: proc_macro::TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let token0: proc_macro2::TokenStream = impl_message(&ast)
        .unwrap_or(TokenStream::new())
        .into();
    let mut token = proc_macro2::TokenStream::new();
    token.append_all(token0);
    token.into()
}

fn impl_message(ast: &DeriveInput) -> Result<TokenStream> {
    // get Derive  name
    let name = &ast.ident;
    let gen = quote! {
        impl Message for #name {
            type Result = Result<String, Error>;
        }
    };
    Ok(gen.into())
}