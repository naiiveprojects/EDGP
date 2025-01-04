use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=res/resources.rc");

    let out_dir = env::var("OUT_DIR").unwrap();
    let status = Command::new("windres")
        .args(&["res/resources.rc", "-O", "coff", "-o"])
        .arg(&format!("{}/resources.res", out_dir))
        .status()
        .expect("Failed to run windres");

    if !status.success() {
        panic!("windres failed");
    }

    println!("cargo:rustc-link-arg-bins={}/resources.res", out_dir);

    let target = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let binary_path = format!("{}/release/GDXD.exe", target);
    let _ = Command::new("strip")
        .arg(&binary_path)
        .status()
        .expect("Failed to strip binary");
}