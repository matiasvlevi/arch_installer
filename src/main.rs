use std::process::{Command, Stdio};
use std::io::{Read, Write};

fn scan_disks() -> Vec<String> {
    
    let mut disks: Vec<String> = Vec::new();

    let mut cmd_lsblk = Command::new("lsblk").arg("-d")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_grep = Command::new("grep").arg("disk")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();


    if let Some(ref mut stdout) = cmd_lsblk.stdout {
        if let Some(ref mut stdin) = cmd_grep.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }

    let lsblk_filtered_output: String =
     String::from_utf8(cmd_grep.wait_with_output().unwrap().stdout).unwrap();

    let mut lsblk_lines:Vec<&str> = lsblk_filtered_output.split("\n").collect();
    lsblk_lines.remove(lsblk_lines.len()-1);

    for line in lsblk_lines {
        let disk_entry: Vec<&str> = line.split(" ").collect();
        let mut dev = String::from("/dev/");
        dev.push_str(disk_entry[0]);
        disks.push(dev);
    }

    return disks;
}

fn main() {
   let disks = scan_disks();

    use terminal_menu::{run, menu, list, string, back_button, mut_menu};
    let menu = menu(vec![
        list("Select disk", disks),
        string("Hostname", "arch", false),
        string("User", "user", false),
        string("User password", "admin", false),
        string("Root password", "admin", false),
        list("Use root password", vec!["Yes", "No"]),
        list("Separate partition for /home", vec!["Yes", "No"]),
        list("File System", vec!["ext4"]),
        back_button("Start Install"),
    ]);
    
    println!("ARCH INSTALLER");
    println!("Press ESC or q to abort installation\n");

    run(&menu);

    let output = mut_menu(&menu);

    // Abort on cancel
    if output.canceled() {
        return 
    };

    // Print values
    println!("{}", output.selection_value("Select disk"));
    println!("{}", output.selection_value("Hostname"));
    println!("{}", output.selection_value("User"));
    println!("{}", output.selection_value("User password"));
    println!("{}", output.selection_value("Root password"));
    println!("{}", output.selection_value("Use root password"));
    println!("{}", output.selection_value("Separate partition for /home"));
    println!("{}", output.selection_value("File System"));

    // Arch install
}
