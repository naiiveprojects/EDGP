use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=res/resources.rc");

    let out: String = env::var("OUT_DIR").expect("E:env not set");
    let sts: std::process::ExitStatus = Command::new("windres")
        .args(&["res/resources.rc", "-O", "coff", "-o"])
        .arg(format!("{}/resources.res", out))
        .status()
        .expect("E:run windres");
    
    if !sts.success() {
        panic!("windres failed");
    }
    
    println!("cargo:rustc-link-arg-bins={}/resources.res", out);
}