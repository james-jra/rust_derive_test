use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Token};

pub struct TypVariant(pub Ident);

impl Parse for TypVariant {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse the
        println!("Foo2");
        let mut vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        println!("Foo3");
        if vars.len() != 1 {
            println!("Foo4");
            Err(input.error("#[typ_variant] only supports one Variant value."))
        } else {
            println!("Foo5");
            Ok(TypVariant(vars.pop().unwrap().into_value()))
        }
    }
}

impl TypVariant {
    pub fn try_from_attribute(attr: Attribute) -> Result<Self, proc_macro2::TokenStream> {
        println!("Foo1");
        match syn::parse2(attr.tts) {
            Ok(data) => Ok(data),
            Err(err) => Err(proc_macro2::TokenStream::from(err.to_compile_error())),
        }
    }
}
