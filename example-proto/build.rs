fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["message.proto"];
    prost_reflect_build::Builder::new()
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos(files, &["proto", "../prost-reflect-validate/proto"])?;
    Ok(())
}
