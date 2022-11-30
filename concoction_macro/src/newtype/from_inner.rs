use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::ParseStream;
use syn::{Attribute, Data, DataStruct, Error, Field, Fields, FieldsUnnamed, Lifetime, Token};

#[derive(Debug)]
struct FromInnerLifetimeArgs {
    lifetimes: Vec<Lifetime>,
}

impl syn::parse::Parse for FromInnerLifetimeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let arg_type: Ident = input.parse()?;

        if arg_type != "lifetimes" {
            return Err(Error::new(arg_type.span(), "Not a `lifetimes` attribute"));
        }

        let _: Token![=] = input.parse()?;

        let mut lifetimes: Vec<Lifetime> = vec![];

        loop {
            match input.parse::<Lifetime>() {
                Ok(lifetime) => {
                    lifetimes.push(lifetime);
                    // Try to consume a comma
                    match input.parse::<Token![,]>() {
                        Ok(_) => {
                            // No worries, continue to parse next lifetime
                        }
                        Err(_) => {
                            // No comma, then we assume no more lifetimes
                            break;
                        }
                    }
                }
                Err(_) => {
                    // Some problem, most probably end of input
                    break;
                }
            }
        }

        Ok(FromInnerLifetimeArgs { lifetimes })
    }
}

pub(crate) fn expand_derive_from_inner(
    input: &syn::DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;
    let inner_type = get_newtype_inner_type(&input.data);

    let from_inner_attributes = input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("from_inner"))
        .collect::<Vec<_>>();
    let args = parse_attributes(&from_inner_attributes);

    let lifetime_tokens = if let Some(lifetime_args) = args {
        let lifetimes = lifetime_args.lifetimes;
        quote! { <#(#lifetimes),*> }
    } else {
        quote! {}
    };

    Ok(quote!(
        impl #lifetime_tokens From<#inner_type> for #ident #lifetime_tokens {
            fn from(inner: #inner_type) -> Self {
                Self(inner)
            }
        }
    ))
}

fn parse_attributes(attributes: &[&Attribute]) -> Option<FromInnerLifetimeArgs> {
    let mut lifetime_args: Option<FromInnerLifetimeArgs> = None;

    for attribute in attributes {
        // Try to parse lifetime args if none
        if lifetime_args.is_none() {
            if let Ok(with_lifetime) = attribute.parse_args::<FromInnerLifetimeArgs>() {
                lifetime_args = Some(with_lifetime);
            }
        }
    }

    lifetime_args
}

// TODO: Move to internal utils
fn get_newtype_inner_type(data: &Data) -> Field {
    match data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let field = &unnamed[0];
                field.clone()
            }
            _ => panic!("Can only get newtype inner type for structs with a single unnamed field"),
        },
        _ => panic!("Can only get newtype inner type for structs"),
    }
}
