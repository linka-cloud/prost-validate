use prost::Message;
use prost_validate_tests_types::pbjson::cases::*;
use prost_validate_tests_types::pbjson::harness::TestSuite;
use prost_validate_tests_types::{Factory, Validator};
use std::collections::HashMap;
use std::sync::Arc;

pub fn decode_suite(bytes: &[u8]) -> HashMap<String, Factory> {
    let suite = TestSuite::decode(bytes).unwrap();
    let cases: HashMap<String, (pbjson_types::Any, i32)> = suite.test_cases.into_iter().map(|case| {
        let input = case.input.unwrap();
        let name = case.name;
        let failures = case.failures;
        (name, (input, failures))
    }).collect();
    // macro_rules! case {
    //     ($($name:ident: $typ:ident,)*) => {
    //         vec![
    //             $({
    //                 let name = stringify!($name).to_string();
    //                 let (pbjson_types::Any { type_url, value }, failures) = cases.get(name.as_str()).unwrap().clone();
    //                 let input = Arc::new(Box::new($typ::decode(value).unwrap()) as Box<dyn Validator>);
    //                 let failures = Arc::new(failures);
    //                 (
    //                     name,
    //                     Box::new(move || {
    //                         let input = *input.clone();
    //                         let failures = *failures.clone();
    //                         (
    //                             input,
    //                             failures
    //                         )
    //                     }) as Factory,
    //                 )
    //             },)*
    //         ]
    //     }
    // }
    let test_cases = vec![
        {
            let name = "float_none_valid".to_string();
            let (pbjson_types::Any { value, .. }, failures) = cases.get(name.as_str()).unwrap().clone();
            let input = Arc::new(FloatNone::decode(value).unwrap());
            let failures = Arc::new(failures);
            (
                name,
                Box::new(move || {
                    let input = *input.clone();
                    let failures = *failures.clone();
                    (
                        Box::new(input) as Box<dyn Validator>,
                        failures
                    )
                }) as Factory,
            )
        }
    ];
    // let test_cases = case![
    //     float_none_valid: FloatNone,
    //
    //     float_const_valid: FloatConst,
    //     float_const_invalid: FloatConst,
    //
    //     float_in_valid: FloatIn,
    //     float_in_invalid: FloatIn,
    //
    //     float_not_in_valid: FloatNotIn,
    //     float_not_in_invalid: FloatNotIn,
    //
    //     float_lt_valid: FloatLt,
    //     float_lt_invalid_equal: FloatLt,
    //     float_lt_invalid: FloatLt,
    //
    //     float_lte_valid: FloatLte,
    //     float_lte_valid_equal: FloatLte,
    //     float_lte_invalid: FloatLte,
    //
    //     float_gt_valid: FloatGt,
    //     float_gt_invalid_equal: FloatGt,
    //     float_gt_invalid: FloatGt,
    //
    //     float_gte_valid: FloatGte,
    //     float_gte_valid_equal: FloatGte,
    //     float_gte_invalid: FloatGte,
    //
    //     float_gt_lt_valid: FloatGtlt,
    //     float_gt_lt_invalid_above: FloatGtlt,
    //     float_gt_lt_invalid_below: FloatGtlt,
    //     float_gt_lt_invalid_max: FloatGtlt,
    //     float_gt_lt_invalid_min: FloatGtlt,
    //
    //     float_exclusive_gt_lt_valid_above: FloatExLtgt,
    //     float_exclusive_gt_lt_valid_below: FloatExLtgt,
    //     float_exclusive_gt_lt_invalid: FloatExLtgt,
    //     float_exclusive_gt_lt_invalid_max: FloatExLtgt,
    //     float_exclusive_gt_lt_invalid_min: FloatExLtgt,
    //
    //     float_gte_lte_valid: FloatGtelte,
    //     float_gte_lte_valid_max: FloatGtelte,
    //     float_gte_lte_valid_min: FloatGtelte,
    //     float_gte_lte_invalid_above: FloatGtelte,
    //     float_gte_lte_invalid_below: FloatGtelte,
    //
    //     float_exclusive_gte_lte_valid_above: FloatExGtelte,
    //     float_exclusive_gte_lte_valid_below: FloatExGtelte,
    //     float_exclusive_gte_lte_valid_max: FloatExGtelte,
    //     float_exclusive_gte_lte_valid_min: FloatExGtelte,
    //     float_exclusive_gte_lte_invalid: FloatExGtelte,
    //     float_ignore_empty_gte_lte_valid: FloatIgnore,
    //
    //     double_none_valid: DoubleNone,
    //
    //     double_const_valid: DoubleConst,
    //     double_const_invalid: DoubleConst,
    //
    //     double_in_valid: DoubleIn,
    //     double_in_invalid: DoubleIn,
    //
    //     double_not_in_valid: DoubleNotIn,
    //     double_not_in_invalid: DoubleNotIn,
    //
    //     double_lt_valid: DoubleLt,
    //     double_lt_invalid_equal: DoubleLt,
    //     double_lt_invalid: DoubleLt,
    //
    //     double_lte_valid: DoubleLte,
    //     double_lte_valid_equal: DoubleLte,
    //     double_lte_invalid: DoubleLte,
    //
    //     double_gt_valid: DoubleGt,
    //     double_gt_invalid_equal: DoubleGt,
    //     double_gt_invalid: DoubleGt,
    //
    //     double_gte_valid: DoubleGte,
    //     double_gte_valid_equal: DoubleGte,
    //     double_gte_invalid: DoubleGte,
    //
    //     double_gt_lt_valid: DoubleGtlt,
    //     double_gt_lt_invalid_above: DoubleGtlt,
    //     double_gt_lt_invalid_below: DoubleGtlt,
    //     double_gt_lt_invalid_max: DoubleGtlt,
    //     double_gt_lt_invalid_min: DoubleGtlt,
    //
    //     double_exclusive_gt_lt_valid_above: DoubleExLtgt,
    //     double_exclusive_gt_lt_valid_below: DoubleExLtgt,
    //     double_exclusive_gt_lt_invalid: DoubleExLtgt,
    //     double_exclusive_gt_lt_invalid_max: DoubleExLtgt,
    //     double_exclusive_gt_lt_invalid_min: DoubleExLtgt,
    //
    //     double_gte_lte_valid: DoubleGtelte,
    //     double_gte_lte_valid_max: DoubleGtelte,
    //     double_gte_lte_valid_min: DoubleGtelte,
    //     double_gte_lte_invalid_above: DoubleGtelte,
    //     double_gte_lte_invalid_below: DoubleGtelte,
    //
    //     double_exclusive_gte_lte_valid_above: DoubleExGtelte,
    //     double_exclusive_gte_lte_valid_below: DoubleExGtelte,
    //     double_exclusive_gte_lte_valid_max: DoubleExGtelte,
    //     double_exclusive_gte_lte_valid_min: DoubleExGtelte,
    //     double_exclusive_gte_lte_invalid: DoubleExGtelte,
    //
    //     double_ignore_empty_gte_lte_valid: DoubleIgnore,
    //
    //     int32_none_valid: Int32None,
    //
    //     int32_const_valid: Int32Const,
    //     int32_const_invalid: Int32Const,
    //
    //     int32_in_valid: Int32In,
    //     int32_in_invalid: Int32In,
    //
    //     int32_not_in_valid: Int32NotIn,
    //     int32_not_in_invalid: Int32NotIn,
    //
    //     int32_lt_valid: Int32Lt,
    //     int32_lt_invalid_equal: Int32Lt,
    //     int32_lt_invalid: Int32Lt,
    //
    //     int32_lte_valid: Int32Lte,
    //     int32_lte_valid_equal: Int32Lte,
    //     int32_lte_invalid: Int32Lte,
    //
    //     int32_gt_valid: Int32Gt,
    //     int32_gt_invalid_equal: Int32Gt,
    //     int32_gt_invalid: Int32Gt,
    //
    //     int32_gte_valid: Int32Gte,
    //     int32_gte_valid_equal: Int32Gte,
    //     int32_gte_invalid: Int32Gte,
    //
    //     int32_gt_lt_valid: Int32Gtlt,
    //     int32_gt_lt_invalid_above: Int32Gtlt,
    //     int32_gt_lt_invalid_below: Int32Gtlt,
    //     int32_gt_lt_invalid_max: Int32Gtlt,
    //     int32_gt_lt_invalid_min: Int32Gtlt,
    //
    //     int32_exclusive_gt_lt_valid_above: Int32ExLtgt,
    //     int32_exclusive_gt_lt_valid_below: Int32ExLtgt,
    //     int32_exclusive_gt_lt_invalid: Int32ExLtgt,
    //     int32_exclusive_gt_lt_invalid_max: Int32ExLtgt,
    //     int32_exclusive_gt_lt_invalid_min: Int32ExLtgt,
    //
    //     int32_gte_lte_valid: Int32Gtelte,
    //     int32_gte_lte_valid_max: Int32Gtelte,
    //     int32_gte_lte_valid_min: Int32Gtelte,
    //     int32_gte_lte_invalid_above: Int32Gtelte,
    //     int32_gte_lte_invalid_below: Int32Gtelte,
    //
    //     int32_exclusive_gte_lte_valid_above: Int32ExGtelte,
    //     int32_exclusive_gte_lte_valid_below: Int32ExGtelte,
    //     int32_exclusive_gte_lte_valid_max: Int32ExGtelte,
    //     int32_exclusive_gte_lte_valid_min: Int32ExGtelte,
    //     int32_exclusive_gte_lte_invalid: Int32ExGtelte,
    //
    //     int32_ignore_empty_gte_lte_valid: Int32Ignore,
    //
    //     int64_none_valid: Int64None,
    //
    //     int64_const_valid: Int64Const,
    //     int64_const_invalid: Int64Const,
    //
    //     int64_in_valid: Int64In,
    //     int64_in_invalid: Int64In,
    //
    //     int64_not_in_valid: Int64NotIn,
    //     int64_not_in_invalid: Int64NotIn,
    //
    //     int64_lt_valid: Int64Lt,
    //     int64_lt_invalid_equal: Int64Lt,
    //     int64_lt_invalid: Int64Lt,
    //
    //     int64_lte_valid: Int64Lte,
    //     int64_lte_valid_equal: Int64Lte,
    //     int64_lte_invalid: Int64Lte,
    //
    //     int64_gt_valid: Int64Gt,
    //     int64_gt_invalid_equal: Int64Gt,
    //     int64_gt_invalid: Int64Gt,
    //
    //     int64_gte_valid: Int64Gte,
    //     int64_gte_valid_equal: Int64Gte,
    //     int64_gte_invalid: Int64Gte,
    //
    //     int64_gt_lt_valid: Int64Gtlt,
    //     int64_gt_lt_invalid_above: Int64Gtlt,
    //     int64_gt_lt_invalid_below: Int64Gtlt,
    //     int64_gt_lt_invalid_max: Int64Gtlt,
    //     int64_gt_lt_invalid_min: Int64Gtlt,
    //
    //     int64_exclusive_gt_lt_valid_above: Int64ExLtgt,
    //     int64_exclusive_gt_lt_valid_below: Int64ExLtgt,
    //     int64_exclusive_gt_lt_invalid: Int64ExLtgt,
    //     int64_exclusive_gt_lt_invalid_max: Int64ExLtgt,
    //     int64_exclusive_gt_lt_invalid_min: Int64ExLtgt,
    //
    //     int64_gte_lte_valid: Int64Gtelte,
    //     int64_gte_lte_valid_max: Int64Gtelte,
    //     int64_gte_lte_valid_min: Int64Gtelte,
    //     int64_gte_lte_invalid_above: Int64Gtelte,
    //     int64_gte_lte_invalid_below: Int64Gtelte,
    //
    //     int64_exclusive_gte_lte_valid_above: Int64ExGtelte,
    //     int64_exclusive_gte_lte_valid_below: Int64ExGtelte,
    //     int64_exclusive_gte_lte_valid_max: Int64ExGtelte,
    //     int64_exclusive_gte_lte_valid_min: Int64ExGtelte,
    //     int64_exclusive_gte_lte_invalid: Int64ExGtelte,
    //
    //     int64_ignore_empty_gte_lte_valid: Int64Ignore,
    //
    //     int64_optional_lte_valid: Int64LteOptional,
    //     int64_optional_lte_valid_equal: Int64LteOptional,
    //     int64_optional_lte_valid_unset: Int64LteOptional,
    //
    //     uint32_none_valid: UInt32None,
    //
    //     uint32_const_valid: UInt32Const,
    //     uint32_const_invalid: UInt32Const,
    //
    //     uint32_in_valid: UInt32In,
    //     uint32_in_invalid: UInt32In,
    //
    //     uint32_not_in_valid: UInt32NotIn,
    //     uint32_not_in_invalid: UInt32NotIn,
    //
    //     uint32_lt_valid: UInt32Lt,
    //     uint32_lt_invalid_equal: UInt32Lt,
    //     uint32_lt_invalid: UInt32Lt,
    //
    //     uint32_lte_valid: UInt32Lte,
    //     uint32_lte_valid_equal: UInt32Lte,
    //     uint32_lte_invalid: UInt32Lte,
    //
    //     uint32_gt_valid: UInt32Gt,
    //     uint32_gt_invalid_equal: UInt32Gt,
    //     uint32_gt_invalid: UInt32Gt,
    //
    //     uint32_gte_valid: UInt32Gte,
    //     uint32_gte_valid_equal: UInt32Gte,
    //     uint32_gte_invalid: UInt32Gte,
    //
    //     uint32_gt_lt_valid: UInt32Gtlt,
    //     uint32_gt_lt_invalid_above: UInt32Gtlt,
    //     uint32_gt_lt_invalid_below: UInt32Gtlt,
    //     uint32_gt_lt_invalid_max: UInt32Gtlt,
    //     uint32_gt_lt_invalid_min: UInt32Gtlt,
    //
    //     uint32_exclusive_gt_lt_valid_above: UInt32ExLtgt,
    //     uint32_exclusive_gt_lt_valid_below: UInt32ExLtgt,
    //     uint32_exclusive_gt_lt_invalid: UInt32ExLtgt,
    //     uint32_exclusive_gt_lt_invalid_max: UInt32ExLtgt,
    //     uint32_exclusive_gt_lt_invalid_min: UInt32ExLtgt,
    //
    //     uint32_gte_lte_valid: UInt32Gtelte,
    //     uint32_gte_lte_valid_max: UInt32Gtelte,
    //     uint32_gte_lte_valid_min: UInt32Gtelte,
    //     uint32_gte_lte_invalid_above: UInt32Gtelte,
    //     uint32_gte_lte_invalid_below: UInt32Gtelte,
    //
    //     uint32_exclusive_gte_lte_valid_above: UInt32ExGtelte,
    //     uint32_exclusive_gte_lte_valid_below: UInt32ExGtelte,
    //     uint32_exclusive_gte_lte_valid_max: UInt32ExGtelte,
    //     uint32_exclusive_gte_lte_valid_min: UInt32ExGtelte,
    //     uint32_exclusive_gte_lte_invalid: UInt32ExGtelte,
    //
    //     uint32_ignore_empty_gte_lte_valid: UInt32Ignore,
    //
    //     uint64_none_valid: UInt64None,
    //
    //     uint64_const_valid: UInt64Const,
    //     uint64_const_invalid: UInt64Const,
    //
    //     uint64_in_valid: UInt64In,
    //     uint64_in_invalid: UInt64In,
    //
    //     uint64_not_in_valid: UInt64NotIn,
    //     uint64_not_in_invalid: UInt64NotIn,
    //
    //     uint64_lt_valid: UInt64Lt,
    //     uint64_lt_invalid_equal: UInt64Lt,
    //     uint64_lt_invalid: UInt64Lt,
    //
    //     uint64_lte_valid: UInt64Lte,
    //     uint64_lte_valid_equal: UInt64Lte,
    //     uint64_lte_invalid: UInt64Lte,
    //
    //     uint64_gt_valid: UInt64Gt,
    //     uint64_gt_invalid_equal: UInt64Gt,
    //     uint64_gt_invalid: UInt64Gt,
    //
    //     uint64_gte_valid: UInt64Gte,
    //     uint64_gte_valid_equal: UInt64Gte,
    //     uint64_gte_invalid: UInt64Gte,
    //
    //     uint64_gt_lt_valid: UInt64Gtlt,
    //     uint64_gt_lt_invalid_above: UInt64Gtlt,
    //     uint64_gt_lt_invalid_below: UInt64Gtlt,
    //     uint64_gt_lt_invalid_max: UInt64Gtlt,
    //     uint64_gt_lt_invalid_min: UInt64Gtlt,
    //
    //     uint64_exclusive_gt_lt_valid_above: UInt64ExLtgt,
    //     uint64_exclusive_gt_lt_valid_below: UInt64ExLtgt,
    //     uint64_exclusive_gt_lt_invalid: UInt64ExLtgt,
    //     uint64_exclusive_gt_lt_invalid_max: UInt64ExLtgt,
    //     uint64_exclusive_gt_lt_invalid_min: UInt64ExLtgt,
    //
    //     uint64_gte_lte_valid: UInt64Gtelte,
    //     uint64_gte_lte_valid_max: UInt64Gtelte,
    //     uint64_gte_lte_valid_min: UInt64Gtelte,
    //     uint64_gte_lte_invalid_above: UInt64Gtelte,
    //     uint64_gte_lte_invalid_below: UInt64Gtelte,
    //
    //     uint64_exclusive_gte_lte_valid_above: UInt64ExGtelte,
    //     uint64_exclusive_gte_lte_valid_below: UInt64ExGtelte,
    //     uint64_exclusive_gte_lte_valid_max: UInt64ExGtelte,
    //     uint64_exclusive_gte_lte_valid_min: UInt64ExGtelte,
    //     uint64_exclusive_gte_lte_invalid: UInt64ExGtelte,
    //
    //     uint64_ignore_empty_gte_lte_valid: UInt64Ignore,
    //
    //     sint32_none_valid: SInt32None,
    //
    //     sint32_const_valid: SInt32Const,
    //     sint32_const_invalid: SInt32Const,
    //
    //     sint32_in_valid: SInt32In,
    //     sint32_in_invalid: SInt32In,
    //
    //     sint32_not_in_valid: SInt32NotIn,
    //     sint32_not_in_invalid: SInt32NotIn,
    //
    //     sint32_lt_valid: SInt32Lt,
    //     sint32_lt_invalid_equal: SInt32Lt,
    //     sint32_lt_invalid: SInt32Lt,
    //
    //     sint32_lte_valid: SInt32Lte,
    //     sint32_lte_valid_equal: SInt32Lte,
    //     sint32_lte_invalid: SInt32Lte,
    //
    //     sint32_gt_valid: SInt32Gt,
    //     sint32_gt_invalid_equal: SInt32Gt,
    //     sint32_gt_invalid: SInt32Gt,
    //
    //     sint32_gte_valid: SInt32Gte,
    //     sint32_gte_valid_equal: SInt32Gte,
    //     sint32_gte_invalid: SInt32Gte,
    //
    //     sint32_gt_lt_valid: SInt32Gtlt,
    //     sint32_gt_lt_invalid_above: SInt32Gtlt,
    //     sint32_gt_lt_invalid_below: SInt32Gtlt,
    //     sint32_gt_lt_invalid_max: SInt32Gtlt,
    //     sint32_gt_lt_invalid_min: SInt32Gtlt,
    //
    //     sint32_exclusive_gt_lt_valid_above: SInt32ExLtgt,
    //     sint32_exclusive_gt_lt_valid_below: SInt32ExLtgt,
    //     sint32_exclusive_gt_lt_invalid: SInt32ExLtgt,
    //     sint32_exclusive_gt_lt_invalid_max: SInt32ExLtgt,
    //     sint32_exclusive_gt_lt_invalid_min: SInt32ExLtgt,
    //
    //     sint32_gte_lte_valid: SInt32Gtelte,
    //     sint32_gte_lte_valid_max: SInt32Gtelte,
    //     sint32_gte_lte_valid_min: SInt32Gtelte,
    //     sint32_gte_lte_invalid_above: SInt32Gtelte,
    //     sint32_gte_lte_invalid_below: SInt32Gtelte,
    //
    //     sint32_exclusive_gte_lte_valid_above: SInt32ExGtelte,
    //     sint32_exclusive_gte_lte_valid_below: SInt32ExGtelte,
    //     sint32_exclusive_gte_lte_valid_max: SInt32ExGtelte,
    //     sint32_exclusive_gte_lte_valid_min: SInt32ExGtelte,
    //     sint32_exclusive_gte_lte_invalid: SInt32ExGtelte,
    //
    //     sint32_ignore_empty_gte_lte_valid: SInt32Ignore,
    //
    //     sint64_none_valid: SInt64None,
    //
    //     sint64_const_valid: SInt64Const,
    //     sint64_const_invalid: SInt64Const,
    //
    //     sint64_in_valid: SInt64In,
    //     sint64_in_invalid: SInt64In,
    //
    //     sint64_not_in_valid: SInt64NotIn,
    //     sint64_not_in_invalid: SInt64NotIn,
    //
    //     sint64_lt_valid: SInt64Lt,
    //     sint64_lt_invalid_equal: SInt64Lt,
    //     sint64_lt_invalid: SInt64Lt,
    //
    //     sint64_lte_valid: SInt64Lte,
    //     sint64_lte_valid_equal: SInt64Lte,
    //     sint64_lte_invalid: SInt64Lte,
    //
    //     sint64_gt_valid: SInt64Gt,
    //     sint64_gt_invalid_equal: SInt64Gt,
    //     sint64_gt_invalid: SInt64Gt,
    //
    //     sint64_gte_valid: SInt64Gte,
    //     sint64_gte_valid_equal: SInt64Gte,
    //     sint64_gte_invalid: SInt64Gte,
    //
    //     sint64_gt_lt_valid: SInt64Gtlt,
    //     sint64_gt_lt_invalid_above: SInt64Gtlt,
    //     sint64_gt_lt_invalid_below: SInt64Gtlt,
    //     sint64_gt_lt_invalid_max: SInt64Gtlt,
    //     sint64_gt_lt_invalid_min: SInt64Gtlt,
    //
    //     sint64_exclusive_gt_lt_valid_above: SInt64ExLtgt,
    //     sint64_exclusive_gt_lt_valid_below: SInt64ExLtgt,
    //     sint64_exclusive_gt_lt_invalid: SInt64ExLtgt,
    //     sint64_exclusive_gt_lt_invalid_max: SInt64ExLtgt,
    //     sint64_exclusive_gt_lt_invalid_min: SInt64ExLtgt,
    //
    //     sint64_gte_lte_valid: SInt64Gtelte,
    //     sint64_gte_lte_valid_max: SInt64Gtelte,
    //     sint64_gte_lte_valid_min: SInt64Gtelte,
    //     sint64_gte_lte_invalid_above: SInt64Gtelte,
    //     sint64_gte_lte_invalid_below: SInt64Gtelte,
    //
    //     sint64_exclusive_gte_lte_valid_above: SInt64ExGtelte,
    //     sint64_exclusive_gte_lte_valid_below: SInt64ExGtelte,
    //     sint64_exclusive_gte_lte_valid_max: SInt64ExGtelte,
    //     sint64_exclusive_gte_lte_valid_min: SInt64ExGtelte,
    //     sint64_exclusive_gte_lte_invalid: SInt64ExGtelte,
    //
    //     sint64_ignore_empty_gte_lte_valid: SInt64Ignore,
    //
    //     fixed32_none_valid: Fixed32None,
    //
    //     fixed32_const_valid: Fixed32Const,
    //     fixed32_const_invalid: Fixed32Const,
    //
    //     fixed32_in_valid: Fixed32In,
    //     fixed32_in_invalid: Fixed32In,
    //
    //     fixed32_not_in_valid: Fixed32NotIn,
    //     fixed32_not_in_invalid: Fixed32NotIn,
    //
    //     fixed32_lt_valid: Fixed32Lt,
    //     fixed32_lt_invalid_equal: Fixed32Lt,
    //     fixed32_lt_invalid: Fixed32Lt,
    //
    //     fixed32_lte_valid: Fixed32Lte,
    //     fixed32_lte_valid_equal: Fixed32Lte,
    //     fixed32_lte_invalid: Fixed32Lte,
    //
    //     fixed32_gt_valid: Fixed32Gt,
    //     fixed32_gt_invalid_equal: Fixed32Gt,
    //     fixed32_gt_invalid: Fixed32Gt,
    //
    //     fixed32_gte_valid: Fixed32Gte,
    //     fixed32_gte_valid_equal: Fixed32Gte,
    //     fixed32_gte_invalid: Fixed32Gte,
    //
    //     fixed32_gt_lt_valid: Fixed32Gtlt,
    //     fixed32_gt_lt_invalid_above: Fixed32Gtlt,
    //     fixed32_gt_lt_invalid_below: Fixed32Gtlt,
    //     fixed32_gt_lt_invalid_max: Fixed32Gtlt,
    //     fixed32_gt_lt_invalid_min: Fixed32Gtlt,
    //
    //     fixed32_exclusive_gt_lt_valid_above: Fixed32ExLtgt,
    //     fixed32_exclusive_gt_lt_valid_below: Fixed32ExLtgt,
    //     fixed32_exclusive_gt_lt_invalid: Fixed32ExLtgt,
    //     fixed32_exclusive_gt_lt_invalid_max: Fixed32ExLtgt,
    //     fixed32_exclusive_gt_lt_invalid_min: Fixed32ExLtgt,
    //
    //     fixed32_gte_lte_valid: Fixed32Gtelte,
    //     fixed32_gte_lte_valid_max: Fixed32Gtelte,
    //     fixed32_gte_lte_valid_min: Fixed32Gtelte,
    //     fixed32_gte_lte_invalid_above: Fixed32Gtelte,
    //     fixed32_gte_lte_invalid_below: Fixed32Gtelte,
    //
    //     fixed32_exclusive_gte_lte_valid_above: Fixed32ExGtelte,
    //     fixed32_exclusive_gte_lte_valid_below: Fixed32ExGtelte,
    //     fixed32_exclusive_gte_lte_valid_max: Fixed32ExGtelte,
    //     fixed32_exclusive_gte_lte_valid_min: Fixed32ExGtelte,
    //     fixed32_exclusive_gte_lte_invalid: Fixed32ExGtelte,
    //
    //     fixed32_ignore_empty_gte_lte_valid: Fixed32Ignore,
    //
    //     fixed64_none_valid: Fixed64None,
    //
    //     fixed64_const_valid: Fixed64Const,
    //     fixed64_const_invalid: Fixed64Const,
    //
    //     fixed64_in_valid: Fixed64In,
    //     fixed64_in_invalid: Fixed64In,
    //
    //     fixed64_not_in_valid: Fixed64NotIn,
    //     fixed64_not_in_invalid: Fixed64NotIn,
    //
    //     fixed64_lt_valid: Fixed64Lt,
    //     fixed64_lt_invalid_equal: Fixed64Lt,
    //     fixed64_lt_invalid: Fixed64Lt,
    //
    //     fixed64_lte_valid: Fixed64Lte,
    //     fixed64_lte_valid_equal: Fixed64Lte,
    //     fixed64_lte_invalid: Fixed64Lte,
    //
    //     fixed64_gt_valid: Fixed64Gt,
    //     fixed64_gt_invalid_equal: Fixed64Gt,
    //     fixed64_gt_invalid: Fixed64Gt,
    //
    //     fixed64_gte_valid: Fixed64Gte,
    //     fixed64_gte_valid_equal: Fixed64Gte,
    //     fixed64_gte_invalid: Fixed64Gte,
    //
    //     fixed64_gt_lt_valid: Fixed64Gtlt,
    //     fixed64_gt_lt_invalid_above: Fixed64Gtlt,
    //     fixed64_gt_lt_invalid_below: Fixed64Gtlt,
    //     fixed64_gt_lt_invalid_max: Fixed64Gtlt,
    //     fixed64_gt_lt_invalid_min: Fixed64Gtlt,
    //
    //     fixed64_exclusive_gt_lt_valid_above: Fixed64ExLtgt,
    //     fixed64_exclusive_gt_lt_valid_below: Fixed64ExLtgt,
    //     fixed64_exclusive_gt_lt_invalid: Fixed64ExLtgt,
    //     fixed64_exclusive_gt_lt_invalid_max: Fixed64ExLtgt,
    //     fixed64_exclusive_gt_lt_invalid_min: Fixed64ExLtgt,
    //
    //     fixed64_gte_lte_valid: Fixed64Gtelte,
    //     fixed64_gte_lte_valid_max: Fixed64Gtelte,
    //     fixed64_gte_lte_valid_min: Fixed64Gtelte,
    //     fixed64_gte_lte_invalid_above: Fixed64Gtelte,
    //     fixed64_gte_lte_invalid_below: Fixed64Gtelte,
    //
    //     fixed64_exclusive_gte_lte_valid_above: Fixed64ExGtelte,
    //     fixed64_exclusive_gte_lte_valid_below: Fixed64ExGtelte,
    //     fixed64_exclusive_gte_lte_valid_max: Fixed64ExGtelte,
    //     fixed64_exclusive_gte_lte_valid_min: Fixed64ExGtelte,
    //     fixed64_exclusive_gte_lte_invalid: Fixed64ExGtelte,
    //
    //     fixed64_ignore_empty_gte_lte_valid: Fixed64Ignore,
    //
    //     sfixed32_none_valid: SFixed32None,
    //
    //     sfixed32_const_valid: SFixed32Const,
    //     sfixed32_const_invalid: SFixed32Const,
    //
    //     sfixed32_in_valid: SFixed32In,
    //     sfixed32_in_invalid: SFixed32In,
    //
    //     sfixed32_not_in_valid: SFixed32NotIn,
    //     sfixed32_not_in_invalid: SFixed32NotIn,
    //
    //     sfixed32_lt_valid: SFixed32Lt,
    //     sfixed32_lt_invalid_equal: SFixed32Lt,
    //     sfixed32_lt_invalid: SFixed32Lt,
    //
    //     sfixed32_lte_valid: SFixed32Lte,
    //     sfixed32_lte_valid_equal: SFixed32Lte,
    //     sfixed32_lte_invalid: SFixed32Lte,
    //
    //     sfixed32_gt_valid: SFixed32Gt,
    //     sfixed32_gt_invalid_equal: SFixed32Gt,
    //     sfixed32_gt_invalid: SFixed32Gt,
    //
    //     sfixed32_gte_valid: SFixed32Gte,
    //     sfixed32_gte_valid_equal: SFixed32Gte,
    //     sfixed32_gte_invalid: SFixed32Gte,
    //
    //     sfixed32_gt_lt_valid: SFixed32Gtlt,
    //     sfixed32_gt_lt_invalid_above: SFixed32Gtlt,
    //     sfixed32_gt_lt_invalid_below: SFixed32Gtlt,
    //     sfixed32_gt_lt_invalid_max: SFixed32Gtlt,
    //     sfixed32_gt_lt_invalid_min: SFixed32Gtlt,
    //
    //     sfixed32_exclusive_gt_lt_valid_above: SFixed32ExLtgt,
    //     sfixed32_exclusive_gt_lt_valid_below: SFixed32ExLtgt,
    //     sfixed32_exclusive_gt_lt_invalid: SFixed32ExLtgt,
    //     sfixed32_exclusive_gt_lt_invalid_max: SFixed32ExLtgt,
    //     sfixed32_exclusive_gt_lt_invalid_min: SFixed32ExLtgt,
    //
    //     sfixed32_gte_lte_valid: SFixed32Gtelte,
    //     sfixed32_gte_lte_valid_max: SFixed32Gtelte,
    //     sfixed32_gte_lte_valid_min: SFixed32Gtelte,
    //     sfixed32_gte_lte_invalid_above: SFixed32Gtelte,
    //     sfixed32_gte_lte_invalid_below: SFixed32Gtelte,
    //
    //     sfixed32_exclusive_gte_lte_valid_above: SFixed32ExGtelte,
    //     sfixed32_exclusive_gte_lte_valid_below: SFixed32ExGtelte,
    //     sfixed32_exclusive_gte_lte_valid_max: SFixed32ExGtelte,
    //     sfixed32_exclusive_gte_lte_valid_min: SFixed32ExGtelte,
    //     sfixed32_exclusive_gte_lte_invalid: SFixed32ExGtelte,
    //
    //     sfixed32_ignore_empty_gte_lte_valid: SFixed32Ignore,
    //
    //     sfixed64_none_valid: SFixed64None,
    //
    //     sfixed64_const_valid: SFixed64Const,
    //     sfixed64_const_invalid: SFixed64Const,
    //
    //     sfixed64_in_valid: SFixed64In,
    //     sfixed64_in_invalid: SFixed64In,
    //
    //     sfixed64_not_in_valid: SFixed64NotIn,
    //     sfixed64_not_in_invalid: SFixed64NotIn,
    //
    //     sfixed64_lt_valid: SFixed64Lt,
    //     sfixed64_lt_invalid_equal: SFixed64Lt,
    //     sfixed64_lt_invalid: SFixed64Lt,
    //
    //     sfixed64_lte_valid: SFixed64Lte,
    //     sfixed64_lte_valid_equal: SFixed64Lte,
    //     sfixed64_lte_invalid: SFixed64Lte,
    //
    //     sfixed64_gt_valid: SFixed64Gt,
    //     sfixed64_gt_invalid_equal: SFixed64Gt,
    //     sfixed64_gt_invalid: SFixed64Gt,
    //
    //     sfixed64_gte_valid: SFixed64Gte,
    //     sfixed64_gte_valid_equal: SFixed64Gte,
    //     sfixed64_gte_invalid: SFixed64Gte,
    //
    //     sfixed64_gt_lt_valid: SFixed64Gtlt,
    //     sfixed64_gt_lt_invalid_above: SFixed64Gtlt,
    //     sfixed64_gt_lt_invalid_below: SFixed64Gtlt,
    //     sfixed64_gt_lt_invalid_max: SFixed64Gtlt,
    //     sfixed64_gt_lt_invalid_min: SFixed64Gtlt,
    //
    //     sfixed64_exclusive_gt_lt_valid_above: SFixed64ExLtgt,
    //     sfixed64_exclusive_gt_lt_valid_below: SFixed64ExLtgt,
    //     sfixed64_exclusive_gt_lt_invalid: SFixed64ExLtgt,
    //     sfixed64_exclusive_gt_lt_invalid_max: SFixed64ExLtgt,
    //     sfixed64_exclusive_gt_lt_invalid_min: SFixed64ExLtgt,
    //
    //     sfixed64_gte_lte_valid: SFixed64Gtelte,
    //     sfixed64_gte_lte_valid_max: SFixed64Gtelte,
    //     sfixed64_gte_lte_valid_min: SFixed64Gtelte,
    //     sfixed64_gte_lte_invalid_above: SFixed64Gtelte,
    //     sfixed64_gte_lte_invalid_below: SFixed64Gtelte,
    //
    //     sfixed64_exclusive_gte_lte_valid_above: SFixed64ExGtelte,
    //     sfixed64_exclusive_gte_lte_valid_below: SFixed64ExGtelte,
    //     sfixed64_exclusive_gte_lte_valid_max: SFixed64ExGtelte,
    //     sfixed64_exclusive_gte_lte_valid_min: SFixed64ExGtelte,
    //     sfixed64_exclusive_gte_lte_invalid: SFixed64ExGtelte,
    //
    //     sfixed64_ignore_empty_gte_lte_valid: SFixed64Ignore,
    //
    //     bool_none_valid: BoolNone,
    //
    //     bool_const_true_valid: BoolConstTrue,
    //     bool_const_true_invalid: BoolConstTrue,
    //     bool_const_false_valid: BoolConstFalse,
    //     bool_const_false_invalid: BoolConstFalse,
    //
    //     string_none_valid: StringNone,
    //     string_const_valid: StringConst,
    //     string_const_invalid: StringConst,
    //
    //     string_in_valid: StringIn,
    //     string_in_invalid: StringIn,
    //
    //     string_not_in_valid: StringNotIn,
    //     string_not_in_invalid: StringNotIn,
    //
    //     string_len_valid: StringLen,
    //     string_len_valid_multibyte: StringLen,
    //
    //     string_len_invalid_lt: StringLen,
    //     string_len_invalid_gt: StringLen,
    //     string_len_invalid_multibyte: StringLen,
    //
    //     string_min_len_valid: StringMinLen,
    //     string_min_len_valid_min: StringMinLen,
    //     string_min_len_invalid: StringMinLen,
    //     string_min_len_invalid_multibyte: StringMinLen,
    //
    //     string_max_len_valid: StringMaxLen,
    //     string_max_len_valid_max: StringMaxLen,
    //     string_max_len_valid_multibyte: StringMaxLen,
    //     string_max_len_invalid: StringMaxLen,
    //
    //     string_min_max_len_valid: StringMinMaxLen,
    //     string_min_max_len_valid_min: StringMinMaxLen,
    //     string_min_max_len_valid_max: StringMinMaxLen,
    //     string_min_max_len_valid_multibyte: StringMinMaxLen,
    //     string_min_max_len_invalid_below: StringMinMaxLen,
    //     string_min_max_len_invalid_above: StringMinMaxLen,
    //
    //     string_equal_min_max_len_valid: StringEqualMinMaxLen,
    //     string_equal_min_max_len_invalid: StringEqualMinMaxLen,
    //
    //     string_len_bytes_valid: StringLenBytes,
    //     string_len_bytes_invalid_lt: StringLenBytes,
    //     string_len_bytes_invalid_gt: StringLenBytes,
    //     string_len_bytes_invalid_multibyte: StringLenBytes,
    //
    //     string_min_bytes_valid: StringMinBytes,
    //     string_min_bytes_valid_min: StringMinBytes,
    //     string_min_bytes_valid_multibyte: StringMinBytes,
    //     string_min_bytes_invalid: StringMinBytes,
    //
    //     string_max_bytes_valid: StringMaxBytes,
    //     string_max_bytes_valid_max: StringMaxBytes,
    //     string_max_bytes_invalid: StringMaxBytes,
    //     string_max_bytes_invalid_multibyte: StringMaxBytes,
    //
    //     string_min_max_bytes_valid: StringMinMaxBytes,
    //     string_min_max_bytes_valid_min: StringMinMaxBytes,
    //     string_min_max_bytes_valid_max: StringMinMaxBytes,
    //     string_min_max_bytes_valid_multibyte: StringMinMaxBytes,
    //     string_min_max_bytes_invalid_below: StringMinMaxBytes,
    //     string_min_max_bytes_invalid_above: StringMinMaxBytes,
    //
    //     string_equal_min_max_bytes_valid: StringEqualMinMaxBytes,
    //     string_equal_min_max_bytes_invalid: StringEqualMinMaxBytes,
    //
    //     string_pattern_valid: StringPattern,
    //     string_pattern_invalid: StringPattern,
    //     string_pattern_invalid_empty: StringPattern,
    //     string_pattern_invalid_null: StringPattern,
    //     string_pattern_escapes_valid: StringPatternEscapes,
    //     string_pattern_escapes_invalid: StringPatternEscapes,
    //     string_pattern_escapes_invalid_empty: StringPatternEscapes,
    //
    //     string_prefix_valid: StringPrefix,
    //     string_prefix_valid_only: StringPrefix,
    //     string_prefix_invalid: StringPrefix,
    //     string_prefix_invalid_case_sensitive: StringPrefix,
    //
    //     string_contains_valid: StringContains,
    //     string_contains_valid_only: StringContains,
    //     string_contains_invalid: StringContains,
    //     string_contains_invalid_case_sensitive: StringContains,
    //
    //     string_not_contains_valid: StringNotContains,
    //     string_not_contains_valid_case_sensitive: StringNotContains,
    //     string_not_contains_invalid: StringNotContains,
    //     string_not_contains_invalid_equal: StringNotContains,
    //
    //     string_suffix_valid: StringSuffix,
    //     string_suffix_valid_only: StringSuffix,
    //     string_suffix_invalid: StringSuffix,
    //     string_suffix_invalid_case_sensitive: StringSuffix,
    //
    //     string_email_valid: StringEmail,
    //     string_email_valid_name: StringEmail,
    //     string_email_invalid: StringEmail,
    //     string_email_invalid_local_segment_too_long: StringEmail,
    //     string_email_invalid_hostname_too_long: StringEmail,
    //     string_email_invalid_bad_hostname: StringEmail,
    //     string_email_empty: StringEmail,
    //
    //     string_address_valid_hostname: StringAddress,
    //     string_address_valid_hostname_uppercase: StringAddress,
    //     string_address_valid_hostname_hyphens: StringAddress,
    //     string_address_valid_hostname_trailing_dot: StringAddress,
    //     string_address_invalid_hostname: StringAddress,
    //     string_address_invalid_hostname_underscore: StringAddress,
    //     string_address_invalid_hostname_too_long: StringAddress,
    //     string_address_invalid_hostname_trailing_hyphens: StringAddress,
    //     string_address_invalid_hostname_leading_hyphens: StringAddress,
    //     string_address_invalid_hostname_empty: StringAddress,
    //     string_address_invalid_hostname_idns: StringAddress,
    //     string_address_valid_ip_v4: StringAddress,
    //     string_address_valid_ip_v6: StringAddress,
    //     string_address_invalid_ip: StringAddress,
    //
    //     string_hostname_valid: StringHostname,
    //     string_hostname_valid_uppercase: StringHostname,
    //     string_hostname_valid_hyphens: StringHostname,
    //     string_hostname_valid_trailing_dot: StringHostname,
    //     string_hostname_invalid: StringHostname,
    //     string_hostname_invalid_underscore: StringHostname,
    //     string_hostname_invalid_too_long: StringHostname,
    //     string_hostname_invalid_trailing_hyphens: StringHostname,
    //     string_hostname_invalid_leading_hyphens: StringHostname,
    //     string_hostname_invalid_empty: StringHostname,
    //     string_hostname_invalid_idns: StringHostname,
    //
    //     string_ip_valid_v4: StringIp,
    //     string_ip_valid_v6: StringIp,
    //     string_ip_invalid: StringIp,
    //
    //     string_ipv4_valid: StringIPv4,
    //     string_ipv4_invalid: StringIPv4,
    //     string_ipv4_invalid_erroneous: StringIPv4,
    //     string_ipv4_invalid_v6: StringIPv4,
    //
    //     string_ipv6_valid: StringIPv6,
    //     string_ipv6_valid_collapsed: StringIPv6,
    //     string_ipv6_invalid: StringIPv6,
    //     string_ipv6_invalid_v4: StringIPv6,
    //     string_ipv6_invalid_erroneous: StringIPv6,
    //
    //     string_uri_valid_2: StringUri,
    //     string_uri_invalid_2: StringUri,
    //     string_uri_invalid_relative: StringUri,
    //     string_uri_valid_3: StringUriRef,
    //     string_uri_valid_relative: StringUriRef,
    //     string_uri_invalid_3: StringUriRef,
    //
    //     string_uuid_valid_nil: StringUuid,
    //     string_uuid_valid_v1: StringUuid,
    //     string_uuid_valid_v1_case_insensitive: StringUuid,
    //     string_uuid_valid_v2: StringUuid,
    //     string_uuid_valid_v2_case_insensitive: StringUuid,
    //     string_uuid_valid_v3: StringUuid,
    //     string_uuid_valid_v3_case_insensitive: StringUuid,
    //     string_uuid_valid_v4: StringUuid,
    //     string_uuid_valid_v4_case_insensitive: StringUuid,
    //     string_uuid_valid_v5: StringUuid,
    //     string_uuid_valid_v5_case_insensitive: StringUuid,
    //     string_uuid_invalid: StringUuid,
    //     string_uuid_invalid_bad_uuid: StringUuid,
    //     string_uuid_valid_ignore_empty: StringUuidIgnore,
    //
    //     string_http_header_name_valid: StringHttpHeaderName,
    //     string_http_header_name_valid_2: StringHttpHeaderName,
    //     string_http_header_name_valid_nums: StringHttpHeaderName,
    //     string_http_header_name_valid_special_token: StringHttpHeaderName,
    //     string_http_header_name_valid_period: StringHttpHeaderName,
    //     string_http_header_name_invalid: StringHttpHeaderName,
    //     string_http_header_name_invalid_2: StringHttpHeaderName,
    //     string_http_header_name_invalid_space: StringHttpHeaderName,
    //     string_http_header_name_invalid_return: StringHttpHeaderName,
    //     string_http_header_name_invalid_tab: StringHttpHeaderName,
    //     string_http_header_name_invalid_slash: StringHttpHeaderName,
    //
    //     string_http_header_value_valid: StringHttpHeaderValue,
    //     string_http_header_value_valid_uppercase: StringHttpHeaderValue,
    //     string_http_header_value_valid_spaces: StringHttpHeaderValue,
    //     string_http_header_value_valid_tab: StringHttpHeaderValue,
    //     string_http_header_value_valid_special_token: StringHttpHeaderValue,
    //     string_http_header_value_invalid_nul: StringHttpHeaderValue,
    //     string_http_header_value_invalid_del: StringHttpHeaderValue,
    //     string_http_header_value_invalid: StringHttpHeaderValue,
    //
    //     string_non_strict_valid_header_valid: StringValidHeader,
    //     string_non_strict_valid_header_valid_uppercase: StringValidHeader,
    //     string_non_strict_valid_header_valid_spaces: StringValidHeader,
    //     string_non_strict_valid_header_valid_tab: StringValidHeader,
    //     string_non_strict_valid_header_valid_del: StringValidHeader,
    //     string_non_strict_valid_header_invalid_nul: StringValidHeader,
    //     string_non_strict_valid_header_invalid_cr: StringValidHeader,
    //     string_non_strict_valid_header_invalid_nl: StringValidHeader,
    //
    //     bytes_none_valid: BytesNone,
    //
    //     bytes_const_valid: BytesConst,
    //     bytes_const_invalid: BytesConst,
    //
    //     bytes_in_valid: BytesIn,
    //     bytes_in_invalid: BytesIn,
    //     bytes_not_in_valid: BytesNotIn,
    //     bytes_not_in_invalid: BytesNotIn,
    //
    //     bytes_len_valid: BytesLen,
    //     bytes_len_invalid_lt: BytesLen,
    //     bytes_len_invalid_gt: BytesLen,
    //
    //     bytes_min_len_valid: BytesMinLen,
    //     bytes_min_len_valid_min: BytesMinLen,
    //     bytes_min_len_invalid: BytesMinLen,
    //
    //     bytes_max_len_valid: BytesMaxLen,
    //     bytes_max_len_valid_max: BytesMaxLen,
    //     bytes_max_len_invalid: BytesMaxLen,
    //
    //     bytes_min_max_len_valid: BytesMinMaxLen,
    //     bytes_min_max_len_valid_min: BytesMinMaxLen,
    //     bytes_min_max_len_valid_max: BytesMinMaxLen,
    //     bytes_min_max_len_invalid_below: BytesMinMaxLen,
    //     bytes_min_max_len_invalid_above: BytesMinMaxLen,
    //
    //     bytes_equal_min_max_len_valid: BytesEqualMinMaxLen,
    //     bytes_equal_min_max_len_invalid: BytesEqualMinMaxLen,
    //
    //     bytes_pattern_valid: BytesPattern,
    //     // b"你好你好"
    //     bytes_pattern_invalid: BytesPattern,
    //     bytes_pattern_invalid_empty: BytesPattern,
    //
    //     bytes_prefix_valid: BytesPrefix,
    //     bytes_prefix_valid_only: BytesPrefix,
    //     bytes_prefix_invalid: BytesPrefix,
    //
    //     bytes_contains_valid: BytesContains,
    //     bytes_contains_valid_only: BytesContains,
    //     bytes_contains_invalid: BytesContains,
    //
    //     bytes_suffix_valid: BytesSuffix,
    //     bytes_suffix_valid_only: BytesSuffix,
    //     bytes_suffix_invalid: BytesSuffix,
    //     bytes_suffix_invalid_case_sensitive: BytesSuffix,
    //
    //     bytes_ip_valid_v4: BytesIp,
    //     bytes_ip_valid_v6: BytesIp,
    //     bytes_ip_invalid: BytesIp,
    //
    //     bytes_ipv4_valid: BytesIPv4,
    //     bytes_ipv4_invalid: BytesIPv4,
    //     bytes_ipv4_invalid_v6: BytesIPv4,
    //
    //     bytes_ipv6_valid: BytesIPv6,
    //     bytes_ipv6_invalid: BytesIPv6,
    //     bytes_ipv6_invalid_v4: BytesIPv6,
    //
    //     bytes_ipv6_valid_ignore_empty: BytesIPv6Ignore,
    //
    //     enum_none_valid: EnumNone,
    //
    //     enum_const_valid: EnumConst,
    //     enum_const_invalid: EnumConst,
    //
    //     enum_alias_const_valid: EnumAliasConst,
    //     enum_alias_const_valid_alias: EnumAliasConst,
    //     enum_alias_const_invalid: EnumAliasConst,
    //
    //     enum_defined_only_valid: EnumDefined,
    //     enum_defined_only_invalid: EnumDefined,
    //
    //     enum_alias_defined_only_valid: EnumAliasDefined,
    //     enum_alias_defined_only_invalid: EnumAliasDefined,
    //
    //     enum_in_valid: EnumIn,
    //     enum_in_invalid: EnumIn,
    //
    //     enum_alias_in_valid: EnumAliasIn,
    //     enum_alias_in_valid_alias: EnumAliasIn,
    //     enum_alias_in_invalid: EnumAliasIn,
    //
    //     enum_not_in_valid: EnumNotIn,
    //     enum_not_in_valid_undefined: EnumNotIn,
    //     enum_not_in_invalid: EnumNotIn,
    //
    //     enum_alias_not_in_valid: EnumAliasNotIn,
    //     enum_alias_not_in_invalid: EnumAliasNotIn,
    //     enum_alias_not_in_invalid_alias: EnumAliasNotIn,
    //
    //     enum_external_defined_only_valid: EnumExternal,
    //     enum_external_defined_only_invalid: EnumExternal,
    //     enum_external_in_valid: EnumExternal3,
    //     enum_external_in_invalid: EnumExternal3,
    //     enum_external_not_in_valid: EnumExternal3,
    //     enum_external_not_in_invalid: EnumExternal3,
    //     enum_external_const_valid: EnumExternal4,
    //     enum_external_const_invalid: EnumExternal4,
    //
    //     enum_repeated_defined_only_valid: RepeatedEnumDefined,
    //     enum_repeated_defined_only_invalid: RepeatedEnumDefined,
    //     enum_repeated_external_defined_only_valid: RepeatedExternalEnumDefined,
    //     enum_repeated_external_defined_only_invalid: RepeatedExternalEnumDefined,
    //     enum_repeated_another_external_defined_only_valid: RepeatedYetAnotherExternalEnumDefined,
    //     enum_repeated_external_in_valid: RepeatedEnumExternal,
    //     enum_repeated_external_in_invalid: RepeatedEnumExternal,
    //     enum_repeated_external_not_in_valid: RepeatedEnumExternal,
    //     enum_repeated_external_not_in_invalid: RepeatedEnumExternal,
    //
    //     enum_map_defined_only_valid: MapEnumDefined,
    //     enum_map_defined_only_invalid: MapEnumDefined,
    //     enum_map_external_defined_only_valid: MapExternalEnumDefined,
    //     enum_map_external_defined_only_invalid: MapExternalEnumDefined,
    //
    //     message_none_valid: MessageNone,
    //     message_none_valid_unset: MessageNone,
    //
    //     message_disabled_valid: MessageDisabled,
    //     message_disabled_valid_invalid_field: MessageDisabled,
    //
    //     message_ignored_valid: MessageIgnored,
    //     message_ignored_valid_invalid_field: MessageIgnored,
    //
    //     message_field_valid: Message,
    //     message_field_valid_unset: Message,
    //     message_field_invalid: Message,
    //     message_field_invalid_transitive: Message,
    //
    //     message_skip_valid: MessageSkip,
    //
    //     message_required_valid: MessageRequired,
    //     message_required_valid_oneof: MessageRequiredOneof,
    //     message_required_invalid: MessageRequired,
    //     message_required_invalid_oneof: MessageRequiredOneof,
    //
    //     message_cross_package_embed_none_valid: MessageCrossPackage,
    //     message_cross_package_embed_none_valid_nil: MessageCrossPackage,
    //     message_cross_package_embed_none_valid_empty: MessageCrossPackage,
    //     message_cross_package_embed_none_invalid: MessageCrossPackage,
    //
    //     message_required_valid_2: MessageRequiredButOptional,
    //     message_required_valid_unset: MessageRequiredButOptional,
    //
    //     repeated_none_valid: RepeatedNone,
    //
    //     repeated_embed_none_valid: RepeatedEmbedNone,
    //     repeated_embed_none_valid_nil: RepeatedEmbedNone,
    //     repeated_embed_none_valid_empty: RepeatedEmbedNone,
    //     repeated_embed_none_invalid: RepeatedEmbedNone,
    //     repeated_cross_package_embed_none_valid: RepeatedEmbedCrossPackageNone,
    //     repeated_cross_package_embed_none_valid_nil: RepeatedEmbedCrossPackageNone,
    //     repeated_cross_package_embed_none_valid_empty: RepeatedEmbedCrossPackageNone,
    //     repeated_cross_package_embed_none_invalid: RepeatedEmbedCrossPackageNone,
    //
    //     repeated_min_valid: RepeatedMin,
    //     repeated_min_valid_equal: RepeatedMin,
    //     repeated_min_invalid: RepeatedMin,
    //     repeated_min_invalid_element: RepeatedMin,
    //
    //     repeated_max_valid: RepeatedMax,
    //     repeated_max_valid_equal: RepeatedMax,
    //     repeated_max_invalid: RepeatedMax,
    //
    //     repeated_min_max_valid: RepeatedMinMax,
    //     repeated_min_max_valid_min: RepeatedMinMax,
    //     repeated_min_max_valid_max: RepeatedMinMax,
    //     repeated_min_max_invalid_below: RepeatedMinMax,
    //     repeated_min_max_invalid_above: RepeatedMinMax,
    //
    //     repeated_exact_valid: RepeatedExact,
    //     repeated_exact_invalid_below: RepeatedExact,
    //     repeated_exact_invalid_above: RepeatedExact,
    //
    //     repeated_unique_valid: RepeatedUnique,
    //     repeated_unique_valid_empty: RepeatedUnique,
    //     repeated_unique_valid_case_sensitivity: RepeatedUnique,
    //     repeated_unique_invalid: RepeatedUnique,
    //
    //     repeated_items_valid: RepeatedItemRule,
    //     repeated_items_valid_empty: RepeatedItemRule,
    //     repeated_items_valid_pattern: RepeatedItemPattern,
    //     repeated_items_invalid: RepeatedItemRule,
    //     repeated_items_invalid_pattern: RepeatedItemPattern,
    //     repeated_items_invalid_in: RepeatedItemIn,
    //     repeated_items_valid_in: RepeatedItemIn,
    //     repeated_items_invalid_not_in: RepeatedItemNotIn,
    //     repeated_items_valid_not_in: RepeatedItemNotIn,
    //
    //     repeated_items_invalid_enum_in: RepeatedEnumIn,
    //     repeated_items_valid_enum_in: RepeatedEnumIn,
    //     repeated_items_invalid_enum_not_in: RepeatedEnumNotIn,
    //     repeated_items_valid_enum_not_in: RepeatedEnumNotIn,
    //     repeated_items_invalid_embedded_enum_in: RepeatedEmbeddedEnumIn,
    //     repeated_items_valid_embedded_enum_in: RepeatedEmbeddedEnumIn,
    //     repeated_items_invalid_embedded_enum_not_in: RepeatedEmbeddedEnumNotIn,
    //     repeated_items_valid_embedded_enum_not_in: RepeatedEmbeddedEnumNotIn,
    //
    //     repeated_items_invalid_any_in: RepeatedAnyIn,
    //     repeated_items_valid_any_in: RepeatedAnyIn,
    //     repeated_items_invalid_any_not_in: RepeatedAnyNotIn,
    //     repeated_items_valid_any_not_in: RepeatedAnyNotIn,
    //
    //     repeated_embed_skip_valid: RepeatedEmbedSkip,
    //     repeated_embed_skip_valid_invalid_element: RepeatedEmbedSkip,
    //     repeated_min_and_items_len_valid: RepeatedMinAndItemLen,
    //     repeated_min_and_items_len_invalid_min: RepeatedMinAndItemLen,
    //     repeated_min_and_items_len_invalid_len: RepeatedMinAndItemLen,
    //     repeated_min_and_max_items_len_valid: RepeatedMinAndMaxItemLen,
    //     repeated_min_and_max_items_len_invalid_min_len: RepeatedMinAndMaxItemLen,
    //     repeated_min_and_max_items_len_invalid_max_len: RepeatedMinAndMaxItemLen,
    //
    //     repeated_duration_gte_valid: RepeatedDuration,
    //     repeated_duration_gte_valid_empty: RepeatedDuration,
    //     repeated_duration_gte_valid_equal: RepeatedDuration,
    //     repeated_duration_gte_invalid: RepeatedDuration,
    //
    //     repeated_exact_valid_ignore_empty: RepeatedExactIgnore,
    //
    //
    //     map_none_valid: MapNone,
    //
    //     map_min_pairs_valid: MapMin,
    //     map_min_pairs_valid_equal: MapMin,
    //     map_min_pairs_invalid: MapMin,
    //
    //     map_max_pairs_valid: MapMax,
    //     map_max_pairs_valid_equal: MapMax,
    //     map_max_pairs_invalid: MapMax,
    //
    //     map_min_max_valid: MapMinMax,
    //     map_min_max_valid_min: MapMinMax,
    //     map_min_max_valid_max: MapMinMax,
    //     map_min_max_invalid_below: MapMinMax,
    //     map_min_max_invalid_above: MapMinMax,
    //
    //     map_exact_valid: MapExact,
    //     map_exact_invalid_below: MapExact,
    //     map_exact_invalid_above: MapExact,
    //
    //     map_no_sparse_valid: MapNoSparse,
    //     map_no_sparse_valid_empty: MapNoSparse,
    //     // sparse maps are no longer supported, so this case is no longer possible
    //     // map_no_sparse_invalid: MapNoSparse,
    //
    //     map_keys_valid: MapKeys,
    //     map_keys_valid_empty: MapKeys,
    //     map_keys_valid_pattern: MapKeysPattern,
    //     map_keys_valid_in: MapKeysIn,
    //     map_keys_valid_not_in: MapKeysNotIn,
    //     map_keys_invalid: MapKeys,
    //     map_keys_invalid_pattern: MapKeysPattern,
    //     map_keys_invalid_in: MapKeysIn,
    //     map_keys_invalid_not_in: MapKeysNotIn,
    //
    //     map_values_valid: MapValues,
    //     map_values_valid_empty: MapValues,
    //     map_values_valid_pattern: MapValuesPattern,
    //     map_values_invalid: MapValues,
    //     map_values_invalid_pattern: MapValuesPattern,
    //
    //     map_recursive_valid: MapRecursive,
    //     map_recursive_invalid: MapRecursive,
    //     map_exact_valid_ignore_empty: MapExactIgnore,
    //     map_multiple_valid: MultipleMaps,
    //
    //
    //     oneof_none_valid: OneOfNone,
    //     oneof_none_valid_empty: OneOfNone,
    //
    //     oneof_field_valid_x: OneOf,
    //     oneof_field_valid_y: OneOf,
    //     oneof_field_valid_z: OneOf,
    //     oneof_field_valid_empty: OneOf,
    //     oneof_field_invalid_x: OneOf,
    //     oneof_field_invalid_y: OneOf,
    //     oneof_filed_invalid_z: OneOf,
    //
    //     oneof_required_valid: OneOfRequired,
    //     oneof_require_invalid: OneOfRequired,
    //
    //     oneof_ignore_empty_valid_x: OneOfIgnoreEmpty,
    //     oneof_ignore_empty_valid_y: OneOfIgnoreEmpty,
    //     oneof_ignore_empty_valid_z: OneOfIgnoreEmpty,
    //
    //
    //     wrapper_none_valid: WrapperNone,
    //     wrapper_none_valid_empty: WrapperNone,
    //
    //     wrapper_float_valid: WrapperFloat,
    //     wrapper_float_valid_empty: WrapperFloat,
    //     wrapper_float_invalid: WrapperFloat,
    //
    //     wrapper_double_valid: WrapperDouble,
    //     wrapper_double_valid_empty: WrapperDouble,
    //     wrapper_double_invalid: WrapperDouble,
    //
    //     wrapper_int64_valid: WrapperInt64,
    //     wrapper_int64_valid_empty: WrapperInt64,
    //     wrapper_int64_invalid: WrapperInt64,
    //
    //     wrapper_int32_valid: WrapperInt32,
    //     wrapper_int32_valid_empty: WrapperInt32,
    //     wrapper_int32_invalid: WrapperInt32,
    //
    //     wrapper_uint64_valid: WrapperUInt64,
    //     wrapper_uint64_valid_empty: WrapperUInt64,
    //     wrapper_uint64_invalid: WrapperUInt64,
    //
    //     wrapper_uint32_valid: WrapperUInt32,
    //     wrapper_uint32_valid_empty: WrapperUInt32,
    //     wrapper_uint32_invalid: WrapperUInt32,
    //
    //     wrapper_bool_valid: WrapperBool,
    //     wrapper_bool_valid_empty: WrapperBool,
    //     wrapper_bool_invalid: WrapperBool,
    //
    //     wrapper_string_valid: WrapperString,
    //     wrapper_string_valid_empty: WrapperString,
    //     wrapper_string_invalid: WrapperString,
    //
    //     wrapper_bytes_valid: WrapperBytes,
    //     wrapper_bytes_valid_empty: WrapperBytes,
    //     wrapper_bytes_invalid: WrapperBytes,
    //
    //     wrapper_required_string_valid: WrapperRequiredString,
    //     wrapper_required_string_invalid: WrapperRequiredString,
    //     wrapper_required_string_invalid_empty: WrapperRequiredString,
    //
    //     wrapper_required_string_empty_valid: WrapperRequiredEmptyString,
    //     wrapper_required_string_empty_invalid: WrapperRequiredEmptyString,
    //     wrapper_required_string_empty_invalid_empty: WrapperRequiredEmptyString,
    //
    //     wrapper_optional_string_uuid_valid: WrapperOptionalUuidString,
    //     wrapper_optional_string_uuid_valid_empty: WrapperOptionalUuidString,
    //     wrapper_optional_string_uuid_invalid: WrapperOptionalUuidString,
    //
    //     wrapper_required_float_valid: WrapperRequiredFloat,
    //     wrapper_required_float_invalid: WrapperRequiredFloat,
    //     wrapper_required_float_invalid_empty: WrapperRequiredFloat,
    //
    //     duration_none_valid: DurationNone,
    //
    //     duration_required_valid: DurationRequired,
    //     duration_required_invalid: DurationRequired,
    //
    //     duration_const_valid: DurationConst,
    //     duration_const_valid_empty: DurationConst,
    //     duration_const_invalid: DurationConst,
    //
    //     duration_in_valid: DurationIn,
    //     duration_in_valid_empty: DurationIn,
    //     duration_in_invalid: DurationIn,
    //
    //     duration_not_in_valid: DurationNotIn,
    //     duration_not_in_valid_empty: DurationNotIn,
    //     duration_not_in_invalid: DurationNotIn,
    //
    //     duration_lt_valid: DurationLt,
    //     duration_lt_valid_empty: DurationLt,
    //     duration_lt_invalid_equal: DurationLt,
    //     duration_lt_invalid: DurationLt,
    //
    //     duration_lte_valid: DurationLte,
    //     duration_lte_valid_empty: DurationLte,
    //     duration_lte_valid_equal: DurationLte,
    //     duration_lte_invalid: DurationLte,
    //
    //     duration_gt_valid: DurationGt,
    //     duration_gt_valid_empty: DurationGt,
    //     duration_gt_invalid_equal: DurationGt,
    //     duration_gt_invalid: DurationGt,
    //
    //     duration_gte_valid: DurationGte,
    //     duration_gte_valid_empty: DurationGte,
    //     duration_gte_valid_equal: DurationGte,
    //     duration_gte_invalid: DurationGte,
    //
    //     duration_gt_lt_valid: DurationGtlt,
    //     duration_gt_lt_valid_empty: DurationGtlt,
    //     duration_gt_lt_invalid_above: DurationGtlt,
    //     duration_gt_lt_invalid_below: DurationGtlt,
    //     duration_gt_lt_invalid_max: DurationGtlt,
    //     duration_gt_lt_invalid_min: DurationGtlt,
    //
    //     duration_exclusive_gt_lt_valid_empty: DurationExLtgt,
    //     duration_exclusive_gt_lt_valid_above: DurationExLtgt,
    //     duration_exclusive_gt_lt_valid_below: DurationExLtgt,
    //     duration_exclusive_gt_lt_invalid: DurationExLtgt,
    //     duration_exclusive_gt_lt_invalid_max: DurationExLtgt,
    //     duration_exclusive_gt_lt_invalid_min: DurationExLtgt,
    //
    //     duration_gte_lte_valid: DurationGtelte,
    //     duration_gte_lte_valid_empty: DurationGtelte,
    //     duration_gte_lte_valid_max: DurationGtelte,
    //     duration_gte_lte_valid_min: DurationGtelte,
    //     duration_gte_lte_invalid_above: DurationGtelte,
    //     duration_gte_lte_invalid_below: DurationGtelte,
    //
    //     duration_gte_lte_valid_empty_2: DurationExGtelte,
    //     duration_exclusive_gte_lte_valid_above: DurationExGtelte,
    //     duration_exclusive_gte_lte_valid_below: DurationExGtelte,
    //     duration_exclusive_gte_lte_valid_max: DurationExGtelte,
    //     duration_exclusive_gte_lte_valid_min: DurationExGtelte,
    //     duration_exclusive_gte_lte_invalid: DurationExGtelte,
    //     duration_fields_with_other_fields_invalid_other_field: DurationFieldWithOtherFields,
    //
    //     timestamp_none_valid: TimestampNone,
    //
    //     timestamp_required_valid: TimestampRequired,
    //     timestamp_required_invalid: TimestampRequired,
    //
    //     timestamp_const_valid: TimestampConst,
    //     timestamp_const_valid_empty: TimestampConst,
    //     timestamp_const_invalid: TimestampConst,
    //
    //     timestamp_lt_valid: TimestampLt,
    //     timestamp_lt_valid_empty: TimestampLt,
    //     timestamp_lt_invalid_equal: TimestampLt,
    //     timestamp_lt_invalid: TimestampLt,
    //
    //     timestamp_lte_valid: TimestampLte,
    //     timestamp_lte_valid_empty: TimestampLte,
    //     timestamp_lte_valid_equal: TimestampLte,
    //     timestamp_lte_invalid: TimestampLte,
    //
    //     timestamp_gt_valid: TimestampGt,
    //     timestamp_gt_valid_empty: TimestampGt,
    //     timestamp_gt_invalid_equal: TimestampGt,
    //     timestamp_gt_invalid: TimestampGt,
    //
    //     timestamp_gte_valid: TimestampGte,
    //     timestamp_gte_valid_empty: TimestampGte,
    //     timestamp_gte_valid_equal: TimestampGte,
    //     timestamp_gte_invalid: TimestampGte,
    //
    //     timestamp_gt_lt_valid: TimestampGtlt,
    //     timestamp_gt_lt_valid_empty: TimestampGtlt,
    //     timestamp_gt_lt_invalid_above: TimestampGtlt,
    //     timestamp_gt_lt_invalid_below: TimestampGtlt,
    //     timestamp_gt_lt_invalid_max: TimestampGtlt,
    //     timestamp_gt_lt_invalid_min: TimestampGtlt,
    //
    //     timestamp_exclusive_gt_lt_valid_empty: TimestampExLtgt,
    //     timestamp_exclusive_gt_lt_valid_above: TimestampExLtgt,
    //     timestamp_exclusive_gt_lt_valid_below: TimestampExLtgt,
    //     timestamp_exclusive_gt_lt_invalid: TimestampExLtgt,
    //     timestamp_exclusive_gt_lt_invalid_max: TimestampExLtgt,
    //     timestamp_exclusive_gt_lt_invalid_min: TimestampExLtgt,
    //
    //     timestamp_gte_lte_valid: TimestampGtelte,
    //     timestamp_gte_lte_valid_empty: TimestampGtelte,
    //     timestamp_gte_lte_valid_max: TimestampGtelte,
    //     timestamp_gte_lte_valid_min: TimestampGtelte,
    //     timestamp_gte_lte_invalid_above: TimestampGtelte,
    //     timestamp_gte_lte_invalid_below: TimestampGtelte,
    //
    //     timestamp_gte_lte_valid_empty_2: TimestampExGtelte,
    //     timestamp_exclusive_gte_lte_valid_above: TimestampExGtelte,
    //     timestamp_exclusive_gte_lte_valid_below: TimestampExGtelte,
    //     timestamp_exclusive_gte_lte_valid_max: TimestampExGtelte,
    //     timestamp_exclusive_gte_lte_valid_min: TimestampExGtelte,
    //     timestamp_exclusive_gte_lte_invalid: TimestampExGtelte,
    //
    //     timestamp_lt_now_valid: TimestampLtNow,
    //     timestamp_lt_now_valid_empty: TimestampLtNow,
    //     timestamp_lt_now_invalid: TimestampLtNow,
    //
    //     timestamp_gt_now_valid: TimestampGtNow,
    //     timestamp_gt_now_valid_empty: TimestampGtNow,
    //     timestamp_gt_now_invalid: TimestampGtNow,
    //
    //     timestamp_within_valid: TimestampWithin,
    //     timestamp_within_valid_empty: TimestampWithin,
    //     timestamp_within_invalid_below: TimestampWithin,
    //     timestamp_within_invalid_above: TimestampWithin,
    //
    //     timestamp_lt_now_within_valid: TimestampLtNowWithin,
    //     timestamp_lt_now_within_valid_empty: TimestampLtNowWithin,
    //     timestamp_lt_now_within_invalid_lt: TimestampLtNowWithin,
    //     timestamp_lt_now_within_invalid_within: TimestampLtNowWithin,
    //
    //     timestamp_gt_now_within_valid: TimestampGtNowWithin,
    //     timestamp_gt_now_within_valid_empty: TimestampGtNowWithin,
    //     timestamp_gt_now_within_invalid_gt: TimestampGtNowWithin,
    //     timestamp_gt_now_within_invalid_within: TimestampGtNowWithin,
    //
    //
    //     any_none_valid: AnyNone,
    //
    //     any_required_valid: AnyRequired,
    //     any_required_invalid: AnyRequired,
    //
    //     any_in_valid: AnyIn,
    //     any_in_valid_empty: AnyIn,
    //     any_in_invalid: AnyIn,
    //
    //     any_not_in_valid: AnyNotIn,
    //     any_not_in_valid_empty: AnyNotIn,
    //     any_not_in_invalid: AnyNotIn,
    //
    //
    //     kitchensink_field_valid: KitchenSinkMessage,
    //     kitchensink_valid_unset: KitchenSinkMessage,
    //     kitchensink_field_invalid: KitchenSinkMessage,
    //     kitchensink_field_embedded_invalid: KitchenSinkMessage,
    //     kitchensink_field_invalid_transitive: KitchenSinkMessage,
    //     kitchensink_many_all_non_message_fields_invalid: KitchenSinkMessage,
    //
    //
    //     nested_wkt_uuid_field_valid: WktLevelOne,
    //     nested_wkt_uuid_field_invalid: WktLevelOne,
    //
    // ];
    test_cases.into_iter().collect()
}
