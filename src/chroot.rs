use std::process::{Command, Stdio};

pub fn to_mnt() {
    let arch_chroot = Command::new("arch-chroot")
        .arg("/mnt")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let lsblk = Command::new("lsblk")
        .stdout(Stdio::inherit())
        .output()
        .unwrap();

    return;
}