pub use std::process::{Command, Stdio};

pub fn pacstrap(packages: Vec<&str>) {
    let pacstrap_cmd = Command::new("pacstrap")
        .arg("/mnt")
        .args(packages)
        .arg("--noconfirm")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed");

    let output = pacstrap_cmd
        .wait_with_output()
        .expect("failed");

    return;
}