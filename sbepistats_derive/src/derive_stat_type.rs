use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::{DeriveInput, Type, parse_quote};

pub fn derive_stat_type(input: TokenStream) -> TokenStream {
    match action(syn::parse_macro_input!(input as DeriveInput)) {
        Ok(expr) => expr.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn action(input: DeriveInput) -> syn::Result<syn::ItemImpl> {
    let ident = &input.ident;

    let attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("stat_type"));
    let data_type = attr
        .map(|attr| attr.parse_args_with(Type::parse))
        .transpose()?
        .unwrap_or(parse_quote!(f32));

    Ok(parse_quote! {
        impl ::sbepistats::StatType for #ident {
            type DataType = #data_type;
        }
    })
}
