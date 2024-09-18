mod proto;
mod cases;

static _HARNESS_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/harness_file_descriptor_set.bin"));
static _CASES_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cases_file_descriptor_set.bin"));
static _CASES_OTHER_PACKAGE_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cases_other_package_file_descriptor_set.bin"));
static _CASES_SORT_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cases_sort_file_descriptor_set.bin"));
static _CASES_SUBDIRECTORY_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cases_subdirectory_file_descriptor_set.bin"));
static _CASES_YET_ANOTHER_PACKAGE_FILE_DESCRIPTOR_SET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cases_yet_another_package_file_descriptor_set.bin"));
