extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::DeriveInput;

mod attributes;
mod contents;

#[proc_macro_derive(ContentsLen, attributes(type_variant))]
pub fn contents_len_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_derive(tokens.into(), contents::derive).into()
}

fn expand_derive(
    input: proc_macro2::TokenStream,
    f: fn(DeriveInput) -> Result<proc_macro2::TokenStream, String>,
) -> proc_macro2::TokenStream {
    let item = syn::parse2(input).unwrap();
    match f(item) {
        Ok(x) => {
            println!("Ok {:?}", x);
            x
        },
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
