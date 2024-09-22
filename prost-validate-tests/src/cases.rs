#[allow(unused_imports)]
use crate::proto::cases;
#[allow(unused_imports)]
use crate::proto::cases::message_required_oneof::One;
#[allow(unused_imports)]
use crate::proto::cases::*;
use prost_reflect::ReflectMessage;
#[allow(unused_imports)]
use prost_reflect_validate::ValidatorExt;
#[allow(unused_imports)]
use prost_types::{Any, Duration, Timestamp};
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused)]
struct TestCase<T: ReflectMessage> {
    pub message: T,
    pub failures: i32,
}

#[allow(unused)]
macro_rules! test_case {
    ($name:ident,$message:expr,$failures:expr) => {
        #[test]
        fn $name() {
            let case = TestCase{
                message: $message,
                failures: $failures,
            };
            match case.message.validate() {
                Ok(_) => assert!(case.failures == 0, "unexpected validation success"),
                Err(err) => assert!(case.failures > 0, "unexpected validation failure: {}", err),
            }
        }
    }
}

#[cfg(test)]
mod float {
    use super::*;
    test_case!(float_none_valid, FloatNone{val: -1.23456}, 0);

    test_case!(float_const_valid, FloatConst{val: 1.23}, 0);
    test_case!(float_const_invalid, FloatConst{val: 4.56}, 1);

    test_case!(float_in_valid, FloatIn{val: 7.89}, 0);
    test_case!(float_in_invalid, FloatIn{val: 10.11}, 1);

    test_case!(float_not_in_valid, FloatNotIn{val: 1.}, 0);
    test_case!(float_not_in_invalid, FloatNotIn{val: 0.}, 1);

    test_case!(float_lt_valid, FloatLt{val: -1.}, 0);
    test_case!(float_lt_invalid_equal, FloatLt{val: 0.}, 1);
    test_case!(float_lt_invalid, FloatLt{val: 1.}, 1);

    test_case!(float_lte_valid, FloatLte{val: 63.}, 0);
    test_case!(float_lte_valid_equal, FloatLte{val: 64.}, 0);
    test_case!(float_lte_invalid, FloatLte{val: 65.}, 1);

    test_case!(float_gt_valid, FloatGt{val: 17.}, 0);
    test_case!(float_gt_invalid_equal, FloatGt{val: 16.}, 1);
    test_case!(float_gt_invalid, FloatGt{val: 15.}, 1);

    test_case!(float_gte_valid, FloatGte{val: 9.}, 0);
    test_case!(float_gte_valid_equal, FloatGte{val: 8.}, 0);
    test_case!(float_gte_invalid, FloatGte{val: 7.}, 1);

    test_case!(float_gt_lt_valid, FloatGtlt{val: 5.}, 0);
    test_case!(float_gt_lt_invalid_above, FloatGtlt{val: 11.}, 1);
    test_case!(float_gt_lt_invalid_below, FloatGtlt{val: -1.}, 1);
    test_case!(float_gt_lt_invalid_max, FloatGtlt{val: 10.}, 1);
    test_case!(float_gt_lt_invalid_min, FloatGtlt{val: 0.}, 1);

    test_case!(float_exclusive_gt_lt_valid_above, FloatExLtgt{val: 11.}, 0);
    test_case!(float_exclusive_gt_lt_valid_below, FloatExLtgt{val: -1.}, 0);
    test_case!(float_exclusive_gt_lt_invalid, FloatExLtgt{val: 5.}, 1);
    test_case!(float_exclusive_gt_lt_invalid_max, FloatExLtgt{val: 10.}, 1);
    test_case!(float_exclusive_gt_lt_invalid_min, FloatExLtgt{val: 0.}, 1);

    test_case!(float_gte_lte_valid, FloatGtelte{val: 200.}, 0);
    test_case!(float_gte_lte_valid_max, FloatGtelte{val: 256.}, 0);
    test_case!(float_gte_lte_valid_min, FloatGtelte{val: 128.}, 0);
    test_case!(float_gte_lte_invalid_above, FloatGtelte{val: 300.}, 1);
    test_case!(float_gte_lte_invalid_below, FloatGtelte{val: 100.}, 1);

    test_case!(float_exclusive_gte_lte_valid_above, FloatExGtelte{val: 300.}, 0);
    test_case!(float_exclusive_gte_lte_valid_below, FloatExGtelte{val: 100.}, 0);
    test_case!(float_exclusive_gte_lte_valid_max, FloatExGtelte{val: 256.}, 0);
    test_case!(float_exclusive_gte_lte_valid_min, FloatExGtelte{val: 128.}, 0);
    test_case!(float_exclusive_gte_lte_invalid, FloatExGtelte{val: 200.}, 1);
    test_case!(float_ignore_empty_gte_lte_valid, FloatIgnore{val: 0.}, 0);
}
#[cfg(test)]
mod double {
    use super::*;
    test_case!(double_none_valid, DoubleNone{val: -1.23456}, 0);

    test_case!(double_const_valid, DoubleConst{val: 1.23}, 0);
    test_case!(double_const_invalid, DoubleConst{val: 4.56}, 1);

    test_case!(double_in_valid, DoubleIn{val: 7.89}, 0);
    test_case!(double_in_invalid, DoubleIn{val: 10.11}, 1);

    test_case!(double_not_in_valid, DoubleNotIn{val: 1.}, 0);
    test_case!(double_not_in_invalid, DoubleNotIn{val: 0.}, 1);

    test_case!(double_lt_valid, DoubleLt{val: -1.}, 0);
    test_case!(double_lt_invalid_equal, DoubleLt{val: 0.}, 1);
    test_case!(double_lt_invalid, DoubleLt{val: 1.}, 1);

    test_case!(double_lte_valid, DoubleLte{val: 63.}, 0);
    test_case!(double_lte_valid_equal, DoubleLte{val: 64.}, 0);
    test_case!(double_lte_invalid, DoubleLte{val: 65.}, 1);

    test_case!(double_gt_valid, DoubleGt{val: 17.}, 0);
    test_case!(double_gt_invalid_equal, DoubleGt{val: 16.}, 1);
    test_case!(double_gt_invalid, DoubleGt{val: 15.}, 1);

    test_case!(double_gte_valid, DoubleGte{val: 9.}, 0);
    test_case!(double_gte_valid_equal, DoubleGte{val: 8.}, 0);
    test_case!(double_gte_invalid, DoubleGte{val: 7.}, 1);

    test_case!(double_gt_lt_valid, DoubleGtlt{val: 5.}, 0);
    test_case!(double_gt_lt_invalid_above, DoubleGtlt{val: 11.}, 1);
    test_case!(double_gt_lt_invalid_below, DoubleGtlt{val: -1.}, 1);
    test_case!(double_gt_lt_invalid_max, DoubleGtlt{val: 10.}, 1);
    test_case!(double_gt_lt_invalid_min, DoubleGtlt{val: 0.}, 1);

    test_case!(double_exclusive_gt_lt_valid_above, DoubleExLtgt{val: 11.}, 0);
    test_case!(double_exclusive_gt_lt_valid_below, DoubleExLtgt{val: -1.}, 0);
    test_case!(double_exclusive_gt_lt_invalid, DoubleExLtgt{val: 5.}, 1);
    test_case!(double_exclusive_gt_lt_invalid_max, DoubleExLtgt{val: 10.}, 1);
    test_case!(double_exclusive_gt_lt_invalid_min, DoubleExLtgt{val: 0.}, 1);

    test_case!(double_gte_lte_valid, DoubleGtelte{val: 200.}, 0);
    test_case!(double_gte_lte_valid_max, DoubleGtelte{val: 256.}, 0);
    test_case!(double_gte_lte_valid_min, DoubleGtelte{val: 128.}, 0);
    test_case!(double_gte_lte_invalid_above, DoubleGtelte{val: 300.}, 1);
    test_case!(double_gte_lte_invalid_below, DoubleGtelte{val: 100.}, 1);

    test_case!(double_exclusive_gte_lte_valid_above, DoubleExGtelte{val: 300.}, 0);
    test_case!(double_exclusive_gte_lte_valid_below, DoubleExGtelte{val: 100.}, 0);
    test_case!(double_exclusive_gte_lte_valid_max, DoubleExGtelte{val: 256.}, 0);
    test_case!(double_exclusive_gte_lte_valid_min, DoubleExGtelte{val: 128.}, 0);
    test_case!(double_exclusive_gte_lte_invalid, DoubleExGtelte{val: 200.}, 1);

    test_case!(double_ignore_empty_gte_lte_valid, DoubleIgnore{val: 0.}, 0);
}
#[cfg(test)]
mod int32 {
    use super::*;
    test_case!(int32_none_valid, Int32None{val: 123}, 0);

    test_case!(int32_const_valid, Int32Const{val: 1}, 0);
    test_case!(int32_const_invalid, Int32Const{val: 2}, 1);

    test_case!(int32_in_valid, Int32In{val: 3}, 0);
    test_case!(int32_in_invalid, Int32In{val: 5}, 1);

    test_case!(int32_not_in_valid, Int32NotIn{val: 1}, 0);
    test_case!(int32_not_in_invalid, Int32NotIn{val: 0}, 1);

    test_case!(int32_lt_valid, Int32Lt{val: -1}, 0);
    test_case!(int32_lt_invalid_equal, Int32Lt{val: 0}, 1);
    test_case!(int32_lt_invalid, Int32Lt{val: 1}, 1);

    test_case!(int32_lte_valid, Int32Lte{val: 63}, 0);
    test_case!(int32_lte_valid_equal, Int32Lte{val: 64}, 0);
    test_case!(int32_lte_invalid, Int32Lte{val: 65}, 1);

    test_case!(int32_gt_valid, Int32Gt{val: 17}, 0);
    test_case!(int32_gt_invalid_equal, Int32Gt{val: 16}, 1);
    test_case!(int32_gt_invalid, Int32Gt{val: 15}, 1);

    test_case!(int32_gte_valid, Int32Gte{val: 9}, 0);
    test_case!(int32_gte_valid_equal, Int32Gte{val: 8}, 0);
    test_case!(int32_gte_invalid, Int32Gte{val: 7}, 1);

    test_case!(int32_gt_lt_valid, Int32Gtlt{val: 5}, 0);
    test_case!(int32_gt_lt_invalid_above, Int32Gtlt{val: 11}, 1);
    test_case!(int32_gt_lt_invalid_below, Int32Gtlt{val: -1}, 1);
    test_case!(int32_gt_lt_invalid_max, Int32Gtlt{val: 10}, 1);
    test_case!(int32_gt_lt_invalid_min, Int32Gtlt{val: 0}, 1);

    test_case!(int32_exclusive_gt_lt_valid_above, Int32ExLtgt{val: 11}, 0);
    test_case!(int32_exclusive_gt_lt_valid_below, Int32ExLtgt{val: -1}, 0);
    test_case!(int32_exclusive_gt_lt_invalid, Int32ExLtgt{val: 5}, 1);
    test_case!(int32_exclusive_gt_lt_invalid_max, Int32ExLtgt{val: 10}, 1);
    test_case!(int32_exclusive_gt_lt_invalid_min, Int32ExLtgt{val: 0}, 1);

    test_case!(int32_gte_lte_valid, Int32Gtelte{val: 200}, 0);
    test_case!(int32_gte_lte_valid_max, Int32Gtelte{val: 256}, 0);
    test_case!(int32_gte_lte_valid_min, Int32Gtelte{val: 128}, 0);
    test_case!(int32_gte_lte_invalid_above, Int32Gtelte{val: 300}, 1);
    test_case!(int32_gte_lte_invalid_below, Int32Gtelte{val: 100}, 1);

    test_case!(int32_exclusive_gte_lte_valid_above, Int32ExGtelte{val: 300}, 0);
    test_case!(int32_exclusive_gte_lte_valid_below, Int32ExGtelte{val: 100}, 0);
    test_case!(int32_exclusive_gte_lte_valid_max, Int32ExGtelte{val: 256}, 0);
    test_case!(int32_exclusive_gte_lte_valid_min, Int32ExGtelte{val: 128}, 0);
    test_case!(int32_exclusive_gte_lte_invalid, Int32ExGtelte{val: 200}, 1);

    test_case!(int32_ignore_empty_gte_lte_valid, Int32Ignore{val: 0}, 0);
}
#[cfg(test)]
mod int64 {
    use super::*;
    test_case!(int64_none_valid, Int64None{val: 123}, 0);

    test_case!(int64_const_valid, Int64Const{val: 1}, 0);
    test_case!(int64_const_invalid, Int64Const{val: 2}, 1);

    test_case!(int64_in_valid, Int64In{val: 3}, 0);
    test_case!(int64_in_invalid, Int64In{val: 5}, 1);

    test_case!(int64_not_in_valid, Int64NotIn{val: 1}, 0);
    test_case!(int64_not_in_invalid, Int64NotIn{val: 0}, 1);

    test_case!(int64_lt_valid, Int64Lt{val: -1}, 0);
    test_case!(int64_lt_invalid_equal, Int64Lt{val: 0}, 1);
    test_case!(int64_lt_invalid, Int64Lt{val: 1}, 1);

    test_case!(int64_lte_valid, Int64Lte{val: 63}, 0);
    test_case!(int64_lte_valid_equal, Int64Lte{val: 64}, 0);
    test_case!(int64_lte_invalid, Int64Lte{val: 65}, 1);

    test_case!(int64_gt_valid, Int64Gt{val: 17}, 0);
    test_case!(int64_gt_invalid_equal, Int64Gt{val: 16}, 1);
    test_case!(int64_gt_invalid, Int64Gt{val: 15}, 1);

    test_case!(int64_gte_valid, Int64Gte{val: 9}, 0);
    test_case!(int64_gte_valid_equal, Int64Gte{val: 8}, 0);
    test_case!(int64_gte_invalid, Int64Gte{val: 7}, 1);

    test_case!(int64_gt_lt_valid, Int64Gtlt{val: 5}, 0);
    test_case!(int64_gt_lt_invalid_above, Int64Gtlt{val: 11}, 1);
    test_case!(int64_gt_lt_invalid_below, Int64Gtlt{val: -1}, 1);
    test_case!(int64_gt_lt_invalid_max, Int64Gtlt{val: 10}, 1);
    test_case!(int64_gt_lt_invalid_min, Int64Gtlt{val: 0}, 1);

    test_case!(int64_exclusive_gt_lt_valid_above, Int64ExLtgt{val: 11}, 0);
    test_case!(int64_exclusive_gt_lt_valid_below, Int64ExLtgt{val: -1}, 0);
    test_case!(int64_exclusive_gt_lt_invalid, Int64ExLtgt{val: 5}, 1);
    test_case!(int64_exclusive_gt_lt_invalid_max, Int64ExLtgt{val: 10}, 1);
    test_case!(int64_exclusive_gt_lt_invalid_min, Int64ExLtgt{val: 0}, 1);

