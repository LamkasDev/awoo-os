use std::env;

fn main() {
    let bios_path = env::var("BIOS_PATH").unwrap();
    let mut command = std::process::Command::new("qemu-system-x86_64");
    command
        .arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));
    let mut process = command.spawn().unwrap();
    process.wait().unwrap();
}
