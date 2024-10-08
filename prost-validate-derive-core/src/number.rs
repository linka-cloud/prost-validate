use crate::field::{Context, ToValidationTokens};
use crate::utils::IsTrueAnd;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Not;
use syn::{LitFloat, LitInt};

macro_rules! make_number_rules {
    ($name:ident,$typ:ident,$lit:ident) => {
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
                let rules = prost_validate_types::$name::from(self.to_owned());
                let field = &ctx.name;
                let err = "is required";
                let required = ctx
                    .rules
                    .to_owned()
                    .message
                    .map(|v| v.required)
                    .is_true_and(|| {
                        quote! {
                            if self.#name.is_none() {
                                return Err(::prost_validate::Error::new(#field, #err));
                            }
                        }
                    });
                let r#const = rules.r#const.map(|v| {
                    let field = &ctx.name;
                    let err = format!("is not equal to \"{v}\"");
                    quote! {
                        if *#name != #v {
                            return Err(::prost_validate::Error::new(#field, #err));
                        }
                    }
                });
                // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/ltgt.go
                let lte_gte = if let Some(lt) = rules.lt {
                    if let Some(gt) = rules.gt {
                        if lt > gt {
                            let field = &ctx.name;
                            let err = format!("must be inside range ({gt}, {lt})");
                            quote! {
                                if *#name <= #gt || *#name >= #lt {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        } else {
                            let field = &ctx.name;
                            let err = format!("must be outside range [{lt}, {gt}]");
                            quote! {
                                if *#name >= #lt && *#name <= #gt {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        }
                    } else if let Some(gte) = rules.gte {
                        if lt > gte {
                            let field = &ctx.name;
                            let err = format!("must be inside range [{gte}, {lt})");
                            quote! {
                                if *#name < #gte || *#name >= #lt {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        } else {
                            let field = &ctx.name;
                            let err = format!("must be outside range [{gte}, {lt})");
                            quote! {
                                if *#name >= #lt && *#name < #gte {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        }
                    } else {
                        let field = &ctx.name;
                        let err = format!("must be less than {lt}");
                        quote! {
                            if *#name >= #lt {
                                return Err(::prost_validate::Error::new(#field, #err));
                            }
                        }
                    }
                } else if let Some(lte) = rules.lte {
                    if let Some(gt) = rules.gt {
                        if lte > gt {
                            let field = &ctx.name;
                            let err = format!("must be inside range ({gt}, {lte}]");
                            quote! {
                                if *#name <= #gt || *#name > #lte {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        } else {
                            let field = &ctx.name;
                            let err = format!("must be outside range ({lte}, {gt}]");
                            quote! {
                                if *#name > #lte && *#name <= #gt {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        }
                    } else if let Some(gte) = rules.gte {
                        if lte > gte {
                            let field = &ctx.name;
                            let err = format!("must be inside range [{gte}, {lte}]");
                            quote! {
                                if *#name < #gte || *#name > #lte {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        } else {
                            let field = &ctx.name;
                            let err = format!("must be outside range ({lte}, {gte})");
                            quote! {
                                if *#name > #lte && *#name < #gte {
                                    return Err(::prost_validate::Error::new(#field, #err));
                                }
                            }
                        }
                    } else {
                        let field = &ctx.name;
                        let err = format!("must be less or equal to {lte}");
                        quote! {
                            if *#name > #lte {
                                return Err(::prost_validate::Error::new(#field, #err));
                            }
                        }
                    }
                } else if let Some(gt) = rules.gt {
                    let field = &ctx.name;
                    let err = format!("must be greater than {gt}");
                    quote! {
                        if *#name <= #gt {
                            return Err(::prost_validate::Error::new(#field, #err));
                        }
                    }
                } else if let Some(gte) = rules.gte {
                    let field = &ctx.name;
                    let err = format!("must be greater or equal to {gte}");
                    quote! {
                        if *#name < #gte {
                            return Err(::prost_validate::Error::new(#field, #err));
                        }
                    }
                } else {
                    quote! {}
                };
                let r#in = rules.r#in.is_empty().not().then(|| {
                    let v = rules.r#in.to_owned();
                    let field = &ctx.name;
                    let err = format!("must be in {:?}", v);
                    quote! {
                        if ![#(#v),*].contains(#name) {
                            return Err(::prost_validate::Error::new(#field, #err));
                        }
                    }
                });
                let not_in = rules.not_in.is_empty().not().then(|| {
                    let v = rules.not_in.to_owned();
                    let field = &ctx.name;
                    let err = format!("must not be in {:?}", v);
                    quote! {
                        if [#(#v),*].contains(#name) {
                            return Err(::prost_validate::Error::new(#field, #err));
                        }
                    }
                });
                let validate = if self.ignore_empty {
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
                };

                quote! {
                    #required
                    #validate
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

make_number_rules!(Int32Rules, i32, LitInt);
make_number_rules!(Int64Rules, i64, LitInt);
make_number_rules!(UInt32Rules, u32, LitInt);
make_number_rules!(UInt64Rules, u64, LitInt);
make_number_rules!(SInt32Rules, i32, LitInt);
make_number_rules!(SInt64Rules, i64, LitInt);
make_number_rules!(Fixed32Rules, u32, LitInt);
make_number_rules!(Fixed64Rules, u64, LitInt);
make_number_rules!(SFixed32Rules, i32, LitInt);
make_number_rules!(SFixed64Rules, i64, LitInt);
make_number_rules!(FloatRules, f32, LitFloat);
make_number_rules!(DoubleRules, f64, LitFloat);
