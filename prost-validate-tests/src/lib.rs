use prost_reflect::ReflectMessage;
use prost_reflect_validate::ValidatorExt;
use prost_validate::Validator as ValidatorDerive;

pub mod cases;
pub mod cases_pbjson;
#[allow(clippy::disallowed_names)]
mod proto;
mod proto_pbjson;
mod test_cases;
mod test_pbjson_cases;

static _HARNESS_FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/harness_file_descriptor_set.bin"));
static _CASES_FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/cases_file_descriptor_set.bin"));
static _CASES_OTHER_PACKAGE_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/cases_other_package_file_descriptor_set.bin"
));
static _CASES_SORT_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/cases_sort_file_descriptor_set.bin"
));
static _CASES_SUBDIRECTORY_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/cases_subdirectory_file_descriptor_set.bin"
));
static _CASES_YET_ANOTHER_PACKAGE_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/cases_yet_another_package_file_descriptor_set.bin"
));

pub trait Validator: ReflectMessage + ValidatorExt + ValidatorDerive {}

impl<T: ReflectMessage + ValidatorExt + ValidatorDerive> Validator for T {}

pub type Factory = Box<dyn Fn() -> (Box<dyn Validator>, i32) + Send + Sync>;

pub(crate) fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}
