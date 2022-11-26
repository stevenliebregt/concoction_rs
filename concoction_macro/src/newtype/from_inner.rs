use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Field, Fields, FieldsUnnamed};

pub(crate) fn expand_derive_from_inner(
    input: &syn::DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;
    let inner_type = get_newtype_inner_type(&input.data);

    Ok(quote!(
        impl From<#inner_type> for #ident {
            fn from(inner: #inner_type) -> Self {
                Self(inner)
            }
        }
    ))
}

// TODO: Move to internal utils
fn get_newtype_inner_type(data: &Data) -> Field {
    match data {
        Data::Struct(DataStruct { fields, .. }) => {
            match fields {
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let field = &unnamed[0];
                    field.clone()
                },
                _ => panic!("Can only get newtype inner type for structs with a single unnamed field")
            }
        }
        _ =>  panic!("Can only get newtype inner type for structs"),
    }
}