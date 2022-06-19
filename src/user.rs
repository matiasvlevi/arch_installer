pub fn hostname(name: &str) {
    std::fs::write("/mnt/etc/hostname", name).expect("failed to write hostname");
}

