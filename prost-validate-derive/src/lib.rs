use proc_macro::{self, TokenStream};

#[proc_macro_derive(Validator, attributes(validate, prost))]
pub fn derive(input: TokenStream) -> TokenStream {
    prost_validate_derive_core::derive(input.into()).into()
}
