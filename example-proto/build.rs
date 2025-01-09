fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["message.proto", "service.proto"];
    let includes = &["proto", "../prost-validate-types/proto"];

    let mut config = {
        let mut c = prost_build::Config::new();
        c.service_generator(tonic_build::configure().service_generator());
        c
    };

    prost_validate_build::Builder::new().configure(&mut config, files, includes)?;

    prost_reflect_build::Builder::new()
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos_with_config(config, files, includes)?;

    Ok(())
}
