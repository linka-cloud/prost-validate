use std::env;
use std::path::PathBuf;

static DIR: &str = "proto/validate";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["validate.proto"];
    files.iter().for_each(|f| {
        println!("cargo:rerun-if-changed={}/{}", DIR, f);
    });

    #[allow(clippy::unwrap_used)]
    let base_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = base_path.join("file_descriptor_set.bin");
    prost_reflect_build::Builder::new()
        .file_descriptor_set_path(&descriptor_path)
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos(files, &[DIR])?;
    Ok(())
}