    test_case!(int64_gte_lte_valid, Int64Gtelte{val: 200}, 0);
    test_case!(int64_gte_lte_valid_max, Int64Gtelte{val: 256}, 0);
    test_case!(int64_gte_lte_valid_min, Int64Gtelte{val: 128}, 0);
    test_case!(int64_gte_lte_invalid_above, Int64Gtelte{val: 300}, 1);
    test_case!(int64_gte_lte_invalid_below, Int64Gtelte{val: 100}, 1);

    test_case!(int64_exclusive_gte_lte_valid_above, Int64ExGtelte{val: 300}, 0);
    test_case!(int64_exclusive_gte_lte_valid_below, Int64ExGtelte{val: 100}, 0);
    test_case!(int64_exclusive_gte_lte_valid_max, Int64ExGtelte{val: 256}, 0);
    test_case!(int64_exclusive_gte_lte_valid_min, Int64ExGtelte{val: 128}, 0);
    test_case!(int64_exclusive_gte_lte_invalid, Int64ExGtelte{val: 200}, 1);

    test_case!(int64_ignore_empty_gte_lte_valid, Int64Ignore{val: 0}, 0);

    test_case!(int64_optional_lte_valid, Int64LteOptional{val: Some(63)}, 0);
    test_case!(int64_optional_lte_valid_equal, Int64LteOptional{val: Some(64)}, 0);
    test_case!(int64_optional_lte_valid_unset, Int64LteOptional{val: None}, 0);
}
#[cfg(test)]
mod uint32 {
    use super::*;
    test_case!(uint32_none_valid, UInt32None{val: 123}, 0);

    test_case!(uint32_const_valid, UInt32Const{val: 1}, 0);
    test_case!(uint32_const_invalid, UInt32Const{val: 2}, 1);

    test_case!(uint32_in_valid, UInt32In{val: 3}, 0);
    test_case!(uint32_in_invalid, UInt32In{val: 5}, 1);

    test_case!(uint32_not_in_valid, UInt32NotIn{val: 1}, 0);
    test_case!(uint32_not_in_invalid, UInt32NotIn{val: 0}, 1);

    test_case!(uint32_lt_valid, UInt32Lt{val: 4}, 0);
    test_case!(uint32_lt_invalid_equal, UInt32Lt{val: 5}, 1);
    test_case!(uint32_lt_invalid, UInt32Lt{val: 6}, 1);

    test_case!(uint32_lte_valid, UInt32Lte{val: 63}, 0);
    test_case!(uint32_lte_valid_equal, UInt32Lte{val: 64}, 0);
    test_case!(uint32_lte_invalid, UInt32Lte{val: 65}, 1);

    test_case!(uint32_gt_valid, UInt32Gt{val: 17}, 0);
    test_case!(uint32_gt_invalid_equal, UInt32Gt{val: 16}, 1);
    test_case!(uint32_gt_invalid, UInt32Gt{val: 15}, 1);

    test_case!(uint32_gte_valid, UInt32Gte{val: 9}, 0);
    test_case!(uint32_gte_valid_equal, UInt32Gte{val: 8}, 0);
    test_case!(uint32_gte_invalid, UInt32Gte{val: 7}, 1);

    test_case!(uint32_gt_lt_valid, UInt32Gtlt{val: 7}, 0);
    test_case!(uint32_gt_lt_invalid_above, UInt32Gtlt{val: 11}, 1);
    test_case!(uint32_gt_lt_invalid_below, UInt32Gtlt{val: 1}, 1);
    test_case!(uint32_gt_lt_invalid_max, UInt32Gtlt{val: 10}, 1);
    test_case!(uint32_gt_lt_invalid_min, UInt32Gtlt{val: 5}, 1);

    test_case!(uint32_exclusive_gt_lt_valid_above, UInt32ExLtgt{val: 11}, 0);
    test_case!(uint32_exclusive_gt_lt_valid_below, UInt32ExLtgt{val: 4}, 0);
    test_case!(uint32_exclusive_gt_lt_invalid, UInt32ExLtgt{val: 7}, 1);
    test_case!(uint32_exclusive_gt_lt_invalid_max, UInt32ExLtgt{val: 10}, 1);
    test_case!(uint32_exclusive_gt_lt_invalid_min, UInt32ExLtgt{val: 5}, 1);

    test_case!(uint32_gte_lte_valid, UInt32Gtelte{val: 200}, 0);
    test_case!(uint32_gte_lte_valid_max, UInt32Gtelte{val: 256}, 0);
    test_case!(uint32_gte_lte_valid_min, UInt32Gtelte{val: 128}, 0);
    test_case!(uint32_gte_lte_invalid_above, UInt32Gtelte{val: 300}, 1);
    test_case!(uint32_gte_lte_invalid_below, UInt32Gtelte{val: 100}, 1);

    test_case!(uint32_exclusive_gte_lte_valid_above, UInt32ExGtelte{val: 300}, 0);
    test_case!(uint32_exclusive_gte_lte_valid_below, UInt32ExGtelte{val: 100}, 0);
    test_case!(uint32_exclusive_gte_lte_valid_max, UInt32ExGtelte{val: 256}, 0);
    test_case!(uint32_exclusive_gte_lte_valid_min, UInt32ExGtelte{val: 128}, 0);
    test_case!(uint32_exclusive_gte_lte_invalid, UInt32ExGtelte{val: 200}, 1);

    test_case!(uint32_ignore_empty_gte_lte_valid, UInt32Ignore{val: 0}, 0);
}
#[cfg(test)]
mod uint64 {
    use super::*;
    test_case!(uint64_none_valid, UInt64None{val: 123}, 0);

    test_case!(uint64_const_valid, UInt64Const{val: 1}, 0);
    test_case!(uint64_const_invalid, UInt64Const{val: 2}, 1);

    test_case!(uint64_in_valid, UInt64In{val: 3}, 0);
    test_case!(uint64_in_invalid, UInt64In{val: 5}, 1);

    test_case!(uint64_not_in_valid, UInt64NotIn{val: 1}, 0);
    test_case!(uint64_not_in_invalid, UInt64NotIn{val: 0}, 1);

    test_case!(uint64_lt_valid, UInt64Lt{val: 4}, 0);
    test_case!(uint64_lt_invalid_equal, UInt64Lt{val: 5}, 1);
    test_case!(uint64_lt_invalid, UInt64Lt{val: 6}, 1);

    test_case!(uint64_lte_valid, UInt64Lte{val: 63}, 0);
    test_case!(uint64_lte_valid_equal, UInt64Lte{val: 64}, 0);
    test_case!(uint64_lte_invalid, UInt64Lte{val: 65}, 1);

    test_case!(uint64_gt_valid, UInt64Gt{val: 17}, 0);
    test_case!(uint64_gt_invalid_equal, UInt64Gt{val: 16}, 1);
    test_case!(uint64_gt_invalid, UInt64Gt{val: 15}, 1);

    test_case!(uint64_gte_valid, UInt64Gte{val: 9}, 0);
    test_case!(uint64_gte_valid_equal, UInt64Gte{val: 8}, 0);
    test_case!(uint64_gte_invalid, UInt64Gte{val: 7}, 1);

    test_case!(uint64_gt_lt_valid, UInt64Gtlt{val: 7}, 0);
    test_case!(uint64_gt_lt_invalid_above, UInt64Gtlt{val: 11}, 1);
    test_case!(uint64_gt_lt_invalid_below, UInt64Gtlt{val: 1}, 1);
    test_case!(uint64_gt_lt_invalid_max, UInt64Gtlt{val: 10}, 1);
    test_case!(uint64_gt_lt_invalid_min, UInt64Gtlt{val: 5}, 1);

    test_case!(uint64_exclusive_gt_lt_valid_above, UInt64ExLtgt{val: 11}, 0);
    test_case!(uint64_exclusive_gt_lt_valid_below, UInt64ExLtgt{val: 4}, 0);
    test_case!(uint64_exclusive_gt_lt_invalid, UInt64ExLtgt{val: 7}, 1);
    test_case!(uint64_exclusive_gt_lt_invalid_max, UInt64ExLtgt{val: 10}, 1);
    test_case!(uint64_exclusive_gt_lt_invalid_min, UInt64ExLtgt{val: 5}, 1);

    test_case!(uint64_gte_lte_valid, UInt64Gtelte{val: 200}, 0);
    test_case!(uint64_gte_lte_valid_max, UInt64Gtelte{val: 256}, 0);
    test_case!(uint64_gte_lte_valid_min, UInt64Gtelte{val: 128}, 0);
    test_case!(uint64_gte_lte_invalid_above, UInt64Gtelte{val: 300}, 1);
    test_case!(uint64_gte_lte_invalid_below, UInt64Gtelte{val: 100}, 1);

    test_case!(uint64_exclusive_gte_lte_valid_above, UInt64ExGtelte{val: 300}, 0);
    test_case!(uint64_exclusive_gte_lte_valid_below, UInt64ExGtelte{val: 100}, 0);
    test_case!(uint64_exclusive_gte_lte_valid_max, UInt64ExGtelte{val: 256}, 0);
    test_case!(uint64_exclusive_gte_lte_valid_min, UInt64ExGtelte{val: 128}, 0);
    test_case!(uint64_exclusive_gte_lte_invalid, UInt64ExGtelte{val: 200}, 1);

    test_case!(uint64_ignore_empty_gte_lte_valid, UInt64Ignore{val: 0}, 0);
}
#[cfg(test)]
mod sint32 {
    use super::*;
    test_case!(sint32_none_valid, SInt32None{val: 123}, 0);

    test_case!(sint32_const_valid, SInt32Const{val: 1}, 0);
    test_case!(sint32_const_invalid, SInt32Const{val: 2}, 1);

    test_case!(sint32_in_valid, SInt32In{val: 3}, 0);
    test_case!(sint32_in_invalid, SInt32In{val: 5}, 1);

    test_case!(sint32_not_in_valid, SInt32NotIn{val: 1}, 0);
    test_case!(sint32_not_in_invalid, SInt32NotIn{val: 0}, 1);

    test_case!(sint32_lt_valid, SInt32Lt{val: -1}, 0);
    test_case!(sint32_lt_invalid_equal, SInt32Lt{val: 0}, 1);
    test_case!(sint32_lt_invalid, SInt32Lt{val: 1}, 1);

    test_case!(sint32_lte_valid, SInt32Lte{val: 63}, 0);
    test_case!(sint32_lte_valid_equal, SInt32Lte{val: 64}, 0);
    test_case!(sint32_lte_invalid, SInt32Lte{val: 65}, 1);

    test_case!(sint32_gt_valid, SInt32Gt{val: 17}, 0);
    test_case!(sint32_gt_invalid_equal, SInt32Gt{val: 16}, 1);
    test_case!(sint32_gt_invalid, SInt32Gt{val: 15}, 1);

    test_case!(sint32_gte_valid, SInt32Gte{val: 9}, 0);
    test_case!(sint32_gte_valid_equal, SInt32Gte{val: 8}, 0);
    test_case!(sint32_gte_invalid, SInt32Gte{val: 7}, 1);

    test_case!(sint32_gt_lt_valid, SInt32Gtlt{val: 5}, 0);
    test_case!(sint32_gt_lt_invalid_above, SInt32Gtlt{val: 11}, 1);
    test_case!(sint32_gt_lt_invalid_below, SInt32Gtlt{val: -1}, 1);
    test_case!(sint32_gt_lt_invalid_max, SInt32Gtlt{val: 10}, 1);
    test_case!(sint32_gt_lt_invalid_min, SInt32Gtlt{val: 0}, 1);

    test_case!(sint32_exclusive_gt_lt_valid_above, SInt32ExLtgt{val: 11}, 0);
    test_case!(sint32_exclusive_gt_lt_valid_below, SInt32ExLtgt{val: -1}, 0);
    test_case!(sint32_exclusive_gt_lt_invalid, SInt32ExLtgt{val: 5}, 1);
    test_case!(sint32_exclusive_gt_lt_invalid_max, SInt32ExLtgt{val: 10}, 1);
    test_case!(sint32_exclusive_gt_lt_invalid_min, SInt32ExLtgt{val: 0}, 1);

    test_case!(sint32_gte_lte_valid, SInt32Gtelte{val: 200}, 0);
    test_case!(sint32_gte_lte_valid_max, SInt32Gtelte{val: 256}, 0);
    test_case!(sint32_gte_lte_valid_min, SInt32Gtelte{val: 128}, 0);
    test_case!(sint32_gte_lte_invalid_above, SInt32Gtelte{val: 300}, 1);
    test_case!(sint32_gte_lte_invalid_below, SInt32Gtelte{val: 100}, 1);

    test_case!(sint32_exclusive_gte_lte_valid_above, SInt32ExGtelte{val: 300}, 0);
    test_case!(sint32_exclusive_gte_lte_valid_below, SInt32ExGtelte{val: 100}, 0);
    test_case!(sint32_exclusive_gte_lte_valid_max, SInt32ExGtelte{val: 256}, 0);
    test_case!(sint32_exclusive_gte_lte_valid_min, SInt32ExGtelte{val: 128}, 0);
    test_case!(sint32_exclusive_gte_lte_invalid, SInt32ExGtelte{val: 200}, 1);

    test_case!(sint32_ignore_empty_gte_lte_valid, SInt32Ignore{val: 0}, 0);
}
#[cfg(test)]
mod sint64 {
    use super::*;
    test_case!(sint64_none_valid, SInt64None{val: 123}, 0);

    test_case!(sint64_const_valid, SInt64Const{val: 1}, 0);
    test_case!(sint64_const_invalid, SInt64Const{val: 2}, 1);

    test_case!(sint64_in_valid, SInt64In{val: 3}, 0);
    test_case!(sint64_in_invalid, SInt64In{val: 5}, 1);

    test_case!(sint64_not_in_valid, SInt64NotIn{val: 1}, 0);
    test_case!(sint64_not_in_invalid, SInt64NotIn{val: 0}, 1);

    test_case!(sint64_lt_valid, SInt64Lt{val: -1}, 0);
    test_case!(sint64_lt_invalid_equal, SInt64Lt{val: 0}, 1);
    test_case!(sint64_lt_invalid, SInt64Lt{val: 1}, 1);

    test_case!(sint64_lte_valid, SInt64Lte{val: 63}, 0);
    test_case!(sint64_lte_valid_equal, SInt64Lte{val: 64}, 0);
    test_case!(sint64_lte_invalid, SInt64Lte{val: 65}, 1);

    test_case!(sint64_gt_valid, SInt64Gt{val: 17}, 0);
    test_case!(sint64_gt_invalid_equal, SInt64Gt{val: 16}, 1);
    test_case!(sint64_gt_invalid, SInt64Gt{val: 15}, 1);

    test_case!(sint64_gte_valid, SInt64Gte{val: 9}, 0);
    test_case!(sint64_gte_valid_equal, SInt64Gte{val: 8}, 0);
    test_case!(sint64_gte_invalid, SInt64Gte{val: 7}, 1);

