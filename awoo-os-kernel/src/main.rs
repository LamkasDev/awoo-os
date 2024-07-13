#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
#![test_runner(awoo_os_kernel::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use awoo_os_kernel::{hlt_loop, BOOTLOADER_CONFIG};
use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    awoo_os_kernel::init(boot_info);

    #[cfg(test)]
    test_main();

    hlt_loop();
}
