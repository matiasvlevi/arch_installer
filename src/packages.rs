pub use std::process::{Command, Stdio};

pub fn pacstrap(packages: Vec<&str>) {
    let mut pacstrap_cmd = Command::new("pacstrap")
        .arg("/mnt")
        .args(packages)
        .arg("--noconfirm")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    return;
}