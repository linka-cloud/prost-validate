use anyhow::Result;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

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
                .replace("/", "_")
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
