mod proto {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| DescriptorPool::decode(include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref()).unwrap());
    include!(concat!(env!("OUT_DIR"), "/validate.example.rs"));
}

fn main() {
    use prost_reflect_validate::ValidatorExt;
    use crate::proto::ExampleMessage;
    
    match ExampleMessage::default().validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
    let msg = ExampleMessage{content: "Hello, world!".to_string()};
    match msg.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
