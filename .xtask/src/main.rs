use std::env;
use anyhow::{Context, Result};
use std::process::Command;

fn print_help() {
    eprintln!(
        "Tasks:
    publish    publish prost-validate crates to crates.io
"
    )
}

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("publish") => publish()?,
        _ => print_help(),
    }
    Ok(())
}

fn publish() -> Result<()> {
    let pkgs = [
        "prost-validate-types",
        "prost-validate-derive-core",
        "prost-validate-derive",
        "prost-validate",
        "prost-validate-build",
        "prost-reflect-validate",
    ];

    for pkg in pkgs {
        println!("publishing: {pkg}");
        cargo(&["publish", "-p", pkg])?;
    }

    println!("all packages published");

    Ok(())
}

fn cargo(cmd: &[&str]) -> Result<()> {
    let mut out = Command::new("cargo").args(cmd).spawn().context("spawn")?;

    let exit = out.wait().context("wait")?;

    if !exit.success() {
        anyhow::bail!("non 0 exit code: {}", exit);
    }

    Ok(())
}
