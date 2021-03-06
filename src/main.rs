mod disks;
mod form;
mod packages;
mod chroot;
mod user;

use ctrlc;

fn main() {

    // Disable quitting
    ctrlc::set_handler(move || {
        println!("Can't abort install");
    })
    .expect("failed to set ctrlc handler");

    let disks = disks::scan();

    use terminal_menu::{run, menu, list, string, back_button, mut_menu};
    let menu = menu(vec![
        list("Select disk", disks),
        string("Hostname", "arch", false),
        string("User", "user", false),
        string("User password", "admin", false),
        string("Root password", "admin", false),
        list("Use root password", vec!["Yes", "No"]),
        list("Separate partition for /home", vec!["Yes", "No"]),
        list("Setup as removable disk", vec!["Yes", "No"]),
        list("File System", vec!["ext4", "btrfs"]),
        list("Package preset", vec!["default", "minimal", "desktop", "server"]),
        back_button("Start Install"),
    ]);
    
    println!("ARCH INSTALLER");
    println!("Press ESC or q to abort installation\n");

    run(&menu);

    let output = mut_menu(&menu);

    // Abort on cancel
    if output.canceled() {
        println!("Arch installation aborted...");
        return;
    };

    // Print values
    for q in form::data() {
        println!("{query}: {user_input}", 
            query=q,
            user_input=output.selection_value(q)
        );
    }

    // Arch Install

    // Partitioning
    disks::partition(
        if output.selection_value("Separate partition for /home") == "Yes" { true } else { false },
        output.selection_value("Select disk"),
        output.selection_value("File System"),
    );

    // Install packages
    let key: &str = output.selection_value("Package preset");
    match key {
        "minimal"=>packages::pacstrap(packages::minimal()),
        "default"=>packages::pacstrap(packages::default()),
        "desktop"=>packages::pacstrap(packages::desktop()),
        "server"=>packages::pacstrap(packages::server()),
        &_ => todo!()
    }

    // fstab
    disks::genfstab();

    // // Host name
    user::hostname(output.selection_value("Hostname"));

    // // Execute tasks done as /mnt root (Root & User setup, bootloader installation)
    chroot::tasks(
        output.selection_value("User"),
        output.selection_value("User password"),
        output.selection_value("Root password"),
        if output.selection_value("Setup as removable disk") == "Yes" { true } else { false }
    );

    return;
}
