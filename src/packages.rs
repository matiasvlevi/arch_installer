use std::process::{Command, Stdio};

pub fn default() -> Vec<&'static str> {vec![
    "base", "base-devel", "linux", "linux-firmware", "sof-firmware",
    "grub", "efibootmgr",
    "iwd", "networkmanager", "net-tools", "dhcpcd", "wpa_supplicant",
    "neovim","git","htop","neofetch"
]}

pub fn minimal() -> Vec<&'static str> {vec![
    "base", "linux", "linux-firmware",
    "grub", "efibootmgr",
    "iwd", "networkmanager", "dhcpcd", "wpa_supplicant",
    "vim"
]}

pub fn desktop() -> Vec<&'static str> {vec![
    "base", "base-devel", "linux", "linux-firmware", "sof-firmware",
    "grub", "efibootmgr",
    "iwd", "networkmanager", "net-tools", "dhcpcd", "wpa_supplicant",
    "xorg-server","xorg-xinit","sddm","plasma","konsole",
    "neovim","git","htop","neofetch"
]}


pub fn server() -> Vec<&'static str> {vec![
    "base", "base-devel", "linux", "linux-firmware",
    "grub", "efibootmgr",
    "iwd", "networkmanager", "dhcpcd", "net-tools", "wpa_supplicant", 
    "openssh", "nginx", "vsftpd", "apache", "nodejs-lts-gallium", "php",
    "vim", "git", "htop"
]}


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