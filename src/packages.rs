pub use std::process::{Command, Stdio};

pub fn pacstrap(packages: Vec<&str>) {
    let pacstrap_cmd = Command::new("pacstrap")
        .arg("/mnt")
        .args(packages)
        .arg("--noconfirm")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    println!("status: {}", pacstrap_cmd.status);

    return;
}