# Arch Installer

Personal Arch Linux installer written in Rust

### Build

```
cargo build
```

### Running

```
./target/debug/arch_installer
```

### Deploy to archiso

set root password with `passwd` and then:
```
scp ./target/debug/arch_installer root@ARCHISO_IP:~/arch_install
```
