static DIR: &str = "proto/validate";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["validate.proto"];
    files.iter().for_each(|f| {
        println!("cargo:rerun-if-changed={}/{}", DIR, f);
    });
    prost_reflect_build::Builder::new()
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos(files, &[DIR])?;
    Ok(())
}
