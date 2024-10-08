use darling::ast::NestedMeta;
use darling::{Error, FromMeta, Result};
use prost_types::Duration as PbDuration;
use prost_types::Timestamp as PbTimestamp;
use quote::ToTokens;
use std::ops::Deref;
use syn::Expr;

#[derive(Default, Debug, Clone, FromMeta)]
pub struct SecsAndNanosInner {
    pub seconds: Option<i64>,
    pub nanos: Option<i32>,
}

impl From<SecsAndNanos> for PbDuration {
    fn from(SecsAndNanos(SecsAndNanosInner { seconds, nanos }): SecsAndNanos) -> Self {
        PbDuration {
            seconds: seconds.unwrap_or_default(),
            nanos: nanos.unwrap_or_default(),
        }
    }
}

impl From<SecsAndNanos> for PbTimestamp {
    fn from(SecsAndNanos(SecsAndNanosInner { seconds, nanos }): SecsAndNanos) -> Self {
        PbTimestamp {
            seconds: seconds.unwrap_or_default(),
            nanos: nanos.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecsAndNanos(SecsAndNanosInner);

impl Deref for SecsAndNanos {
    type Target = SecsAndNanosInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for SecsAndNanos {
    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        SecsAndNanosInner::from_list(items).map(SecsAndNanos)
    }
    fn from_expr(expr: &Expr) -> Result<Self> {
        match *expr {
            Expr::Lit(ref lit) => Self::from_value(&lit.lit),
            Expr::Group(ref group) => Self::from_expr(&group.expr),
            Expr::Paren(ref paren) => {
                let meta = NestedMeta::parse_meta_list(paren.expr.to_token_stream())
                    .map_err(darling::Error::custom)?;
                Self::from_list(meta.as_slice())
            }
            Expr::Tuple(ref tuple) => {
                let meta = NestedMeta::parse_meta_list(tuple.elems.to_token_stream())
                    .map_err(darling::Error::custom)?;
                Self::from_list(meta.as_slice())
            }
            _ => Err(Error::unexpected_expr_type(expr)),
        }
        .map_err(|e| e.with_span(expr))
    }
}

#[derive(Debug, Default, Clone)]
pub struct SecAndNanosVec(Vec<SecsAndNanos>);

impl Deref for SecAndNanosVec {
    type Target = Vec<SecsAndNanos>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for SecAndNanosVec {
    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        items
            .iter()
            .map(SecsAndNanos::from_nested_meta)
            .collect::<Result<Vec<_>>>()
            .map(SecAndNanosVec)
    }

    fn from_value(value: &syn::Lit) -> Result<Self> {
        let expr_array = syn::ExprArray::from_value(value)?;
        Self::from_expr(&Expr::Array(expr_array))
    }

    fn from_expr(expr: &Expr) -> Result<Self> {
        match expr {
            Expr::Array(expr_array) => expr_array
                .elems
                .iter()
                .map(SecsAndNanos::from_expr)
                .collect::<Result<Vec<_>>>()
                .map(SecAndNanosVec),
            Expr::Lit(expr_lit) => Self::from_value(&expr_lit.lit),
            Expr::Group(g) => Self::from_expr(&g.expr),
            _ => Err(Error::unexpected_expr_type(expr)),
        }
    }
}