    test_case!(sint64_gt_lt_valid, SInt64Gtlt{val: 5}, 0);
    test_case!(sint64_gt_lt_invalid_above, SInt64Gtlt{val: 11}, 1);
    test_case!(sint64_gt_lt_invalid_below, SInt64Gtlt{val: -1}, 1);
    test_case!(sint64_gt_lt_invalid_max, SInt64Gtlt{val: 10}, 1);
    test_case!(sint64_gt_lt_invalid_min, SInt64Gtlt{val: 0}, 1);

    test_case!(sint64_exclusive_gt_lt_valid_above, SInt64ExLtgt{val: 11}, 0);
    test_case!(sint64_exclusive_gt_lt_valid_below, SInt64ExLtgt{val: -1}, 0);
    test_case!(sint64_exclusive_gt_lt_invalid, SInt64ExLtgt{val: 5}, 1);
    test_case!(sint64_exclusive_gt_lt_invalid_max, SInt64ExLtgt{val: 10}, 1);
    test_case!(sint64_exclusive_gt_lt_invalid_min, SInt64ExLtgt{val: 0}, 1);

    test_case!(sint64_gte_lte_valid, SInt64Gtelte{val: 200}, 0);
    test_case!(sint64_gte_lte_valid_max, SInt64Gtelte{val: 256}, 0);
    test_case!(sint64_gte_lte_valid_min, SInt64Gtelte{val: 128}, 0);
    test_case!(sint64_gte_lte_invalid_above, SInt64Gtelte{val: 300}, 1);
    test_case!(sint64_gte_lte_invalid_below, SInt64Gtelte{val: 100}, 1);

    test_case!(sint64_exclusive_gte_lte_valid_above, SInt64ExGtelte{val: 300}, 0);
    test_case!(sint64_exclusive_gte_lte_valid_below, SInt64ExGtelte{val: 100}, 0);
    test_case!(sint64_exclusive_gte_lte_valid_max, SInt64ExGtelte{val: 256}, 0);
    test_case!(sint64_exclusive_gte_lte_valid_min, SInt64ExGtelte{val: 128}, 0);
    test_case!(sint64_exclusive_gte_lte_invalid, SInt64ExGtelte{val: 200}, 1);

    test_case!(sint64_ignore_empty_gte_lte_valid, SInt64Ignore{val: 0}, 0);
}
#[cfg(test)]
mod fixed32 {
    use super::*;
    test_case!(fixed32_none_valid, Fixed32None{val: 123}, 0);

    test_case!(fixed32_const_valid, Fixed32Const{val: 1}, 0);
    test_case!(fixed32_const_invalid, Fixed32Const{val: 2}, 1);

    test_case!(fixed32_in_valid, Fixed32In{val: 3}, 0);
    test_case!(fixed32_in_invalid, Fixed32In{val: 5}, 1);

    test_case!(fixed32_not_in_valid, Fixed32NotIn{val: 1}, 0);
    test_case!(fixed32_not_in_invalid, Fixed32NotIn{val: 0}, 1);

    test_case!(fixed32_lt_valid, Fixed32Lt{val: 4}, 0);
    test_case!(fixed32_lt_invalid_equal, Fixed32Lt{val: 5}, 1);
    test_case!(fixed32_lt_invalid, Fixed32Lt{val: 6}, 1);

    test_case!(fixed32_lte_valid, Fixed32Lte{val: 63}, 0);
    test_case!(fixed32_lte_valid_equal, Fixed32Lte{val: 64}, 0);
    test_case!(fixed32_lte_invalid, Fixed32Lte{val: 65}, 1);

    test_case!(fixed32_gt_valid, Fixed32Gt{val: 17}, 0);
    test_case!(fixed32_gt_invalid_equal, Fixed32Gt{val: 16}, 1);
    test_case!(fixed32_gt_invalid, Fixed32Gt{val: 15}, 1);

    test_case!(fixed32_gte_valid, Fixed32Gte{val: 9}, 0);
    test_case!(fixed32_gte_valid_equal, Fixed32Gte{val: 8}, 0);
    test_case!(fixed32_gte_invalid, Fixed32Gte{val: 7}, 1);

    test_case!(fixed32_gt_lt_valid, Fixed32Gtlt{val: 7}, 0);
    test_case!(fixed32_gt_lt_invalid_above, Fixed32Gtlt{val: 11}, 1);
    test_case!(fixed32_gt_lt_invalid_below, Fixed32Gtlt{val: 1}, 1);
    test_case!(fixed32_gt_lt_invalid_max, Fixed32Gtlt{val: 10}, 1);
    test_case!(fixed32_gt_lt_invalid_min, Fixed32Gtlt{val: 5}, 1);

    test_case!(fixed32_exclusive_gt_lt_valid_above, Fixed32ExLtgt{val: 11}, 0);
    test_case!(fixed32_exclusive_gt_lt_valid_below, Fixed32ExLtgt{val: 4}, 0);
    test_case!(fixed32_exclusive_gt_lt_invalid, Fixed32ExLtgt{val: 7}, 1);
    test_case!(fixed32_exclusive_gt_lt_invalid_max, Fixed32ExLtgt{val: 10}, 1);
    test_case!(fixed32_exclusive_gt_lt_invalid_min, Fixed32ExLtgt{val: 5}, 1);

    test_case!(fixed32_gte_lte_valid, Fixed32Gtelte{val: 200}, 0);
    test_case!(fixed32_gte_lte_valid_max, Fixed32Gtelte{val: 256}, 0);
    test_case!(fixed32_gte_lte_valid_min, Fixed32Gtelte{val: 128}, 0);
    test_case!(fixed32_gte_lte_invalid_above, Fixed32Gtelte{val: 300}, 1);
    test_case!(fixed32_gte_lte_invalid_below, Fixed32Gtelte{val: 100}, 1);

    test_case!(fixed32_exclusive_gte_lte_valid_above, Fixed32ExGtelte{val: 300}, 0);
    test_case!(fixed32_exclusive_gte_lte_valid_below, Fixed32ExGtelte{val: 100}, 0);
    test_case!(fixed32_exclusive_gte_lte_valid_max, Fixed32ExGtelte{val: 256}, 0);
    test_case!(fixed32_exclusive_gte_lte_valid_min, Fixed32ExGtelte{val: 128}, 0);
    test_case!(fixed32_exclusive_gte_lte_invalid, Fixed32ExGtelte{val: 200}, 1);

    test_case!(fixed32_ignore_empty_gte_lte_valid, Fixed32Ignore{val: 0}, 0);
}
#[cfg(test)]
mod fixed64 {
    use super::*;
    test_case!(fixed64_none_valid, Fixed64None{val: 123}, 0);

    test_case!(fixed64_const_valid, Fixed64Const{val: 1}, 0);
    test_case!(fixed64_const_invalid, Fixed64Const{val: 2}, 1);

    test_case!(fixed64_in_valid, Fixed64In{val: 3}, 0);
    test_case!(fixed64_in_invalid, Fixed64In{val: 5}, 1);

    test_case!(fixed64_not_in_valid, Fixed64NotIn{val: 1}, 0);
    test_case!(fixed64_not_in_invalid, Fixed64NotIn{val: 0}, 1);

    test_case!(fixed64_lt_valid, Fixed64Lt{val: 4}, 0);
    test_case!(fixed64_lt_invalid_equal, Fixed64Lt{val: 5}, 1);
    test_case!(fixed64_lt_invalid, Fixed64Lt{val: 6}, 1);

    test_case!(fixed64_lte_valid, Fixed64Lte{val: 63}, 0);
    test_case!(fixed64_lte_valid_equal, Fixed64Lte{val: 64}, 0);
    test_case!(fixed64_lte_invalid, Fixed64Lte{val: 65}, 1);

    test_case!(fixed64_gt_valid, Fixed64Gt{val: 17}, 0);
    test_case!(fixed64_gt_invalid_equal, Fixed64Gt{val: 16}, 1);
    test_case!(fixed64_gt_invalid, Fixed64Gt{val: 15}, 1);

    test_case!(fixed64_gte_valid, Fixed64Gte{val: 9}, 0);
    test_case!(fixed64_gte_valid_equal, Fixed64Gte{val: 8}, 0);
    test_case!(fixed64_gte_invalid, Fixed64Gte{val: 7}, 1);

    test_case!(fixed64_gt_lt_valid, Fixed64Gtlt{val: 7}, 0);
    test_case!(fixed64_gt_lt_invalid_above, Fixed64Gtlt{val: 11}, 1);
    test_case!(fixed64_gt_lt_invalid_below, Fixed64Gtlt{val: 1}, 1);
    test_case!(fixed64_gt_lt_invalid_max, Fixed64Gtlt{val: 10}, 1);
    test_case!(fixed64_gt_lt_invalid_min, Fixed64Gtlt{val: 5}, 1);

    test_case!(fixed64_exclusive_gt_lt_valid_above, Fixed64ExLtgt{val: 11}, 0);
    test_case!(fixed64_exclusive_gt_lt_valid_below, Fixed64ExLtgt{val: 4}, 0);
    test_case!(fixed64_exclusive_gt_lt_invalid, Fixed64ExLtgt{val: 7}, 1);
    test_case!(fixed64_exclusive_gt_lt_invalid_max, Fixed64ExLtgt{val: 10}, 1);
    test_case!(fixed64_exclusive_gt_lt_invalid_min, Fixed64ExLtgt{val: 5}, 1);

    test_case!(fixed64_gte_lte_valid, Fixed64Gtelte{val: 200}, 0);
    test_case!(fixed64_gte_lte_valid_max, Fixed64Gtelte{val: 256}, 0);
    test_case!(fixed64_gte_lte_valid_min, Fixed64Gtelte{val: 128}, 0);
    test_case!(fixed64_gte_lte_invalid_above, Fixed64Gtelte{val: 300}, 1);
    test_case!(fixed64_gte_lte_invalid_below, Fixed64Gtelte{val: 100}, 1);

    test_case!(fixed64_exclusive_gte_lte_valid_above, Fixed64ExGtelte{val: 300}, 0);
    test_case!(fixed64_exclusive_gte_lte_valid_below, Fixed64ExGtelte{val: 100}, 0);
    test_case!(fixed64_exclusive_gte_lte_valid_max, Fixed64ExGtelte{val: 256}, 0);
    test_case!(fixed64_exclusive_gte_lte_valid_min, Fixed64ExGtelte{val: 128}, 0);
    test_case!(fixed64_exclusive_gte_lte_invalid, Fixed64ExGtelte{val: 200}, 1);

    test_case!(fixed64_ignore_empty_gte_lte_valid, Fixed64Ignore{val: 0}, 0);
}
#[cfg(test)]
mod sfixed32 {
    use super::*;
    test_case!(sfixed32_none_valid, SFixed32None{val: 123}, 0);

    test_case!(sfixed32_const_valid, SFixed32Const{val: 1}, 0);
    test_case!(sfixed32_const_invalid, SFixed32Const{val: 2}, 1);

    test_case!(sfixed32_in_valid, SFixed32In{val: 3}, 0);
    test_case!(sfixed32_in_invalid, SFixed32In{val: 5}, 1);

    test_case!(sfixed32_not_in_valid, SFixed32NotIn{val: 1}, 0);
    test_case!(sfixed32_not_in_invalid, SFixed32NotIn{val: 0}, 1);

    test_case!(sfixed32_lt_valid, SFixed32Lt{val: -1}, 0);
    test_case!(sfixed32_lt_invalid_equal, SFixed32Lt{val: 0}, 1);
    test_case!(sfixed32_lt_invalid, SFixed32Lt{val: 1}, 1);

    test_case!(sfixed32_lte_valid, SFixed32Lte{val: 63}, 0);
    test_case!(sfixed32_lte_valid_equal, SFixed32Lte{val: 64}, 0);
    test_case!(sfixed32_lte_invalid, SFixed32Lte{val: 65}, 1);

    test_case!(sfixed32_gt_valid, SFixed32Gt{val: 17}, 0);
    test_case!(sfixed32_gt_invalid_equal, SFixed32Gt{val: 16}, 1);
    test_case!(sfixed32_gt_invalid, SFixed32Gt{val: 15}, 1);

    test_case!(sfixed32_gte_valid, SFixed32Gte{val: 9}, 0);
    test_case!(sfixed32_gte_valid_equal, SFixed32Gte{val: 8}, 0);
    test_case!(sfixed32_gte_invalid, SFixed32Gte{val: 7}, 1);

    test_case!(sfixed32_gt_lt_valid, SFixed32Gtlt{val: 5}, 0);
    test_case!(sfixed32_gt_lt_invalid_above, SFixed32Gtlt{val: 11}, 1);
    test_case!(sfixed32_gt_lt_invalid_below, SFixed32Gtlt{val: -1}, 1);
    test_case!(sfixed32_gt_lt_invalid_max, SFixed32Gtlt{val: 10}, 1);
    test_case!(sfixed32_gt_lt_invalid_min, SFixed32Gtlt{val: 0}, 1);

    test_case!(sfixed32_exclusive_gt_lt_valid_above, SFixed32ExLtgt{val: 11}, 0);
    test_case!(sfixed32_exclusive_gt_lt_valid_below, SFixed32ExLtgt{val: -1}, 0);
    test_case!(sfixed32_exclusive_gt_lt_invalid, SFixed32ExLtgt{val: 5}, 1);
    test_case!(sfixed32_exclusive_gt_lt_invalid_max, SFixed32ExLtgt{val: 10}, 1);
    test_case!(sfixed32_exclusive_gt_lt_invalid_min, SFixed32ExLtgt{val: 0}, 1);

    test_case!(sfixed32_gte_lte_valid, SFixed32Gtelte{val: 200}, 0);
    test_case!(sfixed32_gte_lte_valid_max, SFixed32Gtelte{val: 256}, 0);
    test_case!(sfixed32_gte_lte_valid_min, SFixed32Gtelte{val: 128}, 0);
    test_case!(sfixed32_gte_lte_invalid_above, SFixed32Gtelte{val: 300}, 1);
    test_case!(sfixed32_gte_lte_invalid_below, SFixed32Gtelte{val: 100}, 1);

    test_case!(sfixed32_exclusive_gte_lte_valid_above, SFixed32ExGtelte{val: 300}, 0);
    test_case!(sfixed32_exclusive_gte_lte_valid_below, SFixed32ExGtelte{val: 100}, 0);
    test_case!(sfixed32_exclusive_gte_lte_valid_max, SFixed32ExGtelte{val: 256}, 0);
    test_case!(sfixed32_exclusive_gte_lte_valid_min, SFixed32ExGtelte{val: 128}, 0);
    test_case!(sfixed32_exclusive_gte_lte_invalid, SFixed32ExGtelte{val: 200}, 1);

    test_case!(sfixed32_ignore_empty_gte_lte_valid, SFixed32Ignore{val: 0}, 0);
}
#[cfg(test)]
mod sfixed64 {
    use super::*;
    test_case!(sfixed64_none_valid, SFixed64None{val: 123}, 0);

    test_case!(sfixed64_const_valid, SFixed64Const{val: 1}, 0);
    test_case!(sfixed64_const_invalid, SFixed64Const{val: 2}, 1);

    test_case!(sfixed64_in_valid, SFixed64In{val: 3}, 0);
    test_case!(sfixed64_in_invalid, SFixed64In{val: 5}, 1);

