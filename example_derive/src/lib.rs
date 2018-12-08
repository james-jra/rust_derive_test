extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

mod attributes;
mod contents;

#[proc_macro_derive(ContentsLen, attributes(type_variant))]
pub fn contents_len_derive(tokens: TokenStream) -> TokenStream {
    expand_derive(tokens.into(), contents::derive).into()
}

fn expand_derive(
    input: TokenStream2,
    f: fn(DeriveInput) -> Result<TokenStream2, String>,
) -> TokenStream2 {
    let item = syn::parse2(input).unwrap();
    match f(item) {
        Ok(x) => {
            println!("Ok {:?}", x);
            x
        }
        Err(e) => {
            println!("Err {:?}", e);
            quote! {
                compile_error!(#e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
