use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Variant, Token};

pub struct TypVariant(pub Variant);

impl Parse for TypVariant {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse the
        let mut vars = Punctuated::<Variant, Token![,]>::parse_terminated(input)?;
        if vars.len() != 1 {
            Err(input.error("#[typ_variant] only supports one Variant value."))
        } else {
            Ok(TypVariant(vars.pop().unwrap().into_value()))
        }
    }
}

impl TypVariant {
    pub fn try_from_attribute(attr: Attribute) -> Result<Self, proc_macro2::TokenStream> {
        match syn::parse2(attr.tts) {
            Ok(data) => Ok(data),
            Err(err) => Err(proc_macro2::TokenStream::from(err.to_compile_error()))
        }
        // let typ = match syn::parse_macro_input::parse::<TypVariant>(tts_1) {
        //     syn::export::Ok(data) => data,
        //     syn::export::Err(err) => {
        //         return syn::export::TokenStream::from(err.to_compile_error());
        //     }
        // };
    }
}
