use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token};

struct TypVariant(Ident);

impl Parse for TypVariant {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse the
        let mut vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        if vars.len() != 1 {
            Err(input.error("Only one value ident for #[typ_variant] attribute"))
        } else {
            Ok(TypVariant(vars.pop().unwrap().into_value()))
        }
    }
}

impl TypVariant {
    fn get(tts: proc_macro2::TokenStream) -> Result<Self, String> {
        let typ = parse_macro_input!(tts as TypVariant);
    }
}
