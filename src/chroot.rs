use std::process::{Command, Stdio};

pub fn grub_install(is_removable: bool) -> String {

    let mut optional:Vec<&str> = vec!["--recheck"];
    if is_removable { optional.push("--removable") };

    let mut grub_install_cmd = 
        String::from("grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=ARCH");

    for option in optional {
        grub_install_cmd.push(' ');
        grub_install_cmd.push_str(option);
    }

    return grub_install_cmd;
}

pub fn tasks(
    user_name: &str,
    user_password: &str,
    root_password: &str,
    is_removable: bool
) {
    let mut chroot_cmd = String::new();

    // Root password
    chroot_cmd.push_str("echo -e '");
    chroot_cmd.push_str(root_password);
    chroot_cmd.push_str("\\n");
    chroot_cmd.push_str(root_password);
    chroot_cmd.push_str("' | passwd");
    chroot_cmd.push_str(" && ");

    // // User creation
    chroot_cmd.push_str("useradd -m -G wheel -s /bin/bash ");
    chroot_cmd.push_str(user_name);
    chroot_cmd.push_str(" && ");

    // // User password
    chroot_cmd.push_str("echo -e '");
    chroot_cmd.push_str(user_password);
    chroot_cmd.push_str("\\n");
    chroot_cmd.push_str(user_password);
    chroot_cmd.push_str("' | passwd ");
    chroot_cmd.push_str(user_name);
    // chroot_cmd.push_str(" && ");

    // Bootloader installation
    let grub_install_cmd: &str = &grub_install(is_removable);
    chroot_cmd.push_str(grub_install_cmd);
    chroot_cmd.push_str(" && ");

    // Bootloader config
    chroot_cmd.push_str("grub-mkconfig -o /boot/grub/grub.cfg");

    std::fs::write("/mnt/install.sh", &chroot_cmd).expect("failed to write install script");

    println!("\n{}\n",chroot_cmd);

    let mut arch_chroot = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("/bin/bash")
        .arg("-c")
        .arg("\"lsblk\"")
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();
    
    return;
}