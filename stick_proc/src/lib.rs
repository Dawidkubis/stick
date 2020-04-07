use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
	TokenStream::new()
}
