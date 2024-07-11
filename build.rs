use bootloader_api::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    // set by cargo for the kernel artifact dependency
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));

    // specify output paths
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bios_path = out_dir.join("awoo-os-bios.img");

    // create the disk images
    disk_builder.create_bios_image(&bios_path).unwrap();
    
    // pass the disk image paths via environment variables
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}