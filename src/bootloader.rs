


pub fn install_grub(removable:bool) {

    let mut optional:Vec<&str> = vec!["--recheck"];
    if (removable) optional.push("--removable");

    let grub_install_cmd = Command::new("grub-install")
        .arg("--target=x86_64-efi")
        .arg("--efi-directory=/mnt/boot/efi")
        .arg("--bootloader-id=ARCH")
        .args(optional)
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
    

    let grub_mkconfig_cmd = Command::new("grub-mkconfig")
        .arg("-o")
        .arg("/mnt/boot/grub/grub.cfg")
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
}

