use proc_macro::TokenStream;

mod derive_stat_type;

#[proc_macro_derive(StatType, attributes(stat_type))]
pub fn derive_stat_type(input: TokenStream) -> TokenStream {
    derive_stat_type::derive_stat_type(input)
}
