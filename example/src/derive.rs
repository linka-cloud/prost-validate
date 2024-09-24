fn main() {
    use example_proto::ExampleMessage;
    use prost_validate::Validator;

    match ExampleMessage::default().validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
    let msg = ExampleMessage {
        content: "Hello, world!".to_string(),
    };
    match msg.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
