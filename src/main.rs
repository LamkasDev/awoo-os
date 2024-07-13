use std::env;

fn main() {
    let uefi_path = env::var("UEFI_PATH").unwrap();
    println!("Path: {}", uefi_path);
    let mut command = std::process::Command::new("qemu-system-x86_64");
    command
        .arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-drive")
        .arg(format!("format=raw,file={uefi_path}"))
        .arg("-m")
        .arg("4G");
    let mut process = command.spawn().unwrap();
    process.wait().unwrap();
}
