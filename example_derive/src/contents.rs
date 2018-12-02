use quote::quote;
//use quote::{quote, quote_spanned};
// use syn::{
//     parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
// };
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::{Attribute, AttrStyle, Data, DeriveInput, Fields, GenericParam, Generics};

use attributes::TypVariant;

/// Generate the implementation of the `ContentsLen` trait for the given item.
///
/// Returns a `Result` containing the `TokenStream` representing the trait
/// implementation, or an error string describing the problem.
pub fn derive(item: DeriveInput) -> Result<proc_macro2::TokenStream, String> {
    // Validate the input data.
    check_struct_data(&item.data)?;
    check_attrs(&item.attrs)?;
    let struct_name = item.ident;

    // Extract the #[typ_variant(X)] attribute
    let typ_variant = TypVariant::try_from_attribute(item.attrs[0].clone());
    if let Err(err) = typ_variant {
        return Ok(err);
    }
    let typ_variant = typ_variant.unwrap();

    // Add `T: ContentsLen` to all type parameters
    let generics = add_trait_bounds(item.generics);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    //let typ_body = gen_typ_body(&typ_variant)?;
    let typ_body = gen_typ_body()?;
    let typ = quote!{
        fn typ(&self) -> Typ {
            #typ_body
        }
    };

    let contents_len_body = gen_contents_len_body(&item.data)?;
    let contents_len = quote!{
        fn contents_len(&self) -> usize {
            #contents_len_body
        }
    };

    let impl_contents_size = quote!{
        // The generated impl.
        impl #impl_generics ::contents_len::ContentsLen for #struct_name #type_generics #where_clause {
            // The fn typ() block
            #typ
            // The fn contents_len() block
            #contents_len
        }
    };

    Ok(impl_contents_size)
}

/// Generate the body of the `contents_len` function from some `Data`
///
/// # Panics
///
/// This function will panic if the provided `Data` represents an object
/// for which this trait derivation is not supported.
/// To avoid this panic, ensure `check_struct_data` is used first to
/// validate the trait derivation is supported for this data.
// Copies the Syn example approach of using the span of each `syn::Field`
// as the span of the corresponding function call to improve compile error
// messages:
// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
fn gen_contents_len_body(data: &Data) -> Result<proc_macro2::TokenStream, String> {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + self.x.contents_len() + self.y.contents_len() ...
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::contents_len::ContentsLen::contents_len(&self.#name)
                        }
                    });
                    Ok(quote! {
                        0 #(+ #recurse)*
                    })
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

//fn gen_typ_body(typ_variant: &TypVariant) -> Result<proc_macro2::TokenStream, String> {
    // let typ_ident = typ_variant.0;
    // Ok(quote! { #typ_ident })
fn gen_typ_body() -> Result<proc_macro2::TokenStream, String> {
    Ok(quote! { Typ::Foo })
}

/// Add a bound `T: ContentsLen` to every type parameter T.
/// Copied from Syn example code:
/// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::heapsize::HeapSize));
        }
    }
    generics
}

fn check_attrs(attrs: &Vec<Attribute>) -> Result<(), String> {
    match attrs.len() {
        0 => Err("#[derive(ContentsLen)] requires the helper attribute: #[typ_variant(X)]".into()),
        1 => {
            match attrs[0].style {
                AttrStyle::Inner(_) => {
                    Err("#[typ_variant(X)] can only be used as an Outer style attribute".into())
                }
                AttrStyle::Outer => {
                    Ok(())
                }
            }
        },
        _ => Err("#[derive(ContentsLen)] only accepts one helper attribute: typ_variant".into()),
    }
}

/// Validate that trait derivation is defined for the provided item.
/// Returns `Ok(())` or an error string describing the problem.
fn check_struct_data(data: &Data) -> Result<(), String> {
    match data {
        Data::Struct(s) => match s.fields {
            Fields::Named(_) => Ok(()),
            Fields::Unnamed(_) => Err(String::from(
                "#[derive(ContentsLen)] is not defined for tuple structs or tuple variants",
            )),
            Fields::Unit => Err(String::from(
                "#[derive(ContentsLen)] is not defined for unit structs",
            )),
        },
        _ => Err(String::from(
            "#[derive(ContentsLen)] is only defined for structs",
        )),
    }
}
