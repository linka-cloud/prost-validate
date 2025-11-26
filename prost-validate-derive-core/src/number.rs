use crate::field::{Context, ToValidationTokens};
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Not;
use syn::{LitFloat, LitInt};

macro_rules! make_number_rules {
    ($name:ident,$typ:ident,$lit:ident,$module:ident) => {
        #[derive(Debug, FromMeta, Clone)]
        pub struct $name {
            pub r#const: Option<$typ>,
            pub lt: Option<$typ>,
            pub lte: Option<$typ>,
            pub gt: Option<$typ>,
            pub gte: Option<$typ>,
            pub r#in: Option<Vec<$lit>>,
            pub not_in: Option<Vec<$lit>>,
            #[darling(default)]
            pub ignore_empty: bool,
        }

        impl ToValidationTokens for $name {
            fn to_validation_tokens(&self, ctx: &Context, name: &Ident) -> TokenStream {
                let field = &ctx.name;
                let rules = prost_validate_types::$name::from(self.to_owned());
                let maybe_return = ctx.maybe_return();
                let r#const = rules.r#const.map(|v| {
                    quote! {
                        if *#name != #v {
                            #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::Const(#v)));
                        }
                    }
                });
                // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/ltgt.go
                let lte_gte = if let Some(lt) = rules.lt {
                    if let Some(gt) = rules.gt {
                        if lt > gt {
                            quote! {
                                if *#name <= #gt || *#name >= #lt {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::in_range(false, #gt, #lt, false)));
                                }
                            }
                        } else {
                            quote! {
                                if *#name >= #lt && *#name <= #gt {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::not_in_range(true, #lt, #gt, true)));
                                }
                            }
                        }
                    } else if let Some(gte) = rules.gte {
                        if lt > gte {
                            quote! {
                                if *#name < #gte || *#name >= #lt {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::in_range(true, #gte, #lt, false)));
                                }
                            }
                        } else {
                            quote! {
                                if *#name >= #lt && *#name < #gte {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::not_in_range(true, #lt, #gte, false)));
                                }
                            }
                        }
                    } else {
                        quote! {
                            if *#name >= #lt {
                                #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::Lt(#lt)));
                            }
                        }
                    }
                } else if let Some(lte) = rules.lte {
                    if let Some(gt) = rules.gt {
                        if lte > gt {
                            quote! {
                                if *#name <= #gt || *#name > #lte {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::in_range(false, #gt, #lte, true)));
                                }
                            }
                        } else {
                            quote! {
                                if *#name > #lte && *#name <= #gt {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::not_in_range(false, #lte, #gt, true)));
                                }
                            }
                        }
                    } else if let Some(gte) = rules.gte {
                        if lte > gte {
                            quote! {
                                if *#name < #gte || *#name > #lte {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::in_range(true, #gte, #lte, true)));
                                }
                            }
                        } else {
                            quote! {
                                if *#name > #lte && *#name < #gte {
                                    #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::not_in_range(false, #lte, #gte, false)));
                                }
                            }
                        }
                    } else {
                        quote! {
                            if *#name > #lte {
                                #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::Lte(#lte)));
                            }
                        }
                    }
                } else if let Some(gt) = rules.gt {
                    quote! {
                        if *#name <= #gt {
                            #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::Gt(#gt)));
                        }
                    }
                } else if let Some(gte) = rules.gte {
                    quote! {
                        if *#name < #gte {
                            #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::Gte(#gte)));
                        }
                    }
                } else {
                    quote! {}
                };
                let r#in = rules.r#in.is_empty().not().then(|| {
                    let v = rules.r#in.to_owned();
                    quote! {
                        let values = vec![#(#v),*];
                        if !values.contains(#name) {
                            #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::In(values.to_vec())));
                        }
                    }
                });
                let not_in = rules.not_in.is_empty().not().then(|| {
                    let v = rules.not_in.to_owned();
                    quote! {
                        let values = vec![#(#v),*];
                        if values.contains(#name) {
                            #maybe_return(::prost_validate::Error::new(#field, ::prost_validate::errors::$module::Error::NotIn(values.to_vec())));
                        }
                    }
                });
                if self.ignore_empty {
                    quote! {
                        if *#name != $typ::default() {
                            #r#const
                            #lte_gte
                            #r#in
                            #not_in
                        }
                    }
                } else {
                    quote! {
                        #r#const
                        #lte_gte
                        #r#in
                        #not_in
                    }
                }
            }
        }

        impl From<$name> for prost_validate_types::$name {
            fn from(value: $name) -> Self {
                prost_validate_types::$name {
                    r#const: value.r#const,
                    lt: value.lt,
                    lte: value.lte,
                    gt: value.gt,
                    gte: value.gte,
                    r#in: value
                        .r#in
                        .unwrap_or_default()
                        .iter()
                        .map(|v| v.base10_parse().unwrap())
                        .collect(),
                    not_in: value
                        .not_in
                        .unwrap_or_default()
                        .iter()
                        .map(|v| v.base10_parse().unwrap())
                        .collect(),
                    ignore_empty: Some(value.ignore_empty),
                }
            }
        }
    };
}

make_number_rules!(UInt64Rules, u64, LitInt, uint64);
make_number_rules!(UInt32Rules, u32, LitInt, uint32);
make_number_rules!(Int64Rules, i64, LitInt, int64);
make_number_rules!(Int32Rules, i32, LitInt, int32);
make_number_rules!(DoubleRules, f64, LitFloat, double);
make_number_rules!(FloatRules, f32, LitFloat, float);
make_number_rules!(SInt32Rules, i32, LitInt, sint32);
make_number_rules!(SInt64Rules, i64, LitInt, sint64);
make_number_rules!(Fixed32Rules, u32, LitInt, fixed32);
make_number_rules!(Fixed64Rules, u64, LitInt, fixed64);
make_number_rules!(SFixed32Rules, i32, LitInt, sfixed32);
make_number_rules!(SFixed64Rules, i64, LitInt, sfixed64);