    test_case!(sfixed64_not_in_valid, SFixed64NotIn{val: 1}, 0);
    test_case!(sfixed64_not_in_invalid, SFixed64NotIn{val: 0}, 1);

    test_case!(sfixed64_lt_valid, SFixed64Lt{val: -1}, 0);
    test_case!(sfixed64_lt_invalid_equal, SFixed64Lt{val: 0}, 1);
    test_case!(sfixed64_lt_invalid, SFixed64Lt{val: 1}, 1);

    test_case!(sfixed64_lte_valid, SFixed64Lte{val: 63}, 0);
    test_case!(sfixed64_lte_valid_equal, SFixed64Lte{val: 64}, 0);
    test_case!(sfixed64_lte_invalid, SFixed64Lte{val: 65}, 1);

    test_case!(sfixed64_gt_valid, SFixed64Gt{val: 17}, 0);
    test_case!(sfixed64_gt_invalid_equal, SFixed64Gt{val: 16}, 1);
    test_case!(sfixed64_gt_invalid, SFixed64Gt{val: 15}, 1);

    test_case!(sfixed64_gte_valid, SFixed64Gte{val: 9}, 0);
    test_case!(sfixed64_gte_valid_equal, SFixed64Gte{val: 8}, 0);
    test_case!(sfixed64_gte_invalid, SFixed64Gte{val: 7}, 1);

    test_case!(sfixed64_gt_lt_valid, SFixed64Gtlt{val: 5}, 0);
    test_case!(sfixed64_gt_lt_invalid_above, SFixed64Gtlt{val: 11}, 1);
    test_case!(sfixed64_gt_lt_invalid_below, SFixed64Gtlt{val: -1}, 1);
    test_case!(sfixed64_gt_lt_invalid_max, SFixed64Gtlt{val: 10}, 1);
    test_case!(sfixed64_gt_lt_invalid_min, SFixed64Gtlt{val: 0}, 1);

    test_case!(sfixed64_exclusive_gt_lt_valid_above, SFixed64ExLtgt{val: 11}, 0);
    test_case!(sfixed64_exclusive_gt_lt_valid_below, SFixed64ExLtgt{val: -1}, 0);
    test_case!(sfixed64_exclusive_gt_lt_invalid, SFixed64ExLtgt{val: 5}, 1);
    test_case!(sfixed64_exclusive_gt_lt_invalid_max, SFixed64ExLtgt{val: 10}, 1);
    test_case!(sfixed64_exclusive_gt_lt_invalid_min, SFixed64ExLtgt{val: 0}, 1);

    test_case!(sfixed64_gte_lte_valid, SFixed64Gtelte{val: 200}, 0);
    test_case!(sfixed64_gte_lte_valid_max, SFixed64Gtelte{val: 256}, 0);
    test_case!(sfixed64_gte_lte_valid_min, SFixed64Gtelte{val: 128}, 0);
    test_case!(sfixed64_gte_lte_invalid_above, SFixed64Gtelte{val: 300}, 1);
    test_case!(sfixed64_gte_lte_invalid_below, SFixed64Gtelte{val: 100}, 1);

    test_case!(sfixed64_exclusive_gte_lte_valid_above, SFixed64ExGtelte{val: 300}, 0);
    test_case!(sfixed64_exclusive_gte_lte_valid_below, SFixed64ExGtelte{val: 100}, 0);
    test_case!(sfixed64_exclusive_gte_lte_valid_max, SFixed64ExGtelte{val: 256}, 0);
    test_case!(sfixed64_exclusive_gte_lte_valid_min, SFixed64ExGtelte{val: 128}, 0);
    test_case!(sfixed64_exclusive_gte_lte_invalid, SFixed64ExGtelte{val: 200}, 1);

    test_case!(sfixed64_ignore_empty_gte_lte_valid, SFixed64Ignore{val: 0}, 0);
}
#[cfg(test)]
mod bool {
    use super::*;
    test_case!(bool_none_valid, BoolNone{val: true}, 0);

    test_case!(bool_const_true_valid, BoolConstTrue{val: true}, 0);
    test_case!(bool_const_true_invalid, BoolConstTrue{val: false}, 1);
    test_case!(bool_const_false_valid, BoolConstFalse{val: false}, 0);
    test_case!(bool_const_false_invalid, BoolConstFalse{val: true}, 1);
}
#[cfg(test)]
mod string {
    use super::*;
    test_case!(string_none_valid, StringNone{val: "quux".to_string()}, 0);
    test_case!(string_const_valid, StringConst{val: "foo".to_string()}, 0);
    test_case!(string_const_invalid, StringConst{val: "bar".to_string()}, 1);

    test_case!(string_in_valid, StringIn{val: "bar".to_string()}, 0);
    test_case!(string_in_invalid, StringIn{val: "quux".to_string()}, 1);

    test_case!(string_not_in_valid, StringNotIn{val: "quux".to_string()}, 0);
    test_case!(string_not_in_invalid, StringNotIn{val: "fizz".to_string()}, 1);

    test_case!(string_len_valid, StringLen{val: "baz".to_string()}, 0);
    test_case!(string_len_valid_multibyte, StringLen{val: "你好吖".to_string()}, 0);

    test_case!(string_len_invalid_lt, StringLen{val: "go".to_string()}, 1);
    test_case!(string_len_invalid_gt, StringLen{val: "fizz".to_string()}, 1);
    test_case!(string_len_invalid_multibyte, StringLen{val: "你好".to_string()}, 1);

    test_case!(string_min_len_valid, StringMinLen{val: "protoc".to_string()}, 0);
    test_case!(string_min_len_valid_min, StringMinLen{val: "baz".to_string()}, 0);
    test_case!(string_min_len_invalid, StringMinLen{val: "go".to_string()}, 1);
    test_case!(string_min_len_invalid_multibyte, StringMinLen{val: "你好".to_string()}, 1);

    test_case!(string_max_len_valid, StringMaxLen{val: "foo".to_string()}, 0);
    test_case!(string_max_len_valid_max, StringMaxLen{val: "proto".to_string()}, 0);
    test_case!(string_max_len_valid_multibyte, StringMaxLen{val: "你好你好".to_string()}, 0);
    test_case!(string_max_len_invalid, StringMaxLen{val: "1234567890".to_string()}, 1);

    test_case!(string_min_max_len_valid, StringMinMaxLen{val: "quux".to_string()}, 0);
    test_case!(string_min_max_len_valid_min, StringMinMaxLen{val: "foo".to_string()}, 0);
    test_case!(string_min_max_len_valid_max, StringMinMaxLen{val: "proto".to_string()}, 0);
    test_case!(string_min_max_len_valid_multibyte, StringMinMaxLen{val: "你好你好".to_string()}, 0);
    test_case!(string_min_max_len_invalid_below, StringMinMaxLen{val: "go".to_string()}, 1);
    test_case!(string_min_max_len_invalid_above, StringMinMaxLen{val: "validate".to_string()}, 1);

    test_case!(string_equal_min_max_len_valid, StringEqualMinMaxLen{val: "proto".to_string()}, 0);
    test_case!(string_equal_min_max_len_invalid, StringEqualMinMaxLen{val: "validate".to_string()}, 1);

    test_case!(string_len_bytes_valid, StringLenBytes{val: "pace".to_string()}, 0);
    test_case!(string_len_bytes_invalid_lt, StringLenBytes{val: "val".to_string()}, 1);
    test_case!(string_len_bytes_invalid_gt, StringLenBytes{val: "world".to_string()}, 1);
    test_case!(string_len_bytes_invalid_multibyte, StringLenBytes{val: "世界和平".to_string()}, 1);

    test_case!(string_min_bytes_valid, StringMinBytes{val: "proto".to_string()}, 0);
    test_case!(string_min_bytes_valid_min, StringMinBytes{val: "quux".to_string()}, 0);
    test_case!(string_min_bytes_valid_multibyte, StringMinBytes{val: "你好".to_string()}, 0);
    test_case!(string_min_bytes_invalid, StringMinBytes{val: "".to_string()}, 1);

    test_case!(string_max_bytes_valid, StringMaxBytes{val: "foo".to_string()}, 0);
    test_case!(string_max_bytes_valid_max, StringMaxBytes{val: "12345678".to_string()}, 0);
    test_case!(string_max_bytes_invalid, StringMaxBytes{val: "123456789".to_string()}, 1);
    test_case!(string_max_bytes_invalid_multibyte, StringMaxBytes{val: "你好你好你好".to_string()}, 1);

    test_case!(string_min_max_bytes_valid, StringMinMaxBytes{val: "protoc".to_string()}, 0);
    test_case!(string_min_max_bytes_valid_min, StringMinMaxBytes{val: "quux".to_string()}, 0);
    test_case!(string_min_max_bytes_valid_max, StringMinMaxBytes{val: "fizzbuzz".to_string()}, 0);
    test_case!(string_min_max_bytes_valid_multibyte, StringMinMaxBytes{val: "你好".to_string()}, 0);
    test_case!(string_min_max_bytes_invalid_below, StringMinMaxBytes{val: "foo".to_string()}, 1);
    test_case!(string_min_max_bytes_invalid_above, StringMinMaxBytes{val: "你好你好你".to_string()}, 1);

    test_case!(string_equal_min_max_bytes_valid, StringEqualMinMaxBytes{val: "protoc".to_string()}, 0);
    test_case!(string_equal_min_max_bytes_invalid, StringEqualMinMaxBytes{val: "foo".to_string()}, 1);

    test_case!(string_pattern_valid, StringPattern{val: "Foo123".to_string()}, 0);
    test_case!(string_pattern_invalid, StringPattern{val: "!@#$%^&*()".to_string()}, 1);
    test_case!(string_pattern_invalid_empty, StringPattern{val: "".to_string()}, 1);
    test_case!(string_pattern_invalid_null, StringPattern{val: "a\000".to_string()}, 1);
    test_case!(string_pattern_escapes_valid, StringPatternEscapes{val: "* \\ x".to_string()}, 0);
    test_case!(string_pattern_escapes_invalid, StringPatternEscapes{val: "invalid".to_string()}, 1);
    test_case!(string_pattern_escapes_invalid_empty, StringPatternEscapes{val: "".to_string()}, 1);

    test_case!(string_prefix_valid, StringPrefix{val: "foobar".to_string()}, 0);
    test_case!(string_prefix_valid_only, StringPrefix{val: "foo".to_string()}, 0);
    test_case!(string_prefix_invalid, StringPrefix{val: "bar".to_string()}, 1);
    test_case!(string_prefix_invalid_case_sensitive, StringPrefix{val: "Foobar".to_string()}, 1);

    test_case!(string_contains_valid, StringContains{val: "candy bars".to_string()}, 0);
    test_case!(string_contains_valid_only, StringContains{val: "bar".to_string()}, 0);
    test_case!(string_contains_invalid, StringContains{val: "candy bazs".to_string()}, 1);
    test_case!(string_contains_invalid_case_sensitive, StringContains{val: "Candy Bars".to_string()}, 1);

    test_case!(string_not_contains_valid, StringNotContains{val: "candy bazs".to_string()}, 0);
    test_case!(string_not_contains_valid_case_sensitive, StringNotContains{val: "Candy Bars".to_string()}, 0);
    test_case!(string_not_contains_invalid, StringNotContains{val: "candy bars".to_string()}, 1);
    test_case!(string_not_contains_invalid_equal, StringNotContains{val: "bar".to_string()}, 1);

    test_case!(string_suffix_valid, StringSuffix{val: "foobaz".to_string()}, 0);
    test_case!(string_suffix_valid_only, StringSuffix{val: "baz".to_string()}, 0);
    test_case!(string_suffix_invalid, StringSuffix{val: "foobar".to_string()}, 1);
    test_case!(string_suffix_invalid_case_sensitive, StringSuffix{val: "FooBaz".to_string()}, 1);

