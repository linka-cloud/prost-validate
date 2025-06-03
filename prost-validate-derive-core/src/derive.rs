use crate::field::Field;
use darling::ast::Data;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(validate, prost), supports(struct_named, enum_any))]
struct Opts {
    data: Data<Field, Field>,
}

pub fn derive(input: TokenStream) -> proc_macro2::TokenStream {
    derive_with_module(input, None)
}

pub fn derive_with_module(
    input: TokenStream,
    module: Option<TokenStream>,
) -> proc_macro2::TokenStream {
    let input = syn::parse2(input).unwrap();
    let opts = Opts::from_derive_input(&input).expect("Wrong validate options");
    let DeriveInput { ident, .. } = input;

    let implementation = match opts.data {
        Data::Enum(e) => e
            .iter()
            .map(|v| Field {
                module: module.clone().map(|v| v.to_string()),
                ..v.clone()
            })
            .map(|v| v.to_token_stream())
            .collect::<proc_macro2::TokenStream>(),
        Data::Struct(s) => s
            .fields
            .iter()
            .map(|v| Field {
                module: module.clone().map(|v| v.to_string()),
                ..v.clone()
            })
            .map(|field| field.into_token_stream())
            .collect::<proc_macro2::TokenStream>(),
    };

    let allow = quote! {
        #[allow(clippy::regex_creation_in_loops)]
        #[allow(irrefutable_let_patterns)]
        #[allow(unused_variables)]
    };

    let path = if let Some(module) = module {
        quote! { #module::#ident }
    } else {
        quote! { #ident }
    };
    if !implementation.is_empty() {
        quote! {
            impl ::prost_validate::Validator for #path {
                #allow
                fn validate(&self) -> prost_validate::Result<()> {
                    #implementation
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl ::prost_validate::Validator for #path {}
        }
    }
}

#[cfg(test)]
pub fn derive_2(input: proc_macro2::TokenStream) -> String {
    let output = derive(input);
    let syntax_tree = syn::parse2(output.clone())
        .map_err(|e| format!("Failed to parse syntax tree: {}\n{}", e, output))
        .unwrap();
    prettyplease::unparse(&syntax_tree)
}

#[cfg(test)]
pub mod test_utils {
    use proc_macro2::TokenStream;
    use std::io::Write;
    use std::process::Command;

    pub fn _format(tk: &TokenStream) -> anyhow::Result<String> {
        let mut rustfmt = Command::new("rustfmt")
            .args(["--edition", "2018"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        rustfmt
            .stdin
            .take()
            .unwrap()
            .write_all(tk.to_string().as_bytes())?;
        let output = rustfmt.wait_with_output()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = quote! {
            pub struct WrapperRequiredFloat {
                #[prost(message, optional, tag = "1")]
                #[validate(name = "tests.harness.cases.WrapperRequiredFloat.val")]
                #[validate(r#type(float(gt = 0.0)), message(required = true))]
                pub val: ::core::option::Option<::pbjson_types::FloatValue>,
            }
        };
        println!("{}", derive_2(input));
    }
}
