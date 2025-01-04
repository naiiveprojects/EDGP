#![windows_subsystem = "windows"]

use std::process::{Command, exit};
use std::env;
use std::path::Path;

fn exists(p: &Path) {
    if !p.exists() { exit(1); }
}

fn main() {
    let slf = env::current_exe().expect("E:path");
    let dir = slf.parent().expect("E:directory");
    let n = slf.file_stem().expect("E:name").to_string_lossy();
    let exe = dir.join(format!("{}/Bin/win.exe", n));
    let pck = dir.join(format!("{}/Data/main.pck", n));
    exists(&exe);
    exists(&pck);
    Command::new(exe)
        .arg("--main-pack")
        .arg(pck)
        .spawn().expect("E:process");
}