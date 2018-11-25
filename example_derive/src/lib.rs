extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::DeriveInput;

mod contents;

#[proc_macro_derive(ContentsLen)]
pub fn contents_len_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_derive(tokens, contents::derive)
}

fn expand_derive(
    input: proc_macro::TokenStream,
    f: fn(DeriveInput) -> Result<proc_macro2::TokenStream, String>,
) -> proc_macro::TokenStream {
    let item = syn::parse(input).unwrap();
    match f(item) {
        Ok(x) => x,
        Err(e) => {
            quote! {
                compile_error(#e)
            }
        }
    }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
