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
        list("Add a swap partition", vec!["Yes", "No"]),
        list("File System", vec!["ext4"]),
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
    // disks::partition(
    //     if output.selection_value("Separate partition for /home") == "Yes" { true } else { false },
    //     output.selection_value("Select disk"),
    //     output.selection_value("File System"),
    // );

    // // Install packages
    // packages::pacstrap(vec![
    //     "base"
    // ]);

    // disks::genfstab();

    user::hostname(output.selection_value("Hostname"));

    chroot::tasks(
        output.selection_value("User"),
        output.selection_value("User password"),
        output.selection_value("Root password"),
        if output.selection_value("Setup as removable disk") == "Yes" { true } else { false }
    );

    return;
}
