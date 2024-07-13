use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    // specify output paths
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("awoo-os-uefi.img");

    // set by cargo for the kernel artifact dependency
    let kernel_path = env::var("CARGO_BIN_FILE_AWOO_OS_KERNEL_awoo-os-kernel").unwrap();
    let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));

    // create the disk image
    disk_builder.create_uefi_image(&uefi_path).unwrap();

    // pass the disk image path via environment variable
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
}
