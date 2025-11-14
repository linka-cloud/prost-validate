use anyhow::Result;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

#[allow(clippy::unwrap_used)]
fn main() -> Result<()> {
    let dirs = [
        "proto/harness",
        "proto/cases",
        "proto/cases/other_package",
        "proto/cases/sort",
        "proto/cases/subdirectory",
        "proto/cases/yet_another_package",
    ];
    let includes = [
        "proto/harness",
        "proto/cases",
        "proto/cases/other_package",
        "proto/cases/sort",
        "proto/cases/subdirectory",
        "proto/cases/yet_another_package",
        "../prost-validate-types/proto",
    ];

    for dir in dirs.iter() {
        let files = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(|e| e.path().to_str().unwrap().to_string())
            .filter(|e| e.ends_with(".proto"))
            .collect::<Vec<String>>();
        gen(
            dir.strip_prefix("proto/")
                .unwrap()
                .replace('/', "_")
                .as_str(),
            &files,
            &includes,
        )?;
    }

    Ok(())
}

fn gen(name: &str, files: &[String], includes: &[&str]) -> Result<()> {
    if files.is_empty() {
        return Ok(());
    }
    files.iter().for_each(|f| {
        println!("cargo:rerun-if-changed={}", f);
    });

    // pbjson_types build
    let base_path = PathBuf::from(env::var("OUT_DIR")?);

    let pbjson_path = base_path.join("pbjson");

    std::fs::create_dir_all(&pbjson_path)?;

    let descriptor_path = base_path.join("file_descriptor_set.bin");

    let mut pbjson_config = {
        let mut c = prost_build::Config::new();
        c.file_descriptor_set_path(&descriptor_path)
            .compile_well_known_types()
            .extern_path(".google.protobuf", "::pbjson_types")
            .out_dir(&pbjson_path);
        c
    };
    prost_validate_build::Builder::new().configure(&mut pbjson_config, files, includes)?;
    prost_reflect_build::Builder::new()
        .file_descriptor_set_bytes(format!(
            "crate::_{}_FILE_DESCRIPTOR_SET_BYTES",
            name.to_uppercase()
        ))
        .file_descriptor_set_path(
            env::var_os("OUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
                .join(format!("{}_file_descriptor_set.bin", name)),
        )
        .compile_protos_with_config(pbjson_config, files, includes)?;

    // prost_types build
    let mut config = prost_build::Config::new();
    prost_validate_build::Builder::new().configure(&mut config, files, includes)?;
    prost_reflect_build::Builder::new()
        .file_descriptor_set_bytes(format!(
            "crate::_{}_FILE_DESCRIPTOR_SET_BYTES",
            name.to_uppercase()
        ))
        .file_descriptor_set_path(
            env::var_os("OUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
                .join(format!("{}_file_descriptor_set.bin", name)),
        )
        .compile_protos_with_config(config, files, includes)?;
    Ok(())
}
