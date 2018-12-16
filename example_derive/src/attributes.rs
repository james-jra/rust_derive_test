use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Path, PathSegment, Token};

pub struct TypVariant(pub Path);

impl Parse for TypVariant {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse the
        // println!("Bar0");
        // println!("{:?}", input);
        // let path = input.call(Path::parse_mod_style)?;
        // println!("Bar1");
        // Ok(TypVariant(path))
        let mut vars = Punctuated::<PathSegment, Token![,]>::parse_terminated(input)?;
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
        println!("Baz1");
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(err) => return Err(proc_macro2::TokenStream::from(err.to_compile_error())),
        };
        println!("Baz2");

        match meta {
            Meta::Word(word) => println!("Word {:?}", word),
            Meta::List(list) => println!("List {:?}", list.ident),
            Meta::NameValue(nv) => println!("NameValue {:?}", nv.ident),
        };
        println!("Foo1");
        match syn::parse2(attr.tts) {
            Ok(data) => Ok(data),
            Err(err) => Err(proc_macro2::TokenStream::from(err.to_compile_error())),
        }
    }
}