    test_case!(string_email_valid, StringEmail{val: "foo@bar.com".to_string()}, 0);
    test_case!(string_email_valid_name, StringEmail{val: "John Smith <foo@bar.com>".to_string()}, 0);
    test_case!(string_email_invalid, StringEmail{val: "foobar".to_string()}, 1);
    test_case!(string_email_invalid_local_segment_too_long, StringEmail{val: "x0123456789012345678901234567890123456789012345678901234567890123456789@example.com".to_string()}, 1);
    test_case!(string_email_invalid_hostname_too_long, StringEmail{val: "foo@x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}, 1);
    test_case!(string_email_invalid_bad_hostname, StringEmail{val: "foo@-bar.com".to_string()}, 1);
    test_case!(string_email_empty, StringEmail{val: "".to_string()}, 1);

    test_case!(string_address_valid_hostname, StringAddress{val: "example.com".to_string()}, 0);
    test_case!(string_address_valid_hostname_uppercase, StringAddress{val: "ASD.example.com".to_string()}, 0);
    test_case!(string_address_valid_hostname_hyphens, StringAddress{val: "foo-bar.com".to_string()}, 0);
    test_case!(string_address_valid_hostname_trailing_dot, StringAddress{val: "example.com.".to_string()}, 0);
    test_case!(string_address_invalid_hostname, StringAddress{val: "!@#$%^&".to_string()}, 1);
    test_case!(string_address_invalid_hostname_underscore, StringAddress{val: "foo_bar.com".to_string()}, 1);
    test_case!(string_address_invalid_hostname_too_long, StringAddress{val: "x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}, 1);
    test_case!(string_address_invalid_hostname_trailing_hyphens, StringAddress{val: "foo-bar-.com".to_string()}, 1);
    test_case!(string_address_invalid_hostname_leading_hyphens, StringAddress{val: "foo-bar.-com".to_string()}, 1);
    test_case!(string_address_invalid_hostname_empty, StringAddress{val: "asd..asd.com".to_string()}, 1);
    test_case!(string_address_invalid_hostname_idns, StringAddress{val: "你好.com".to_string()}, 1);
    test_case!(string_address_valid_ip_v4, StringAddress{val: "192.168.0.1".to_string()}, 0);
    test_case!(string_address_valid_ip_v6, StringAddress{val: "3e::99".to_string()}, 0);
    test_case!(string_address_invalid_ip, StringAddress{val: "ff::fff::0b".to_string()}, 1);

    test_case!(string_hostname_valid, StringHostname{val: "example.com".to_string()}, 0);
    test_case!(string_hostname_valid_uppercase, StringHostname{val: "ASD.example.com".to_string()}, 0);
    test_case!(string_hostname_valid_hyphens, StringHostname{val: "foo-bar.com".to_string()}, 0);
    test_case!(string_hostname_valid_trailing_dot, StringHostname{val: "example.com.".to_string()}, 0);
    test_case!(string_hostname_invalid, StringHostname{val: "!@#$%^&".to_string()}, 1);
    test_case!(string_hostname_invalid_underscore, StringHostname{val: "foo_bar.com".to_string()}, 1);
    test_case!(string_hostname_invalid_too_long, StringHostname{val: "x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}, 1);
    test_case!(string_hostname_invalid_trailing_hyphens, StringHostname{val: "foo-bar-.com".to_string()}, 1);
    test_case!(string_hostname_invalid_leading_hyphens, StringHostname{val: "foo-bar.-com".to_string()}, 1);
    test_case!(string_hostname_invalid_empty, StringHostname{val: "asd..asd.com".to_string()}, 1);
    test_case!(string_hostname_invalid_idns, StringHostname{val: "你好.com".to_string()}, 1);

    test_case!(string_ip_valid_v4, StringIp{val: "192.168.0.1".to_string()}, 0);
    test_case!(string_ip_valid_v6, StringIp{val: "3e::99".to_string()}, 0);
    test_case!(string_ip_invalid, StringIp{val: "foobar".to_string()}, 1);

    test_case!(string_ipv4_valid, StringIPv4{val: "192.168.0.1".to_string()}, 0);
    test_case!(string_ipv4_invalid, StringIPv4{val: "foobar".to_string()}, 1);
    test_case!(string_ipv4_invalid_erroneous, StringIPv4{val: "256.0.0.0".to_string()}, 1);
    test_case!(string_ipv4_invalid_v6, StringIPv4{val: "3e::99".to_string()}, 1);

    test_case!(string_ipv6_valid, StringIPv6{val: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()}, 0);
    test_case!(string_ipv6_valid_collapsed, StringIPv6{val: "2001:db8:85a3::8a2e:370:7334".to_string()}, 0);
    test_case!(string_ipv6_invalid, StringIPv6{val: "foobar".to_string()}, 1);
    test_case!(string_ipv6_invalid_v4, StringIPv6{val: "192.168.0.1".to_string()}, 1);
    test_case!(string_ipv6_invalid_erroneous, StringIPv6{val: "ff::fff::0b".to_string()}, 1);

    test_case!(string_uri_valid_2, StringUri{val: "http://example.com/foo/bar?baz=quux".to_string()}, 0);
    test_case!(string_uri_invalid_2, StringUri{val: "!@#$%^&*%$#".to_string()}, 1);
    test_case!(string_uri_invalid_relative, StringUri{val: "/foo/bar?baz=quux".to_string()}, 1);
    test_case!(string_uri_valid_3, StringUriRef{val: "http://example.com/foo/bar?baz=quux".to_string()}, 0);
    test_case!(string_uri_valid_relative, StringUriRef{val: "/foo/bar?baz=quux".to_string()}, 0);
    test_case!(string_uri_invalid_3, StringUriRef{val: "!@#$%^&*%$#".to_string()}, 1);

    test_case!(string_uuid_valid_nil, StringUuid{val: "00000000-0000-0000-0000-000000000000".to_string()}, 0);
    test_case!(string_uuid_valid_v1, StringUuid{val: "b45c0c80-8880-11e9-a5b1-000000000000".to_string()}, 0);
    test_case!(string_uuid_valid_v1_case_insensitive, StringUuid{val: "B45C0C80-8880-11E9-A5B1-000000000000".to_string()}, 0);
    test_case!(string_uuid_valid_v2, StringUuid{val: "b45c0c80-8880-21e9-a5b1-000000000000".to_string()}, 0);
    test_case!(string_uuid_valid_v2_case_insensitive, StringUuid{val: "B45C0C80-8880-21E9-A5B1-000000000000".to_string()}, 0);
    test_case!(string_uuid_valid_v3, StringUuid{val: "a3bb189e-8bf9-3888-9912-ace4e6543002".to_string()}, 0);
    test_case!(string_uuid_valid_v3_case_insensitive, StringUuid{val: "A3BB189E-8BF9-3888-9912-ACE4E6543002".to_string()}, 0);
    test_case!(string_uuid_valid_v4, StringUuid{val: "8b208305-00e8-4460-a440-5e0dcd83bb0a".to_string()}, 0);
    test_case!(string_uuid_valid_v4_case_insensitive, StringUuid{val: "8B208305-00E8-4460-A440-5E0DCD83BB0A".to_string()}, 0);
    test_case!(string_uuid_valid_v5, StringUuid{val: "a6edc906-2f9f-5fb2-a373-efac406f0ef2".to_string()}, 0);
    test_case!(string_uuid_valid_v5_case_insensitive, StringUuid{val: "A6EDC906-2F9F-5FB2-A373-EFAC406F0EF2".to_string()}, 0);
    test_case!(string_uuid_invalid, StringUuid{val: "foobar".to_string()}, 1);
    test_case!(string_uuid_invalid_bad_uuid, StringUuid{val: "ffffffff-ffff-ffff-ffff-fffffffffffff".to_string()}, 1);
    test_case!(string_uuid_valid_ignore_empty, StringUuidIgnore{val: "".to_string()}, 0);

    test_case!(string_http_header_name_valid, StringHttpHeaderName{val: "clustername".to_string()}, 0);
    test_case!(string_http_header_name_valid_2, StringHttpHeaderName{val: ":path".to_string()}, 0);
    test_case!(string_http_header_name_valid_nums, StringHttpHeaderName{val: "cluster-123".to_string()}, 0);
    test_case!(string_http_header_name_valid_special_token, StringHttpHeaderName{val: "!+#&.%".to_string()}, 0);
    test_case!(string_http_header_name_valid_period, StringHttpHeaderName{val: "CLUSTER.NAME".to_string()}, 0);
    test_case!(string_http_header_name_invalid, StringHttpHeaderName{val: ":".to_string()}, 1);
    test_case!(string_http_header_name_invalid_2, StringHttpHeaderName{val: ":path:".to_string()}, 1);
    test_case!(string_http_header_name_invalid_space, StringHttpHeaderName{val: "cluster name".to_string()}, 1);
    test_case!(string_http_header_name_invalid_return, StringHttpHeaderName{val: "example\r".to_string()}, 1);
    test_case!(string_http_header_name_invalid_tab, StringHttpHeaderName{val: "example\t".to_string()}, 1);
    test_case!(string_http_header_name_invalid_slash, StringHttpHeaderName{val: "/test/long/url".to_string()}, 1);

    test_case!(string_http_header_value_valid, StringHttpHeaderValue{val: "cluster.name.123".to_string()}, 0);
    test_case!(string_http_header_value_valid_uppercase, StringHttpHeaderValue{val: "/TEST/LONG/URL".to_string()}, 0);
    test_case!(string_http_header_value_valid_spaces, StringHttpHeaderValue{val: "cluster name".to_string()}, 0);
    test_case!(string_http_header_value_valid_tab, StringHttpHeaderValue{val: "example\t".to_string()}, 0);
    test_case!(string_http_header_value_valid_special_token, StringHttpHeaderValue{val: "!#%&./+".to_string()}, 0);
    test_case!(string_http_header_value_invalid_nul, StringHttpHeaderValue{val: "foo\u{0000}bar".to_string()}, 1);
    test_case!(string_http_header_value_invalid_del, StringHttpHeaderValue{val: "\u{007f}".to_string()}, 1);
    test_case!(string_http_header_value_invalid, StringHttpHeaderValue{val: "example\r".to_string()}, 1);

    test_case!(string_non_strict_valid_header_valid, StringValidHeader{val: "cluster.name.123".to_string()}, 0);
    test_case!(string_non_strict_valid_header_valid_uppercase, StringValidHeader{val: "/TEST/LONG/URL".to_string()}, 0);
    test_case!(string_non_strict_valid_header_valid_spaces, StringValidHeader{val: "cluster name".to_string()}, 0);
    test_case!(string_non_strict_valid_header_valid_tab, StringValidHeader{val: "example\t".to_string()}, 0);
    test_case!(string_non_strict_valid_header_valid_del, StringValidHeader{val: "\u{007f}".to_string()}, 0);
    test_case!(string_non_strict_valid_header_invalid_nul, StringValidHeader{val: "foo\u{0000}bar".to_string()}, 1);
    test_case!(string_non_strict_valid_header_invalid_cr, StringValidHeader{val: "example\r".to_string()}, 1);
    test_case!(string_non_strict_valid_header_invalid_nl, StringValidHeader{val: "exa\u{000A}mple".to_string()}, 1);
}
#[cfg(test)]
mod bytes {
    use super::*;
    test_case!(bytes_none_valid, BytesNone{val: b"quux".to_vec()}, 0);

    test_case!(bytes_const_valid, BytesConst{val: b"foo".to_vec()}, 0);
    test_case!(bytes_const_invalid, BytesConst{val: b"bar".to_vec()}, 1);

    test_case!(bytes_in_valid, BytesIn{val: b"bar".to_vec()}, 0);
    test_case!(bytes_in_invalid, BytesIn{val: b"quux".to_vec()}, 1);
    test_case!(bytes_not_in_valid, BytesNotIn{val: b"quux".to_vec()}, 0);
    test_case!(bytes_not_in_invalid, BytesNotIn{val: b"fizz".to_vec()}, 1);

    test_case!(bytes_len_valid, BytesLen{val: b"baz".to_vec()}, 0);
    test_case!(bytes_len_invalid_lt, BytesLen{val: b"go".to_vec()}, 1);
    test_case!(bytes_len_invalid_gt, BytesLen{val: b"fizz".to_vec()}, 1);

    test_case!(bytes_min_len_valid, BytesMinLen{val: b"fizz".to_vec()}, 0);
    test_case!(bytes_min_len_valid_min, BytesMinLen{val: b"baz".to_vec()}, 0);
    test_case!(bytes_min_len_invalid, BytesMinLen{val: b"go".to_vec()}, 1);

    test_case!(bytes_max_len_valid, BytesMaxLen{val: b"foo".to_vec()}, 0);
    test_case!(bytes_max_len_valid_max, BytesMaxLen{val: b"proto".to_vec()}, 0);
    test_case!(bytes_max_len_invalid, BytesMaxLen{val: b"1234567890".to_vec()}, 1);

    test_case!(bytes_min_max_len_valid, BytesMinMaxLen{val: b"quux".to_vec()}, 0);
    test_case!(bytes_min_max_len_valid_min, BytesMinMaxLen{val: b"foo".to_vec()}, 0);
    test_case!(bytes_min_max_len_valid_max, BytesMinMaxLen{val: b"proto".to_vec()}, 0);
    test_case!(bytes_min_max_len_invalid_below, BytesMinMaxLen{val: b"go".to_vec()}, 1);
    test_case!(bytes_min_max_len_invalid_above, BytesMinMaxLen{val: b"validate".to_vec()}, 1);

    test_case!(bytes_equal_min_max_len_valid, BytesEqualMinMaxLen{val: b"proto".to_vec()}, 0);
    test_case!(bytes_equal_min_max_len_invalid, BytesEqualMinMaxLen{val: b"validate".to_vec()}, 1);

    test_case!(bytes_pattern_valid, BytesPattern{val: b"Foo123".to_vec()}, 0);
    // b"你好你好"
    test_case!(bytes_pattern_invalid, BytesPattern{val: b"\xE4\xBD\xA0\xE5\xA5\xBD\xE4\xBD\xA0\xE5\xA5\xBD".to_vec()}, 1);
    test_case!(bytes_pattern_invalid_empty, BytesPattern{val: b"".to_vec()}, 1);

    test_case!(bytes_prefix_valid, BytesPrefix{val: vec![0x99, 0x9f, 0x08]}, 0);
    test_case!(bytes_prefix_valid_only, BytesPrefix{val: vec![0x99]}, 0);
    test_case!(bytes_prefix_invalid, BytesPrefix{val: b"bar".to_vec()}, 1);

    test_case!(bytes_contains_valid, BytesContains{val: b"candy bars".to_vec()}, 0);
    test_case!(bytes_contains_valid_only, BytesContains{val: b"bar".to_vec()}, 0);
    test_case!(bytes_contains_invalid, BytesContains{val: b"candy bazs".to_vec()}, 1);

    test_case!(bytes_suffix_valid, BytesSuffix{val: vec![0x62, 0x75, 0x7A, 0x7A]}, 0);
    test_case!(bytes_suffix_valid_only, BytesSuffix{val: b"\x62\x75\x7A\x7A".to_vec()}, 0);
    test_case!(bytes_suffix_invalid, BytesSuffix{val: b"foobar".to_vec()}, 1);
    test_case!(bytes_suffix_invalid_case_sensitive, BytesSuffix{val: b"FooBaz".to_vec()}, 1);

    test_case!(bytes_ip_valid_v4, BytesIp{val: vec![0xC0, 0xA8, 0x00, 0x01]}, 0);
    test_case!(bytes_ip_valid_v6, BytesIp{val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34".to_vec()}, 0);
    test_case!(bytes_ip_invalid, BytesIp{val: b"foobar".to_vec()}, 1);

    test_case!(bytes_ipv4_valid, BytesIPv4{val: vec![0xC0, 0xA8, 0x00, 0x01]}, 0);
    test_case!(bytes_ipv4_invalid, BytesIPv4{val: b"foobar".to_vec()}, 1);
    test_case!(bytes_ipv4_invalid_v6, BytesIPv4{val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34".to_vec()}, 1);

    test_case!(bytes_ipv6_valid, BytesIPv6{val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34".to_vec()}, 0);
    test_case!(bytes_ipv6_invalid, BytesIPv6{val: b"fooar".to_vec()}, 1);
    test_case!(bytes_ipv6_invalid_v4, BytesIPv6{val: vec![0xC0, 0xA8, 0x00, 0x01]}, 1);

    test_case!(bytes_ipv6_valid_ignore_empty, BytesIPv6Ignore::default(), 0);
}
#[cfg(test)]
mod r#enum {
    use super::*;
    test_case!(enum_none_valid, EnumNone{val: TestEnum::One.into()}, 0);

    test_case!(enum_const_valid, EnumConst{val: TestEnum::Two.into()}, 0);
    test_case!(enum_const_invalid, EnumConst{val: TestEnum::One.into()}, 1);

    test_case!(enum_alias_const_valid, EnumAliasConst{val: TestEnumAlias::C.into()}, 0);
    test_case!(enum_alias_const_valid_alias, EnumAliasConst{val: TestEnumAlias::C.into()}, 0);
    test_case!(enum_alias_const_invalid, EnumAliasConst{val: TestEnumAlias::A.into()}, 1);

    test_case!(enum_defined_only_valid, EnumDefined{val: 0}, 0);
    test_case!(enum_defined_only_invalid, EnumDefined{val: i32::MAX}, 1);

    test_case!(enum_alias_defined_only_valid, EnumAliasDefined{val: 1}, 0);
    test_case!(enum_alias_defined_only_invalid, EnumAliasDefined{val: i32::MAX}, 1);

    test_case!(enum_in_valid, EnumIn{val: TestEnum::Two.into()}, 0);
    test_case!(enum_in_invalid, EnumIn{val: TestEnum::One.into()}, 1);

    test_case!(enum_alias_in_valid, EnumAliasIn{val: TestEnumAlias::A.into()}, 0);
    test_case!(enum_alias_in_valid_alias, EnumAliasIn{val: TestEnumAlias::A.into()}, 0);
    test_case!(enum_alias_in_invalid, EnumAliasIn{val: TestEnumAlias::B.into()}, 1);

    test_case!(enum_not_in_valid, EnumNotIn{val: TestEnum::Zero.into()}, 0);
    test_case!(enum_not_in_valid_undefined, EnumNotIn{val: i32::MAX}, 0);
    test_case!(enum_not_in_invalid, EnumNotIn{val: TestEnum::One.into()}, 1);

    test_case!(enum_alias_not_in_valid, EnumAliasNotIn{val: TestEnumAlias::A.into()}, 0);
    test_case!(enum_alias_not_in_invalid, EnumAliasNotIn{val: TestEnumAlias::B.into()}, 1);
    test_case!(enum_alias_not_in_invalid_alias, EnumAliasNotIn{val: TestEnumAlias::B.into()}, 1);

    test_case!(enum_external_defined_only_valid, EnumExternal{val: other_package::embed::Enumerated::Value.into()}, 0);
    test_case!(enum_external_defined_only_invalid, EnumExternal{val: i32::MAX}, 1);
    test_case!(enum_external_in_valid, EnumExternal3{foo: other_package::embed::FooNumber::Zero.into(), bar: 0}, 0);
    test_case!(enum_external_in_invalid, EnumExternal3{foo: other_package::embed::FooNumber::One.into(), bar: 0}, 1);
    test_case!(enum_external_not_in_valid, EnumExternal3{bar: yet_another_package::embed::BarNumber::Zero.into(), foo: 0}, 0);
    test_case!(enum_external_not_in_invalid, EnumExternal3{bar: yet_another_package::embed::BarNumber::One.into(), foo: 0}, 1);
    test_case!(enum_external_const_valid, EnumExternal4{sort_direction: sort::Direction::Asc.into()}, 0);
    test_case!(enum_external_const_invalid, EnumExternal4{sort_direction: sort::Direction::Desc.into()}, 1);

    test_case!(enum_repeated_defined_only_valid, RepeatedEnumDefined{val: vec![TestEnum::One.into(), TestEnum::Two.into()]}, 0);
    test_case!(enum_repeated_defined_only_invalid, RepeatedEnumDefined{val: vec![TestEnum::One.into(), i32::MAX]}, 1);
    test_case!(enum_repeated_external_defined_only_valid, RepeatedExternalEnumDefined{val: vec![other_package::embed::Enumerated::Value.into()]}, 0);
    test_case!(enum_repeated_external_defined_only_invalid, RepeatedExternalEnumDefined{val: vec![i32::MAX]}, 1);
    test_case!(enum_repeated_another_external_defined_only_valid, RepeatedYetAnotherExternalEnumDefined{val: vec![yet_another_package::embed::Enumerated::Value.into()]}, 0);
    test_case!(enum_repeated_external_in_valid, RepeatedEnumExternal{foo: vec![other_package::embed::FooNumber::Zero.into(), other_package::embed::FooNumber::Two.into()], bar: vec![]}, 0);
    test_case!(enum_repeated_external_in_invalid, RepeatedEnumExternal{foo: vec![other_package::embed::FooNumber::One.into()], bar: vec![]}, 1);
    test_case!(enum_repeated_external_not_in_valid, RepeatedEnumExternal{bar: vec![yet_another_package::embed::BarNumber::Zero.into(), yet_another_package::embed::BarNumber::Two.into()], foo: vec![]}, 0);
    test_case!(enum_repeated_external_not_in_invalid, RepeatedEnumExternal{bar: vec![yet_another_package::embed::BarNumber::One.into()], foo: vec![]}, 1);

    test_case!(enum_map_defined_only_valid, MapEnumDefined{val: HashMap::from([("foo".to_string(), TestEnum::Two.into())])}, 0);
    test_case!(enum_map_defined_only_invalid, MapEnumDefined{val: HashMap::from([("foo".to_string(), i32::MAX)])}, 1);
    test_case!(enum_map_external_defined_only_valid, MapExternalEnumDefined{val: HashMap::from([("foo".to_string(), other_package::embed::Enumerated::Value.into())])}, 0);
    test_case!(enum_map_external_defined_only_invalid, MapExternalEnumDefined{val: HashMap::from([("foo".to_string(), i32::MAX)])}, 1);
}
#[cfg(test)]
mod message {
    use super::*;
    test_case!(message_none_valid, MessageNone{val: Some(message_none::NoneMsg::default())}, 0);
    test_case!(message_none_valid_unset, MessageNone::default(), 0);

    test_case!(message_disabled_valid, MessageDisabled{val: 456}, 0);
    test_case!(message_disabled_valid_invalid_field, MessageDisabled{val: 0}, 0);

    test_case!(message_ignored_valid, MessageIgnored{val: 456}, 0);
    test_case!(message_ignored_valid_invalid_field, MessageIgnored{val: 0}, 0);

    test_case!(message_field_valid, cases::Message{val: Some(TestMsg{r#const: "foo".to_string(), ..TestMsg::default()})}, 0);
    test_case!(message_field_valid_unset, cases::Message::default(), 0);
    test_case!(message_field_invalid, cases::Message{val: Some(TestMsg::default())}, 1);
    test_case!(message_field_invalid_transitive, cases::Message{val: Some(TestMsg{r#const: "foo".to_string(), nested: Some(Box::new(TestMsg::default()))})}, 1);

    test_case!(message_skip_valid, MessageSkip{val: Some(TestMsg::default())}, 0);

    test_case!(message_required_valid, MessageRequired{val: Some(TestMsg{r#const: "foo".to_string(), ..TestMsg::default()})}, 0);
    test_case!(message_required_valid_oneof, MessageRequiredOneof{one: Some(One::Val(TestMsg{r#const: "foo".to_string(), ..TestMsg::default()}))}, 0);
    test_case!(message_required_invalid, MessageRequired::default(), 1);
    test_case!(message_required_invalid_oneof, MessageRequiredOneof::default(), 1);

    test_case!(message_cross_package_embed_none_valid, MessageCrossPackage{val: Some(other_package::Embed{val: 1})}, 0);
    test_case!(message_cross_package_embed_none_valid_nil, MessageCrossPackage::default(), 0);
    test_case!(message_cross_package_embed_none_valid_empty, MessageCrossPackage{val: Some(other_package::Embed::default())}, 1);
    test_case!(message_cross_package_embed_none_invalid, MessageCrossPackage{val: Some(other_package::Embed{val: -1})}, 1);

    test_case!(message_required_valid_2, MessageRequiredButOptional{val: Some(TestMsg{r#const: "foo".to_string(), ..TestMsg::default()})}, 0);
    test_case!(message_required_valid_unset, MessageRequiredButOptional::default(), 0);
}
#[cfg(test)]
mod repeated {
    use super::*;

    test_case!(repeated_none_valid, RepeatedNone{val: vec![1, 2, 3]}, 0);

    test_case!(repeated_embed_none_valid, RepeatedEmbedNone{val: vec![cases::Embed{val: 1}]}, 0);
    test_case!(repeated_embed_none_valid_nil, RepeatedEmbedNone::default(), 0);
    test_case!(repeated_embed_none_valid_empty, RepeatedEmbedNone{val: vec![]}, 0);
    test_case!(repeated_embed_none_invalid, RepeatedEmbedNone{val: vec![cases::Embed{val: -1}]}, 1);
    test_case!(repeated_cross_package_embed_none_valid, RepeatedEmbedCrossPackageNone{val: vec![other_package::Embed{val: 1}]}, 0);
    test_case!(repeated_cross_package_embed_none_valid_nil, RepeatedEmbedCrossPackageNone::default(), 0);
    test_case!(repeated_cross_package_embed_none_valid_empty, RepeatedEmbedCrossPackageNone{val: vec![]}, 0);
    test_case!(repeated_cross_package_embed_none_invalid, RepeatedEmbedCrossPackageNone{val: vec![other_package::Embed{val: -1}]}, 1);

    test_case!(repeated_min_valid, RepeatedMin{val: vec![cases::Embed{val: 1}, cases::Embed{val: 2}, cases::Embed{val: 3}]}, 0);
    test_case!(repeated_min_valid_equal, RepeatedMin{val: vec![cases::Embed{val: 1}, cases::Embed{val: 2}]}, 0);
    test_case!(repeated_min_invalid, RepeatedMin{val: vec![cases::Embed{val: 1}]}, 1);
    test_case!(repeated_min_invalid_element, RepeatedMin{val: vec![cases::Embed{val: 1}, cases::Embed{val: -1}]}, 1);

    test_case!(repeated_max_valid, RepeatedMax{val: vec![1., 2.]}, 0);
    test_case!(repeated_max_valid_equal, RepeatedMax{val: vec![1., 2., 3.]}, 0);
    test_case!(repeated_max_invalid, RepeatedMax{val: vec![1., 2., 3., 4.]}, 1);

    test_case!(repeated_min_max_valid, RepeatedMinMax{val: vec![1, 2, 3]}, 0);
    test_case!(repeated_min_max_valid_min, RepeatedMinMax{val: vec![1, 2]}, 0);
    test_case!(repeated_min_max_valid_max, RepeatedMinMax{val: vec![1, 2, 3, 4]}, 0);
    test_case!(repeated_min_max_invalid_below, RepeatedMinMax{val: vec![]}, 1);
    test_case!(repeated_min_max_invalid_above, RepeatedMinMax{val: vec![1, 2, 3, 4, 5]}, 1);

    test_case!(repeated_exact_valid, RepeatedExact{val: vec![1, 2, 3]}, 0);
    test_case!(repeated_exact_invalid_below, RepeatedExact{val: vec![1, 2]}, 1);
    test_case!(repeated_exact_invalid_above, RepeatedExact{val: vec![1, 2, 3, 4]}, 1);

    test_case!(repeated_unique_valid, RepeatedUnique{val: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]}, 0);
    test_case!(repeated_unique_valid_empty, RepeatedUnique::default(), 0);
    test_case!(repeated_unique_valid_case_sensitivity, RepeatedUnique{val: vec!["foo".to_string(), "Foo".to_string()]}, 0);
    test_case!(repeated_unique_invalid, RepeatedUnique{val: vec!["foo".to_string(), "bar".to_string(), "foo".to_string(), "baz".to_string()]}, 1);

    test_case!(repeated_items_valid, RepeatedItemRule{val: vec![1., 2., 3.]}, 0);
    test_case!(repeated_items_valid_empty, RepeatedItemRule{val: vec![]}, 0);
    test_case!(repeated_items_valid_pattern, RepeatedItemPattern{val: vec!["Alpha".to_string(), "Beta123".to_string()]}, 0);
    test_case!(repeated_items_invalid, RepeatedItemRule{val: vec![1., -2., 3.]}, 1);
    test_case!(repeated_items_invalid_pattern, RepeatedItemPattern{val: vec!["Alpha".to_string(), "!@#$%^&*()".to_string()]}, 1);
    test_case!(repeated_items_invalid_in, RepeatedItemIn{val: vec!["baz".to_string()]}, 1);
    test_case!(repeated_items_valid_in, RepeatedItemIn{val: vec!["foo".to_string()]}, 0);
    test_case!(repeated_items_invalid_not_in, RepeatedItemNotIn{val: vec!["foo".to_string()]}, 1);
    test_case!(repeated_items_valid_not_in, RepeatedItemNotIn{val: vec!["baz".to_string()]}, 0);

    test_case!(repeated_items_invalid_enum_in, RepeatedEnumIn{val: vec![1]}, 1);
    test_case!(repeated_items_valid_enum_in, RepeatedEnumIn{val: vec![0]}, 0);
    test_case!(repeated_items_invalid_enum_not_in, RepeatedEnumNotIn{val: vec![0]}, 1);
    test_case!(repeated_items_valid_enum_not_in, RepeatedEnumNotIn{val: vec![1]}, 0);
    test_case!(repeated_items_invalid_embedded_enum_in, RepeatedEmbeddedEnumIn{val: vec![1]}, 1);
    test_case!(repeated_items_valid_embedded_enum_in, RepeatedEmbeddedEnumIn{val: vec![0]}, 0);
    test_case!(repeated_items_invalid_embedded_enum_not_in, RepeatedEmbeddedEnumNotIn{val: vec![0]}, 1);
    test_case!(repeated_items_valid_embedded_enum_not_in, RepeatedEmbeddedEnumNotIn{val: vec![1]}, 0);

    test_case!(repeated_items_invalid_any_in, RepeatedAnyIn{val: vec![Any{type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(), value: vec![]}]}, 1);
    test_case!(repeated_items_valid_any_in, RepeatedAnyIn{val: vec![Any{type_url: "type.googleapis.com/google.protobuf.Duration".to_string(), value: vec![]}]}, 0);
    test_case!(repeated_items_invalid_any_not_in, RepeatedAnyNotIn{val: vec![Any{type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(), value: vec![]}]}, 1);
    test_case!(repeated_items_valid_any_not_in, RepeatedAnyNotIn{val: vec![Any{type_url: "type.googleapis.com/google.protobuf.Duration".to_string(), value: vec![]}]}, 0);

    test_case!(repeated_embed_skip_valid, RepeatedEmbedSkip{val: vec![cases::Embed{val: 1}]}, 0);
    test_case!(repeated_embed_skip_valid_invalid_element, RepeatedEmbedSkip{val: vec![cases::Embed{val: -1}]}, 0);
    test_case!(repeated_min_and_items_len_valid, RepeatedMinAndItemLen{val: vec!["aaa".to_string(), "bbb".to_string()]}, 0);
    test_case!(repeated_min_and_items_len_invalid_min, RepeatedMinAndItemLen{val: vec![String::default()]}, 1);
    test_case!(repeated_min_and_items_len_invalid_len, RepeatedMinAndItemLen{val: vec!["x".to_string()]}, 1);
    test_case!(repeated_min_and_max_items_len_valid, RepeatedMinAndMaxItemLen{val: vec!["aaa".to_string(), "bbb".to_string()]}, 0);
    test_case!(repeated_min_and_max_items_len_invalid_min_len, RepeatedMinAndMaxItemLen::default(), 1);
    test_case!(repeated_min_and_max_items_len_invalid_max_len, RepeatedMinAndMaxItemLen{val: vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string(), "ddd".to_string()]}, 1);

    test_case!(repeated_duration_gte_valid, RepeatedDuration{val: vec![Duration{seconds: 3, nanos: 0}]}, 0);
    test_case!(repeated_duration_gte_valid_empty, RepeatedDuration::default(), 0);
    test_case!(repeated_duration_gte_valid_equal, RepeatedDuration{val: vec![Duration{nanos: 1000000, seconds: 0}]}, 0);
    test_case!(repeated_duration_gte_invalid, RepeatedDuration{val: vec![Duration{seconds: -1, nanos: 0}]}, 1);

    test_case!(repeated_exact_valid_ignore_empty, RepeatedExactIgnore{val: vec![]}, 0);
}
#[cfg(test)]
mod map {
    use super::*;

    test_case!(map_none_valid, MapNone{val: HashMap::from([(123, true), (456, false)])}, 0);

    test_case!(map_min_pairs_valid, MapMin{val: HashMap::from([(1, 2.), (3, 4.), (5, 6.)])}, 0);
    test_case!(map_min_pairs_valid_equal, MapMin{val: HashMap::from([(1, 2.), (3, 4.)])}, 0);
    test_case!(map_min_pairs_invalid, MapMin{val: HashMap::from([(1, 2.)])}, 1);

    test_case!(map_max_pairs_valid, MapMax{val: HashMap::from([(1, 2.), (3, 4.)])}, 0);
    test_case!(map_max_pairs_valid_equal, MapMax{val: HashMap::from([(1, 2.), (3, 4.), (5, 6.)])}, 0);
    test_case!(map_max_pairs_invalid, MapMax{val: HashMap::from([(1, 2.), (3, 4.), (5, 6.), (7, 8.)])}, 1);

    test_case!(map_min_max_valid, MapMinMax{val: HashMap::from([("a".to_string(), true), ("b".to_string(), false), ("c".to_string(), true)])}, 0);
    test_case!(map_min_max_valid_min, MapMinMax{val: HashMap::from([("a".to_string(), true), ("b".to_string(), false)])}, 0);
    test_case!(map_min_max_valid_max, MapMinMax{val: HashMap::from([("a".to_string(), true), ("b".to_string(), false), ("c".to_string(), true), ("d".to_string(), false)])}, 0);
    test_case!(map_min_max_invalid_below, MapMinMax{val: HashMap::from([])}, 1);
    test_case!(map_min_max_invalid_above, MapMinMax{val: HashMap::from([("a".to_string(), true), ("b".to_string(), false), ("c".to_string(), true), ("d".to_string(), false), ("e".to_string(), true)])}, 1);

    test_case!(map_exact_valid, MapExact{val: HashMap::from([(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())])}, 0);
    test_case!(map_exact_invalid_below, MapExact{val: HashMap::from([(1, "a".to_string()), (2, "b".to_string())])}, 1);
    test_case!(map_exact_invalid_above, MapExact{val: HashMap::from([(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string()), (4, "d".to_string())])}, 1);

    test_case!(map_no_sparse_valid, MapNoSparse{val: HashMap::from([(1, cases::map_no_sparse::Msg::default()), (2, cases::map_no_sparse::Msg::default())])}, 0);
    test_case!(map_no_sparse_valid_empty, MapNoSparse{val: HashMap::from([])}, 0);
    // sparse maps are no longer supported, so this case is no longer possible
    // "map_no_sparse_invalid", MapNoSparse{val: HashMap::from([(1, cases::map_no_sparse::Msg::default()), (2, None)])}, 1),

    test_case!(map_keys_valid, MapKeys{val: HashMap::from([(-1, "a".to_string()), (-2, "b".to_string())])}, 0);
    test_case!(map_keys_valid_empty, MapKeys{val: HashMap::default()}, 0);
    test_case!(map_keys_valid_pattern, MapKeysPattern{val: HashMap::from([("A".to_string(), "a".to_string())])}, 0);
    test_case!(map_keys_valid_in, MapKeysIn{val: HashMap::from([("foo".to_string(), "value".to_string())])}, 0);
    test_case!(map_keys_valid_not_in, MapKeysNotIn{val: HashMap::from([("baz".to_string(), "value".to_string())])}, 0);
    test_case!(map_keys_invalid, MapKeys{val: HashMap::from([(1, "a".to_string())])}, 1);
    test_case!(map_keys_invalid_pattern, MapKeysPattern{val: HashMap::from([("A".to_string(), "a".to_string()), ("!@#$%^&*()".to_string(), "b".to_string())])}, 1);
    test_case!(map_keys_invalid_in, MapKeysIn{val: HashMap::from([("baz".to_string(), "value".to_string())])}, 1);
    test_case!(map_keys_invalid_not_in, MapKeysNotIn{val: HashMap::from([("foo".to_string(), "value".to_string())])}, 1);

    test_case!(map_values_valid, MapValues{val: HashMap::from([("a".to_string(), "Alpha".to_string()), ("b".to_string(), "Beta".to_string())])}, 0);
    test_case!(map_values_valid_empty, MapValues{val: HashMap::default()}, 0);
    test_case!(map_values_valid_pattern, MapValuesPattern{val: HashMap::from([("a".to_string(), "A".to_string())])}, 0);
    test_case!(map_values_invalid, MapValues{val: HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())])}, 2);
    test_case!(map_values_invalid_pattern, MapValuesPattern{val: HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "!@#$%^&*()".to_string())])}, 1);

    test_case!(map_recursive_valid, MapRecursive{val: HashMap::from([(1, cases::map_recursive::Msg{val: "abc".to_string()})])}, 0);
    test_case!(map_recursive_invalid, MapRecursive{val: HashMap::from([(1, cases::map_recursive::Msg::default())])}, 1);
    test_case!(map_exact_valid_ignore_empty, MapExactIgnore::default(), 0);
    test_case!(map_multiple_valid, MultipleMaps{first: HashMap::from([(1, "a".to_string()), (2, "b".to_string())]), second: HashMap::from([(-1, true), (-2, false)]), third: HashMap::default()}, 0);
}
#[cfg(test)]
mod oneof {
    use super::*;

    test_case!(oneof_none_valid, OneOfNone{o: Some(one_of_none::O::X("foo".to_string()))}, 0);
    test_case!(oneof_none_valid_empty, OneOfNone::default(), 0);

    test_case!(oneof_field_valid_x, OneOf{o: Some(one_of::O::X("foobar".to_string()))}, 0);
    test_case!(oneof_field_valid_y, OneOf{o: Some(one_of::O::Y(123))}, 0);
    test_case!(oneof_field_valid_z, OneOf{o: Some(one_of::O::Z(TestOneOfMsg{val: true}))}, 0);
    test_case!(oneof_field_valid_empty, OneOf::default(), 0);
    test_case!(oneof_field_invalid_x, OneOf{o: Some(one_of::O::X("fizzbuzz".to_string()))}, 1);
    test_case!(oneof_field_invalid_y, OneOf{o: Some(one_of::O::Y(-1))}, 1);
    test_case!(oneof_filed_invalid_z, OneOf{o: Some(one_of::O::Z(TestOneOfMsg::default()))}, 1);

    test_case!(oneof_required_valid, OneOfRequired{o: Some(one_of_required::O::X("".to_string()))}, 0);
    test_case!(oneof_require_invalid, OneOfRequired::default(), 1);

    test_case!(oneof_ignore_empty_valid_x, OneOfIgnoreEmpty{o: Some(one_of_ignore_empty::O::X("".to_string()))}, 0);
    test_case!(oneof_ignore_empty_valid_y, OneOfIgnoreEmpty{o: Some(one_of_ignore_empty::O::Y(b"".to_vec()))}, 0);
    test_case!(oneof_ignore_empty_valid_z, OneOfIgnoreEmpty{o: Some(one_of_ignore_empty::O::Z(0))}, 0);
}
#[cfg(test)]
mod wrapper {
    use super::*;

    test_case!(wrapper_none_valid, WrapperNone{val: Some(123)}, 0);
    test_case!(wrapper_none_valid_empty, WrapperNone{val: None}, 0);

    test_case!(wrapper_float_valid, WrapperFloat{val: Some(1.)}, 0);
    test_case!(wrapper_float_valid_empty, WrapperFloat{val: None}, 0);
    test_case!(wrapper_float_invalid, WrapperFloat{val: Some(0.)}, 1);

    test_case!(wrapper_double_valid, WrapperDouble{val: Some(1.)}, 0);
    test_case!(wrapper_double_valid_empty, WrapperDouble{val: None}, 0);
    test_case!(wrapper_double_invalid, WrapperDouble{val: Some(0.)}, 1);

    test_case!(wrapper_int64_valid, WrapperInt64{val: Some(1)}, 0);
    test_case!(wrapper_int64_valid_empty, WrapperInt64{val: None}, 0);
    test_case!(wrapper_int64_invalid, WrapperInt64{val: Some(0)}, 1);

    test_case!(wrapper_int32_valid, WrapperInt32{val: Some(1)}, 0);
    test_case!(wrapper_int32_valid_empty, WrapperInt32{val: None}, 0);
    test_case!(wrapper_int32_invalid, WrapperInt32{val: Some(0)}, 1);

    test_case!(wrapper_uint64_valid, WrapperUInt64{val: Some(1)}, 0);
    test_case!(wrapper_uint64_valid_empty, WrapperUInt64{val: None}, 0);
    test_case!(wrapper_uint64_invalid, WrapperUInt64{val: Some(0)}, 1);

    test_case!(wrapper_uint32_valid, WrapperUInt32{val: Some(1)}, 0);
    test_case!(wrapper_uint32_valid_empty, WrapperUInt32{val: None}, 0);
    test_case!(wrapper_uint32_invalid, WrapperUInt32{val: Some(0)}, 1);

    test_case!(wrapper_bool_valid, WrapperBool{val: Some(true)}, 0);
    test_case!(wrapper_bool_valid_empty, WrapperBool{val: None}, 0);
    test_case!(wrapper_bool_invalid, WrapperBool{val: Some(false)}, 1);

    test_case!(wrapper_string_valid, WrapperString{val: Some("foobar".to_string())}, 0);
    test_case!(wrapper_string_valid_empty, WrapperString{val: None}, 0);
    test_case!(wrapper_string_invalid, WrapperString{val: Some("fizzbuzz".to_string())}, 1);

    test_case!(wrapper_bytes_valid, WrapperBytes{val: Some(b"foo".to_vec())}, 0);
    test_case!(wrapper_bytes_valid_empty, WrapperBytes{val: None}, 0);
    test_case!(wrapper_bytes_invalid, WrapperBytes{val: Some(b"x".to_vec())}, 1);

    test_case!(wrapper_required_string_valid, WrapperRequiredString{val: Some("bar".to_string())}, 0);
    test_case!(wrapper_required_string_invalid, WrapperRequiredString{val: Some("foo".to_string())}, 1);
    test_case!(wrapper_required_string_invalid_empty, WrapperRequiredString::default(), 1);

    test_case!(wrapper_required_string_empty_valid, WrapperRequiredEmptyString{val: Some("".to_string())}, 0);
    test_case!(wrapper_required_string_empty_invalid, WrapperRequiredEmptyString{val: Some("foo".to_string())}, 1);
    test_case!(wrapper_required_string_empty_invalid_empty, WrapperRequiredEmptyString::default(), 1);

    test_case!(wrapper_optional_string_uuid_valid, WrapperOptionalUuidString{val: Some("8b72987b-024a-43b3-b4cf-647a1f925c5d".to_string())}, 0);
    test_case!(wrapper_optional_string_uuid_valid_empty, WrapperOptionalUuidString::default(), 0);
    test_case!(wrapper_optional_string_uuid_invalid, WrapperOptionalUuidString{val: Some("foo".to_string())}, 1);

    test_case!(wrapper_required_float_valid, WrapperRequiredFloat{val: Some(1.)}, 0);
    test_case!(wrapper_required_float_invalid, WrapperRequiredFloat{val: Some(-5.)}, 1);
    test_case!(wrapper_required_float_invalid_empty, WrapperRequiredFloat::default(), 1);
}
#[cfg(test)]
mod duration {
    use super::*;
    test_case!(duration_none_valid, DurationNone{val: Some(Duration{seconds: 123, nanos: 0})}, 0);

    test_case!(duration_required_valid, DurationRequired{val: Some(Duration::default())}, 0);
    test_case!(duration_required_invalid, DurationRequired::default(), 1);

    test_case!(duration_const_valid, DurationConst{val: Some(Duration{seconds: 3, nanos: 0})}, 0);
    test_case!(duration_const_valid_empty, DurationConst::default(), 0);
    test_case!(duration_const_invalid, DurationConst{val: Some(Duration{nanos:3, seconds: 0})}, 1);

    test_case!(duration_in_valid, DurationIn{val: Some(Duration{seconds: 1, nanos: 0})}, 0);
    test_case!(duration_in_valid_empty, DurationIn::default(), 0);
    test_case!(duration_in_invalid, DurationIn{val: Some(Duration::default())}, 1);

    test_case!(duration_not_in_valid, DurationNotIn{val: Some(Duration{nanos:1, seconds: 0})}, 0);
    test_case!(duration_not_in_valid_empty, DurationNotIn::default(), 0);
    test_case!(duration_not_in_invalid, DurationNotIn{val: Some(Duration::default())}, 1);

    test_case!(duration_lt_valid, DurationLt{val: Some(Duration{nanos:-1, seconds: 0})}, 0);
    test_case!(duration_lt_valid_empty, DurationLt::default(), 0);
    test_case!(duration_lt_invalid_equal, DurationLt{val: Some(Duration::default())}, 1);
    test_case!(duration_lt_invalid, DurationLt{val: Some(Duration{seconds: 1, nanos: 0})}, 1);

    test_case!(duration_lte_valid, DurationLte{val: Some(Duration::default())}, 0);
    test_case!(duration_lte_valid_empty, DurationLte::default(), 0);
    test_case!(duration_lte_valid_equal, DurationLte{val: Some(Duration{seconds: 1, nanos: 0})}, 0);
    test_case!(duration_lte_invalid, DurationLte{val: Some(Duration{seconds: 1, nanos: 1})}, 1);

    test_case!(duration_gt_valid, DurationGt{val: Some(Duration{seconds: 1, nanos: 0})}, 0);
    test_case!(duration_gt_valid_empty, DurationGt::default(), 0);
    test_case!(duration_gt_invalid_equal, DurationGt{val: Some(Duration{nanos:1000, seconds: 0})}, 1);
    test_case!(duration_gt_invalid, DurationGt{val: Some(Duration::default())}, 1);

    test_case!(duration_gte_valid, DurationGte{val: Some(Duration{seconds: 3, nanos: 0})}, 0);
    test_case!(duration_gte_valid_empty, DurationGte::default(), 0);
    test_case!(duration_gte_valid_equal, DurationGte{val: Some(Duration{nanos:1000000, seconds: 0})}, 0);
    test_case!(duration_gte_invalid, DurationGte{val: Some(Duration{seconds: -1, nanos: 0})}, 1);

    test_case!(duration_gt_lt_valid, DurationGtlt{val: Some(Duration{nanos:1000, seconds: 0})}, 0);
    test_case!(duration_gt_lt_valid_empty, DurationGtlt::default(), 0);
    test_case!(duration_gt_lt_invalid_above, DurationGtlt{val: Some(Duration{seconds: 1000, nanos: 0})}, 1);
    test_case!(duration_gt_lt_invalid_below, DurationGtlt{val: Some(Duration{nanos:-1000, seconds: 0})}, 1);
    test_case!(duration_gt_lt_invalid_max, DurationGtlt{val: Some(Duration{seconds: 1, nanos: 0})}, 1);
    test_case!(duration_gt_lt_invalid_min, DurationGtlt{val: Some(Duration::default())}, 1);

    test_case!(duration_exclusive_gt_lt_valid_empty, DurationExLtgt::default(), 0);
    test_case!(duration_exclusive_gt_lt_valid_above, DurationExLtgt{val: Some(Duration{seconds: 2, nanos: 0})}, 0);
    test_case!(duration_exclusive_gt_lt_valid_below, DurationExLtgt{val: Some(Duration{nanos:-1, seconds: 0})}, 0);
    test_case!(duration_exclusive_gt_lt_invalid, DurationExLtgt{val: Some(Duration{nanos:1000, seconds: 0})}, 1);
    test_case!(duration_exclusive_gt_lt_invalid_max, DurationExLtgt{val: Some(Duration{seconds: 1, nanos: 0})}, 1);
    test_case!(duration_exclusive_gt_lt_invalid_min, DurationExLtgt{val: Some(Duration::default())}, 1);

    test_case!(duration_gte_lte_valid, DurationGtelte{val: Some(Duration{seconds: 60, nanos: 1})}, 0);
    test_case!(duration_gte_lte_valid_empty, DurationGtelte::default(), 0);
    test_case!(duration_gte_lte_valid_max, DurationGtelte{val: Some(Duration{seconds: 3600, nanos: 0})}, 0);
    test_case!(duration_gte_lte_valid_min, DurationGtelte{val: Some(Duration{seconds: 60, nanos: 0})}, 0);
    test_case!(duration_gte_lte_invalid_above, DurationGtelte{val: Some(Duration{seconds: 3600, nanos: 1})}, 1);
    test_case!(duration_gte_lte_invalid_below, DurationGtelte{val: Some(Duration{seconds: 59, nanos: 0})}, 1);

    test_case!(duration_gte_lte_valid_empty_2, DurationExGtelte::default(), 0);
    test_case!(duration_exclusive_gte_lte_valid_above, DurationExGtelte{val: Some(Duration{seconds: 3601, nanos: 0})}, 0);
    test_case!(duration_exclusive_gte_lte_valid_below, DurationExGtelte{val: Some(Duration::default())}, 0);
    test_case!(duration_exclusive_gte_lte_valid_max, DurationExGtelte{val: Some(Duration{seconds: 3600, nanos: 0})}, 0);
    test_case!(duration_exclusive_gte_lte_valid_min, DurationExGtelte{val: Some(Duration{seconds: 60, nanos: 0})}, 0);
    test_case!(duration_exclusive_gte_lte_invalid, DurationExGtelte{val: Some(Duration{seconds: 61, nanos: 0})}, 1);
    test_case!(duration_fields_with_other_fields_invalid_other_field, DurationFieldWithOtherFields{duration_val: None, int_val: 12}, 1);
}
#[cfg(test)]
mod timestamp {
    use super::*;
    test_case!(timestamp_none_valid, TimestampNone{val: Some(Timestamp{seconds: 123, nanos: 0})}, 0);

    test_case!(timestamp_required_valid, TimestampRequired{val: Some(Timestamp::default())}, 0);
    test_case!(timestamp_required_invalid, TimestampRequired{val: None}, 1);

    test_case!(timestamp_const_valid, TimestampConst{val: Some(Timestamp{seconds: 3, nanos: 0})}, 0);
    test_case!(timestamp_const_valid_empty, TimestampConst::default(), 0);
    test_case!(timestamp_const_invalid, TimestampConst{val: Some(Timestamp{nanos: 3, seconds: 0})}, 1);

    test_case!(timestamp_lt_valid, TimestampLt{val: Some(Timestamp{seconds: -1, nanos: 0})}, 0);
    test_case!(timestamp_lt_valid_empty, TimestampLt::default(), 0);
    test_case!(timestamp_lt_invalid_equal, TimestampLt{val: Some(Timestamp::default())}, 1);
    test_case!(timestamp_lt_invalid, TimestampLt{val: Some(Timestamp{seconds: 1, nanos: 0})}, 1);

    test_case!(timestamp_lte_valid, TimestampLte{val: Some(Timestamp::default())}, 0);
    test_case!(timestamp_lte_valid_empty, TimestampLte::default(), 0);
    test_case!(timestamp_lte_valid_equal, TimestampLte{val: Some(Timestamp{seconds: 1, nanos: 0})}, 0);
    test_case!(timestamp_lte_invalid, TimestampLte{val: Some(Timestamp{seconds: 1, nanos: 1})}, 1);

    test_case!(timestamp_gt_valid, TimestampGt{val: Some(Timestamp{seconds: 1, nanos: 0})}, 0);
    test_case!(timestamp_gt_valid_empty, TimestampGt::default(), 0);
    test_case!(timestamp_gt_invalid_equal, TimestampGt{val: Some(Timestamp{nanos: 1000, seconds: 0})}, 1);
    test_case!(timestamp_gt_invalid, TimestampGt{val: Some(Timestamp::default())}, 1);

    test_case!(timestamp_gte_valid, TimestampGte{val: Some(Timestamp{seconds: 3, nanos: 0})}, 0);
    test_case!(timestamp_gte_valid_empty, TimestampGte::default(), 0);
    test_case!(timestamp_gte_valid_equal, TimestampGte{val: Some(Timestamp{nanos: 1000000, seconds: 0})}, 0);
    test_case!(timestamp_gte_invalid, TimestampGte{val: Some(Timestamp{seconds: -1, nanos: 0})}, 1);

    test_case!(timestamp_gt_lt_valid, TimestampGtlt{val: Some(Timestamp{nanos: 1000, seconds: 0})}, 0);
    test_case!(timestamp_gt_lt_valid_empty, TimestampGtlt::default(), 0);
    test_case!(timestamp_gt_lt_invalid_above, TimestampGtlt{val: Some(Timestamp{seconds: 1000, nanos: 0})}, 1);
    test_case!(timestamp_gt_lt_invalid_below, TimestampGtlt{val: Some(Timestamp{seconds: -1000, nanos: 0})}, 1);
    test_case!(timestamp_gt_lt_invalid_max, TimestampGtlt{val: Some(Timestamp{seconds: 1, nanos: 0})}, 1);
    test_case!(timestamp_gt_lt_invalid_min, TimestampGtlt{val: Some(Timestamp::default())}, 1);

    test_case!(timestamp_exclusive_gt_lt_valid_empty, TimestampExLtgt::default(), 0);
    test_case!(timestamp_exclusive_gt_lt_valid_above, TimestampExLtgt{val: Some(Timestamp{seconds: 2, nanos: 0})}, 0);
    test_case!(timestamp_exclusive_gt_lt_valid_below, TimestampExLtgt{val: Some(Timestamp{seconds: -1, nanos: 0})}, 0);
    test_case!(timestamp_exclusive_gt_lt_invalid, TimestampExLtgt{val: Some(Timestamp{nanos: 1000, seconds: 0})}, 1);
    test_case!(timestamp_exclusive_gt_lt_invalid_max, TimestampExLtgt{val: Some(Timestamp{seconds: 1, nanos: 0})}, 1);
    test_case!(timestamp_exclusive_gt_lt_invalid_min, TimestampExLtgt{val: Some(Timestamp::default())}, 1);

    test_case!(timestamp_gte_lte_valid, TimestampGtelte{val: Some(Timestamp{seconds: 60, nanos: 1})}, 0);
    test_case!(timestamp_gte_lte_valid_empty, TimestampGtelte::default(), 0);
    test_case!(timestamp_gte_lte_valid_max, TimestampGtelte{val: Some(Timestamp{seconds: 3600, nanos: 0})}, 0);
    test_case!(timestamp_gte_lte_valid_min, TimestampGtelte{val: Some(Timestamp{seconds: 60, nanos: 0})}, 0);
    test_case!(timestamp_gte_lte_invalid_above, TimestampGtelte{val: Some(Timestamp{seconds: 3600, nanos: 1})}, 1);
    test_case!(timestamp_gte_lte_invalid_below, TimestampGtelte{val: Some(Timestamp{seconds: 59, nanos: 0})}, 1);

    test_case!(timestamp_gte_lte_valid_empty_2, TimestampExGtelte::default(), 0);
    test_case!(timestamp_exclusive_gte_lte_valid_above, TimestampExGtelte{val: Some(Timestamp{seconds: 3601, nanos: 0})}, 0);
    test_case!(timestamp_exclusive_gte_lte_valid_below, TimestampExGtelte{val: Some(Timestamp::default())}, 0);
    test_case!(timestamp_exclusive_gte_lte_valid_max, TimestampExGtelte{val: Some(Timestamp{seconds: 3600, nanos: 0})}, 0);
    test_case!(timestamp_exclusive_gte_lte_valid_min, TimestampExGtelte{val: Some(Timestamp{seconds: 60, nanos: 0})}, 0);
    test_case!(timestamp_exclusive_gte_lte_invalid, TimestampExGtelte{val: Some(Timestamp{seconds: 61, nanos: 0})}, 1);

    test_case!(timestamp_lt_now_valid, TimestampLtNow{val: Some(Timestamp::default())}, 0);
    test_case!(timestamp_lt_now_valid_empty, TimestampLtNow::default(), 0);
    test_case!(timestamp_lt_now_invalid, TimestampLtNow{val: Some(Timestamp{seconds: now() + 7200, nanos: 0})}, 1);

    test_case!(timestamp_gt_now_valid, TimestampGtNow{val: Some(Timestamp{seconds: now() + 7200, nanos: 0})}, 0);
    test_case!(timestamp_gt_now_valid_empty, TimestampGtNow::default(), 0);
    test_case!(timestamp_gt_now_invalid, TimestampGtNow{val: Some(Timestamp::default())}, 1);

    test_case!(timestamp_within_valid, TimestampWithin{val: Some(Timestamp{seconds: now(), nanos: 0})}, 0);
    test_case!(timestamp_within_valid_empty, TimestampWithin::default(), 0);
    test_case!(timestamp_within_invalid_below, TimestampWithin{val: Some(Timestamp::default())}, 1);
    test_case!(timestamp_within_invalid_above, TimestampWithin{val: Some(Timestamp{seconds: now() + 7200, nanos: 0})}, 1);

    test_case!(timestamp_lt_now_within_valid, TimestampLtNowWithin{val: Some(Timestamp{seconds: now() - 1800, nanos: 0})}, 0);
    test_case!(timestamp_lt_now_within_valid_empty, TimestampLtNowWithin::default(), 0);
    test_case!(timestamp_lt_now_within_invalid_lt, TimestampLtNowWithin{val: Some(Timestamp{seconds: now() + 1800, nanos: 0})}, 1);
    test_case!(timestamp_lt_now_within_invalid_within, TimestampLtNowWithin{val: Some(Timestamp{seconds: now() - 7200, nanos: 0})}, 1);

    test_case!(timestamp_gt_now_within_valid, TimestampGtNowWithin{val: Some(Timestamp{seconds: now() + 1800, nanos: 0})}, 0);
    test_case!(timestamp_gt_now_within_valid_empty, TimestampGtNowWithin::default(), 0);
    test_case!(timestamp_gt_now_within_invalid_gt, TimestampGtNowWithin{val: Some(Timestamp{seconds: now() - 1800, nanos: 0})}, 1);
    test_case!(timestamp_gt_now_within_invalid_within, TimestampGtNowWithin{val: Some(Timestamp{seconds: now() + 7200, nanos: 0})}, 1);
}
#[cfg(test)]
mod any {
    use super::*;

    test_case!(any_none_valid, AnyNone{val: Some(Any::default())}, 0);

    test_case!(any_required_valid, AnyRequired{val: Some(Any::default())}, 0);
    test_case!(any_required_invalid, AnyRequired{val: None}, 1);

    test_case!(any_in_valid, AnyIn{val: Some(Any{type_url: "type.googleapis.com/google.protobuf.Duration".to_string(), value: vec![]})}, 0);
    test_case!(any_in_valid_empty, AnyIn::default(), 0);
    test_case!(any_in_invalid, AnyIn{val: Some(Any{type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(), value: vec![]})}, 1);

    test_case!(any_not_in_valid, AnyNotIn{val: Some(Any{type_url: "type.googleapis.com/google.protobuf.Duration".to_string(), value: vec![]})}, 0);
    test_case!(any_not_in_valid_empty, AnyNotIn::default(), 0);
    test_case!(any_not_in_invalid, AnyNotIn{val: Some(Any{type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(), value: vec![]})}, 1);
}
#[cfg(test)]
mod kitchensink {
    use super::*;

    test_case!(kitchensink_field_valid, KitchenSinkMessage{
        val: Some(ComplexTestMsg{
            r#const: "abcd".to_string(),
            int_const: 5,
            bool_const: false,
            float_val: Some(1.),
            dur_val: Some(Duration{seconds: 3, nanos: 0}),
            ts_val: Some(Timestamp{seconds: 17, nanos: 0}),
            float_const: 7.,
            double_in: 123.,
            enum_const: ComplexTestEnum::ComplexTwo.into(),
            any_val: Some(Any{type_url: "type.googleapis.com/google.protobuf.Duration".to_string(), value: vec![]}),
            rep_ts_val: vec![Timestamp{seconds: 3, nanos: 0}],
            map_val: HashMap::from([(-1, "a".to_string()), (-2, "b".to_string())]),
            bytes_val: b"\x00\x99".to_vec(),
            o: Some(complex_test_msg::O::X("foobar".to_string())),
            ..ComplexTestMsg::default()
        })}, 0);
    test_case!(kitchensink_valid_unset, KitchenSinkMessage::default(), 0);
    test_case!(kitchensink_field_invalid, KitchenSinkMessage{val: Some(ComplexTestMsg::default())}, 7);
    test_case!(kitchensink_field_embedded_invalid, KitchenSinkMessage{val: Some(ComplexTestMsg{another: Some(Box::new(ComplexTestMsg::default())), ..ComplexTestMsg::default()})}, 14);
    test_case!(kitchensink_field_invalid_transitive, KitchenSinkMessage{val: Some(ComplexTestMsg{r#const: "abcd".to_string(), bool_const: true, nested: Some(Box::new(ComplexTestMsg::default())), ..ComplexTestMsg::default()})}, 14);
    test_case!(kitchensink_many_all_non_message_fields_invalid, KitchenSinkMessage{val:
        Some(ComplexTestMsg{
            bool_const: true,
            float_val: Some(0.),
            ts_val: Some(Timestamp::default()),
            float_const: 8.,
            any_val: Some(Any{type_url: "asdf".to_string(), value: vec![]}),
            rep_ts_val: vec![Timestamp{seconds: 0, nanos: 1}],
            ..ComplexTestMsg::default()
        })}, 13);
}
#[cfg(test)]
mod nested {
    use super::*;

    test_case!(nested_wkt_uuid_field_valid, WktLevelOne{two: Some(cases::wkt_level_one::WktLevelTwo{three: Some(cases::wkt_level_one::wkt_level_two::WktLevelThree{uuid: "f81d16ef-40e2-40c6-bebc-89aaf5292f9a".to_string()})})}, 0);
    test_case!(nested_wkt_uuid_field_invalid, WktLevelOne{two: Some(cases::wkt_level_one::WktLevelTwo{three: Some(cases::wkt_level_one::wkt_level_two::WktLevelThree{uuid: "not-a-valid-uuid".to_string()})})}, 1);
}

#[allow(unused)]
fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}
