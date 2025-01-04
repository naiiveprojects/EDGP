use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=res/resources.rc");

    let out_dir = env::var("OUT_DIR").expect("E:env not set");
    let status = Command::new("windres")
        .args(&["res/resources.rc", "-O", "coff", "-o"])
        .arg(format!("{}/resources.res", out_dir))
        .status()
        .expect("E:run windres");
    
    if !status.success() {
        panic!("windres failed");
    }
    
    println!("cargo:rustc-link-arg-bins={}/resources.res", out_dir);
}