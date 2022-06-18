pub use std::process::{Command, Stdio};

pub fn pacstrap(packages: Vec<&str>) {
    let mut pacstrap_cmd = Command::new("pacstrap")
        .arg("/mnt")
        .args(packages)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    
    let _result_pacstrap_cmd = pacstrap_cmd.wait().unwrap();

    return;
}