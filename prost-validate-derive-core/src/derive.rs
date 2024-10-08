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
    let input = syn::parse2(input).unwrap();
    let opts = Opts::from_derive_input(&input).expect("Wrong validate options");
    let DeriveInput { ident, .. } = input;

    let implementation = match opts.data {
        Data::Enum(e) => e
            .iter()
            .map(|v| v.to_token_stream())
            .collect::<proc_macro2::TokenStream>(),
        Data::Struct(s) => s
            .fields
            .iter()
            .map(|field| field.into_token_stream())
            .collect::<proc_macro2::TokenStream>(),
    };

    if !implementation.is_empty() {
        quote! {
            impl ::prost_validate::Validator for #ident {
                fn validate(&self) -> prost_validate::Result<()> {
                    #implementation
                    Ok(())
                }
            }
        }
    } else {
        quote! {
            impl ::prost_validate::Validator for #ident {}
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
            .args(&["--edition", "2018"])
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
            pub struct ComplexTestMsg {
            #[prost(string, tag = "1")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.const")]
            #[validate(r#type(string(r#const = "abcd")))]
            pub r#const: ::prost::alloc::string::String,
            #[prost(message, optional, boxed, tag = "2")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.nested")]
            pub nested: ::core::option::Option<::prost::alloc::boxed::Box<ComplexTestMsg>>,
            #[prost(int32, tag = "3")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.int_const")]
            #[validate(r#type(int32(r#const = 5)))]
            pub int_const: i32,
            #[prost(bool, tag = "4")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.bool_const")]
            #[validate(r#type(bool(r#const = false)))]
            pub bool_const: bool,
            #[prost(message, optional, tag = "5")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.float_val")]
            #[validate(r#type(float(gt = 0.0)))]
            pub float_val: ::core::option::Option<f32>,
            #[prost(message, optional, tag = "6")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.dur_val")]
            #[validate(r#type(duration(required = true, lt(seconds = 17))))]
            pub dur_val: ::core::option::Option<::prost_types::Duration>,
            #[prost(message, optional, tag = "7")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.ts_val")]
            #[validate(r#type(timestamp(gt(seconds = 7))))]
            pub ts_val: ::core::option::Option<::prost_types::Timestamp>,
            #[prost(message, optional, boxed, tag = "8")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.another")]
            pub another: ::core::option::Option<::prost::alloc::boxed::Box<ComplexTestMsg>>,
            #[prost(float, tag = "9")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.float_const")]
            #[validate(r#type(float(lt = 8.0)))]
            pub float_const: f32,
            #[prost(double, tag = "10")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.double_in")]
            #[validate(r#type(double(r#in = [456.789, 123.0])))]
            pub double_in: f64,
            #[prost(enumeration = "ComplexTestEnum", tag = "11")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.enum_const")]
            #[validate(r#type(r#enum(r#const = 2)))]
            pub enum_const: i32,
            #[prost(message, optional, tag = "12")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.any_val")]
            #[validate(r#type(any(r#in = ["type.googleapis.com/google.protobuf.Duration"])))]
            pub any_val: ::core::option::Option<::prost_types::Any>,
            #[prost(message, repeated, tag = "13")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.rep_ts_val")]
            #[validate(r#type(repeated(items(r#type(timestamp(gte(nanos = 1000000)))))))]
            pub rep_ts_val: ::prost::alloc::vec::Vec<::prost_types::Timestamp>,
            #[prost(map = "sint32, string", tag = "14")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.map_val")]
            #[validate(r#type(map(keys(r#type(sint32(lt = 0))))))]
            pub map_val: ::std::collections::HashMap<i32, ::prost::alloc::string::String>,
            #[prost(bytes = "vec", tag = "15")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.bytes_val")]
            #[validate(r#type(bytes(r#const = b"\0\x99")))]
            pub bytes_val: ::prost::alloc::vec::Vec<u8>,
            #[prost(oneof = "complex_test_msg::O", tags = "16, 17")]
            #[validate(name = "tests.harness.cases.ComplexTestMsg.o")]
            #[validate(required)]
            pub o: ::core::option::Option<complex_test_msg::O>,
        }
        };
        println!("{}", derive_2(input));
    }
}
