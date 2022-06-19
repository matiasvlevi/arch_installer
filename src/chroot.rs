use std::process::{Command, Stdio};

pub fn grub_install(is_removable: bool) -> String {

    let mut optional:Vec<&str> = vec!["--recheck"];
    if is_removable { optional.push("--removable") };

    let mut grub_install_cmd = 
        String::from("grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=ARCH");

    for option in optional {
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

    let mut root_pw_cmd = String::from("\"echo ");
    root_pw_cmd.push_str(root_password);
    root_pw_cmd.push_str(" | passwd --stdin \"");

    let mut create_user_command = String::from("\"useradd -m -G wheel -s /bin/bash ");
    create_user_command.push_str(user_name);
    create_user_command.push('\"');

    let mut user_pw_command = String::from("\"echo ");
    user_pw_command.push_str(user_password);
    user_pw_command.push_str(" | passwd --stdin ");
    user_pw_command.push_str(user_name);
    user_pw_command.push('\"');

    let grub_install_cmd: &str = &grub_install(is_removable);
    
    let mut arch_chroot = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("/bin/bash")
        .arg("-c")
        // Login setup
        .args(vec![
            root_pw_cmd,
            create_user_command,
            user_pw_command
        ])
        // Bootloader installation
        .args(vec![
            grub_install_cmd,
            "grub-mkconfig -o /boot/grub/grub.cfg"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    
    return;
}