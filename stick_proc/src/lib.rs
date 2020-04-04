use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {}
