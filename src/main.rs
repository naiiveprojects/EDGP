use std::{env, process::{Command, self}};

fn main() {
    let slf = env::current_exe().unwrap_or_else(|_| process::exit(1));
    let fnm = slf.file_name().unwrap_or_else(|| process::exit(1)).to_string_lossy();
    let bin_name = fnm.split_once('.')
        .map(|(name, _)| name)
        .unwrap_or_else(|| process::exit(1));

    let bin_dir = slf.parent().unwrap_or_else(|| process::exit(1)).join(bin_name);
    if !bin_dir.exists() || !bin_dir.is_dir() {
        process::exit(1);
    }

    let engine_path = bin_dir.join("Bin").join("Engine.exe");
    if !engine_path.exists() {
        process::exit(1);
    }

    let data_path = bin_dir.join("Data").join("Master.pck");
    if !data_path.exists() {
        process::exit(1);
    }

    let sts = Command::new(engine_path)
        .args(&["--main-pack", data_path.to_str().unwrap(), "-v"])
        .status()
        .unwrap_or_else(|_| process::exit(1));
    process::exit(sts.code().unwrap_or(1));
}
