pub use std::process::{Command, Stdio};
pub use std::io::{Read, Write};

pub fn space_as_string(size: u32, unit: &str ) -> String {
    let mut ans = String::new();
    ans.push_str(&size.to_string());
    ans.push_str(unit);
    return ans;
}

fn trim_whitespace(input: &str) -> String {
    let mut ans:String = String::new();
    let mut char_before:bool = true;
    for c in input.chars() {
        if c != ' ' {
            // If non-whitespace, push to string
            ans.push(c);
            char_before = true;
        } else if char_before {
            // If first whitespace, push to string
            ans.push(c);
            char_before = false;
        }
    }
    return ans;
}

fn lsblk_disks() -> Vec<String> {
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

    let mut ans: Vec<String> = Vec::new();
    for line in lsblk_lines {
        ans.push(line.to_string());
    }

    return ans;
}

pub fn get_size(disk: &str) -> String {
    let lsblk_output = lsblk_disks();
    for line in lsblk_output {
        let fline = trim_whitespace(&line);
        let disk_entry: Vec<&str> = fline.split(" ").collect();
        let mut device = String::from("/dev/");
        device.push_str(disk_entry[0]);
        if device == disk {
            return disk_entry[3].to_string();
        }
    }
    return "0B".to_string();
}


pub fn scan() -> Vec<String> {

    let mut disks: Vec<String> = Vec::new();

    let lsblk_output = lsblk_disks();

    for line in lsblk_output {
        let disk_entry: Vec<&str> = line.split(" ").collect();
        let mut device = String::from("/dev/");
        device.push_str(disk_entry[0]);
        disks.push(device);
    }

    return disks;
}

pub fn partition(separate_home:bool, disk: &str, fstype: &str) {
    let boot_partition_size:u32 = 300;
    let boot_partition_size_s: String = space_as_string(boot_partition_size, "MB");

    let swap_partition_size:u32 = 4096;
    let swap_partition_size_begin_s: String = space_as_string(boot_partition_size + 1, "MB");
    let swap_partition_size_end_s: String = space_as_string(boot_partition_size + swap_partition_size + 1, "MB");
    

    let root_partition_size:u32 = 4096 * 2; // TEMPORARY
    let root_partition_size_begin_s: String = space_as_string(boot_partition_size + swap_partition_size + 2, "MB");
    let root_partition_size_end_s: String = if separate_home {
        space_as_string(boot_partition_size + swap_partition_size + root_partition_size + 2, "MB")
    } else { String::from("100%") };

    let home_partition_size_begin_s: String = space_as_string(boot_partition_size + swap_partition_size + root_partition_size + 3, "MB");
    let home_partition_size_end_s: String = String::from("100%");

    // Wipe filesystem
    let mut wipefs_cmd = Command::new("wipefs")
        .arg("-a")
        .arg(disk)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_wipefs_cmd = wipefs_cmd.wait().unwrap();

    // Set GPT label
    let mut gpt_label_cmd = Command::new("parted")
        .arg(disk)
        .arg("mklabel")
        .arg("gpt")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    
    let _result_gpt_label_cmd = gpt_label_cmd.wait().unwrap();

    // Create boot partition
    let mut boot_partition_cmd = Command::new("parted")
        .arg("-s")
        .arg("-a")
        .arg("optimal")
        .arg(disk)
        .arg("mkpart")
        .arg("primary")
        .arg("0%")
        .arg(boot_partition_size_s)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    
    let _result_boot_partition_cmd = boot_partition_cmd.wait().unwrap();

    // Make boot partition file system
    let mut boot_partition = disk.to_string().clone();
    boot_partition.push_str("1"); 
    let mut boot_fs_cmd = Command::new("mkfs.fat")
        .arg("-F32")
        .arg(&boot_partition)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_boot_fs_cmd = boot_fs_cmd.wait().unwrap();

    // Create swap partition
    let mut swap_partition_cmd = Command::new("parted")
        .arg("-s")
        .arg("-a")
        .arg("optimal")
        .arg(disk)
        .arg("mkpart")
        .arg("primary")
        .arg(swap_partition_size_begin_s)
        .arg(swap_partition_size_end_s)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_swap_partition_cmd = swap_partition_cmd.wait().unwrap();

    // Make swap file system
    let mut swap_partition = disk.to_string().clone();
    swap_partition.push_str("2"); 
    let mut swap_fs_cmd = Command::new("mkswap")
        .arg(&swap_partition)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_swap_fs_cmd = swap_fs_cmd.wait().unwrap();

    // Mount swap
    let mut swap_mount_cmd = Command::new("swapon")
        .arg(&swap_partition)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_swap_mount_cmd = swap_mount_cmd.wait().unwrap();

    // Create root partition
    let mut root_partition_cmd = Command::new("parted")
        .arg("-s")
        .arg("-a")
        .arg("optimal")
        .arg(disk)
        .arg("mkpart")
        .arg("primary")
        .arg(root_partition_size_begin_s)
        .arg(root_partition_size_end_s)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_root_partition_cmd = root_partition_cmd.wait().unwrap();

    // Create root file system
    let mut root_partition = disk.to_string().clone();
    root_partition.push_str("3"); 
    let mut root_fs_cmd = Command::new("mkfs.ext4")
        .arg(&root_partition)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_root_fs_cmd = root_fs_cmd.wait().unwrap();

    // Create root & boot mount directories
    let mut mkdir_root = Command::new("mkdir")
        .arg("-p")
        .arg("/mnt")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mkdir_root = mkdir_root.wait().unwrap();

    // Mount root partition
    let mut mount_root = Command::new("mount")
        .arg(&root_partition)
        .arg("/mnt")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mount_root = mount_root.wait().unwrap();

    let mut mkdir_efi = Command::new("mkdir")
        .arg("-p")
        .arg("/mnt/boot/efi")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mkdir_efi = mkdir_efi.wait().unwrap();

    // Mount boot partition
    let mut mount_boot = Command::new("mount")
        .arg(&boot_partition)
        .arg("/mnt/boot/efi")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mount_boot = mount_boot.wait().unwrap();

    if !separate_home { return }; // EXIT IF NO HOME PARTITION ---

    // Create home partition
    let mut home_partition_cmd = Command::new("parted")
        .arg("-s")
        .arg("-a")
        .arg("optimal")
        .arg(disk)
        .arg("mkpart")
        .arg("primary")
        .arg(home_partition_size_begin_s)
        .arg(home_partition_size_end_s)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_home_partition_cmd = home_partition_cmd.wait().unwrap();

    // Create File System on home directory
    let mut home_partition = disk.to_string().clone();
    home_partition.push_str("4"); 
    let mut home_fs_cmd = Command::new("mkfs.ext4")
        .arg(&home_partition)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_home_fs_cmd = home_fs_cmd.wait().unwrap();

    // Create home mount directory
    let mut mkdir_home = Command::new("mkdir")
        .arg("-p")
        .arg("/mnt/home")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mkdir_home = mkdir_home.wait().unwrap();

    // Mount home directory
    let mut mount_home = Command::new("mount")
        .arg(&home_partition)
        .arg("/mnt/home")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let _result_mount_home = mount_home.wait().unwrap();

    return;
}

pub fn genfstab() {
    // Mount home directory
    let mut genfstab_cmd = Command::new("genfstab")
        .arg("-U")    
        .arg("-p")
        .arg("/mnt")  
        .stdout(Stdio::piped())
        .output()
        .expect("failed to generate fstab file");

    let output = String::from_utf8(genfstab_cmd.stdout).unwrap();
    std::fs::write("/mnt/etc/fstab", &output).expect("Failed to write file");
} 