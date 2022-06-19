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

    let mut root_pw_cmd = String::from("\"echo -e \"");
    root_pw_cmd.push_str(root_password);
    root_pw_cmd.push('\n');
    root_pw_cmd.push_str(root_password);
    root_pw_cmd.push_str("\" | passwd \"");

    let mut create_user_cmd = String::from("\"useradd -m -G wheel -s /bin/bash ");
    create_user_cmd.push_str(user_name);
    create_user_cmd.push('\"');

    let mut user_pw_cmd = String::from("\"echo -e \"");
    user_pw_cmd.push_str(user_password);
    user_pw_cmd.push('\n');
    user_pw_cmd.push_str(user_password);
    user_pw_cmd.push_str("\" | passwd ");
    user_pw_cmd.push_str(user_name);
    user_pw_cmd.push('\"');

    let grub_install_cmd: &str = &grub_install(is_removable);
    
    let mut arch_chroot = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("/bin/bash")
        .arg("-c")
        // Login setup
        .args(vec![
            root_pw_cmd,
            create_user_cmd,
            user_pw_cmd
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