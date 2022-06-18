mod disks;
mod form;

pub use std::process::{Command, Stdio};

fn main() {

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
        return 
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


    return;
}
