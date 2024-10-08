use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;

#[allow(clippy::unwrap_used)]
pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});

include!(concat!(env!("OUT_DIR"), "/validate.rs"));
