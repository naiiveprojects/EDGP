use std::{env, process::{Command, exit}};

fn main() {
    let slf = env::current_exe().unwrap_or_else(|_| exit(1));
    let fnm = slf.file_name().unwrap_or_else(|| exit(1)).to_string_lossy();
    let trg = fnm.split_once('.')
        .map(|(name, _)| format!("{}.exe", name))
        .unwrap_or_else(|| exit(1));

    let pth = slf.parent().unwrap_or_else(|| exit(1)).join(&trg);
    if !pth.exists() || pth == slf {
        exit(1);
    }
    let sts = Command::new(pth)
        .args(&vec!["-v", "-d"])
        .status()
        .unwrap_or_else(|_| exit(1));
    exit(sts.code().unwrap_or(1));
}
