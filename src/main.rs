#![windows_subsystem = "windows"]

use std::process::{Command, exit};
use std::env;
use std::path::PathBuf;

fn main() {
    let exe = env::current_exe().expect("path"); // Executable path
    let bdr = exe.parent().expect("directory").to_path_buf(); // Base directory
    let fnm = exe.file_name().expect("filename").to_string_lossy(); // File name
    let bnm = fnm.split_once('.').map(|(base, _)| base).unwrap_or(&fnm); // Base name
    let fdr = bdr.join(bnm); // Folder directory
    let dbg = fnm.chars().filter(|&c| c == '.').count() > 1; // (. before .exe)
    
    
    if fdr.exists() {
        run(&fdr, dbg);
    } else if dbg {
        alt(&bdr, &bnm, &exe);
    } else {
        exit(1);
    }
}

fn run(fdr: &PathBuf, dbg: bool) {
    let exe = fdr.join("Bin/win.exe");
    let pck = fdr.join("Data/main.pck");

    if exe.exists() && pck.exists() {
        if dbg {
            Command::new("cmd")
                .arg("/C")
                .arg(&exe)
                .arg("--main-pack")
                .arg(&pck)
                .arg("-v")
                .status()
                .expect("run");
        } else {
            Command::new(&exe)
                .arg("--main-pack")
                .arg(&pck)
                .spawn()
                .expect("run");
        }
    }
}

fn alt(bdr: &PathBuf, bnm: &str, exe: &PathBuf) {
    let alt = bdr.join(format!("{}.exe", bnm));

    if alt.exists() && alt != *exe {
        Command::new("cmd")
            .arg("/C")
            .arg(&alt)
            .args(&["-v", "-d"])
            .status()
            .expect("alt");
    }
}
