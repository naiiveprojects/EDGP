#![windows_subsystem = "windows"]

use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let exe: PathBuf = env::current_exe().unwrap(); // Executable path
    let bdr: PathBuf = exe.parent().unwrap().to_path_buf(); // Base directory
    let fnm: String = exe.file_name().unwrap().to_string_lossy().to_string(); // File name
    let bnm: &str = fnm.split_once('.').map(|(base, _)| base).unwrap_or(&fnm); // Base name
    let fdr: PathBuf = bdr.join(bnm); // Folder directory
    let dbg: bool = fnm.chars().filter(|&c| c == '.').count() > 1; // (. before .exe)
    
    if fdr.exists() {
        run(&fdr, dbg);
    } else if dbg {
        alt(&bdr, bnm, &exe);
    } else {
        std::process::exit(1);
    }
}

fn run(fdr: &Path, dbg: bool) {
    let exe: PathBuf = fdr.join("Bin/win.exe"); // exe path
    let pck: PathBuf = fdr.join("Data/main.pck"); // pck path
    let ext: Vec<String> = env::args().skip(1).collect();

    if exe.exists() && pck.exists() {
        let mut cmd: Command;
        if dbg {
            cmd = Command::new("cmd");
            cmd.arg("/C").arg(&exe).arg("--main-pack").arg(&pck).arg("-v");
        } else {
            cmd = Command::new(&exe);
            cmd.arg("--main-pack").arg(&pck);
        }
        cmd.args(&ext).spawn().unwrap();
    }
}

fn alt(bdr: &Path, bnm: &str, exe: &Path) {
    let alt: PathBuf = bdr.join(format!("{}.exe", bnm));
    let ext: Vec<String> = env::args().skip(1).collect();

    if alt.exists() && alt.canonicalize().unwrap() != exe.canonicalize().unwrap() {
        let mut cmd: Command = Command::new("cmd");
        cmd.arg("/C").arg(&alt).args(&["-v", "-d"]).args(&ext).spawn().unwrap();
    }
}
