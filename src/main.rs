#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(awoo_os::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use awoo_os::{hlt_loop, println};
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    awoo_os::init(boot_info);

    #[cfg(test)]
    test_main();

    hlt_loop();
}
