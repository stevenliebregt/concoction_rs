use quote::quote;
use syn::parse_macro_input;

mod newtype;

/// A derive macro that generates a `From<T>` implementation where `T` is the inner type of the
/// newtype.
#[proc_macro_derive(FromInner, attributes(from_inner))]
pub fn derive_from_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    newtype::from_inner::expand_derive_from_inner(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
